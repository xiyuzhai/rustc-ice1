// Minimal reproduction of rustc ICE in borrow checker
// Related to generic const parameters + complex lifetimes + associated types

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub trait IsScheme: 'static {
    type Node: Eq + Copy + 'static;
    const N: usize;
}

pub trait IsContext<'db>: Copy {
    type Scheme: IsScheme;
    fn deps(self, node: <Self::Scheme as IsScheme>::Node) -> impl IntoIterator<Item = <Self::Scheme as IsScheme>::Node>;
}

pub struct CycleGroup<S: IsScheme>
where
    [(); S::N]:,
{
    nodes: [S::Node; S::N],
    len: usize,
}

impl<S: IsScheme> CycleGroup<S>
where
    [(); S::N]:,
{
    pub fn iter(&self) -> impl Iterator<Item = &S::Node> {
        self.nodes[..self.len].iter()
    }
}

pub struct Map<S: IsScheme, V>
where
    [(); S::N]:,
{
    entries: [(S::Node, V); S::N],
    len: usize,
}

impl<S: IsScheme, V> Map<S, V>
where
    [(); S::N]:,
{
    pub fn new<'db, C>(ctx: C, cycle_group: &'db CycleGroup<S>) -> Self
    where
        C: IsContext<'db, Scheme = S>,
        V: Default,
    {
        let mut entries = std::array::from_fn(|_| (unsafe { std::mem::zeroed() }, V::default()));
        let mut len = 0;
        for &node in cycle_group.iter() {
            entries[len] = (node, V::default());
            len += 1;
        }
        Self { entries, len }
    }

    pub unsafe fn entries_mut(&mut self) -> &mut [(S::Node, V)] {
        &mut self.entries[..self.len]
    }
}

struct LocalGraph<'db, C: IsContext<'db>>
where
    [(); <C::Scheme>::N]:,
{
    ctx: C,
    map: Map<C::Scheme, usize>,
    deps: Vec<Vec<usize>>,
}

impl<'db, C: IsContext<'db>> LocalGraph<'db, C>
where
    [(); <C::Scheme>::N]:,
{
    // This function triggers the ICE
    fn new(ctx: C, cycle_group: &'db CycleGroup<C::Scheme>) -> Self {
        Self {
            ctx,
            map: Map::new(ctx, cycle_group),
            deps: cycle_group
                .iter()
                .map(|&node| {
                    ctx.deps(node)
                        .into_iter()
                        .filter_map(|dep_node| {
                            cycle_group
                                .iter()
                                .position(|&cycle_group_node| cycle_group_node == dep_node)
                        })
                        .collect()
                })
                .collect(),
        }
    }
}
