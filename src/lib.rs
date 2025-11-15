// Minimal reproduction of rustc ICE in borrow checker
// Related to generic const parameters + complex lifetimes + associated types

#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

pub trait IsScheme {
    type Node;
    const N: usize;
}

pub trait IsContext<'db> {
    type Scheme: IsScheme;
}

pub struct Map<S: IsScheme, V>
where
    [(); S::N]:,
{
    _entries: [(S::Node, V); S::N],
}

impl<S: IsScheme, V> Map<S, V>
where
    [(); S::N]:,
{
    pub fn new<'db, C>(_ctx: C, _x: &'db ()) -> Self
    where
        C: IsContext<'db, Scheme = S>,
        V: Default,
    {
        let _entries = std::array::from_fn(|_| (unsafe { std::mem::zeroed() }, V::default()));
        Self { _entries }
    }

}

struct LocalGraph<'db, C: IsContext<'db>>
where
    [(); <C::Scheme>::N]:,
{
    map: Map<C::Scheme, usize>,
}

impl<'db, C: IsContext<'db>> LocalGraph<'db, C>
where
    [(); <C::Scheme>::N]:,
{
    // This function triggers the ICE
    fn new(ctx: C, x: &'db ()) -> Self {
        Self {
            map: Map::new(ctx, x),
        }
    }
}
