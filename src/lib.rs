#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait IsScheme {
    type Node;
    const N: usize;
}

trait IsContext<'db> {
    type Scheme: IsScheme;
}

struct Map<S: IsScheme>
where
    [(); S::N]:,
{
    _data: [S::Node; S::N],
}

impl<S: IsScheme> Map<S>
where
    [(); S::N]:,
{
    fn new<'db, C>(_ctx: C) -> Self
    where
        C: IsContext<'db, Scheme = S>,
    {
        Self {
            _data: unsafe { std::mem::zeroed() },
        }
    }
}

fn trigger_ice<'db, C: IsContext<'db>>(ctx: C) -> Map<C::Scheme>
where
    [(); <C::Scheme>::N]:,
{
    Map::new(ctx)
}
