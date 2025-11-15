#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

trait Scheme {
    type Node;
    const N: usize;
}

trait Context<'a> {
    type S: Scheme;
}

struct Map<S: Scheme>
where
    [(); S::N]:,
{
    _data: [S::Node; S::N],
}

impl<S: Scheme> Map<S>
where
    [(); S::N]:,
{
    fn new<'a, C>() -> Self
    where
        C: Context<'a, S = S>,
    {
        Self {
            _data: unsafe { std::mem::zeroed() },
        }
    }
}

fn trigger_ice<'a, C: Context<'a>>() -> Map<C::S>
where
    [(); C::S::N]:,
{
    Map::new::<C>()
}
