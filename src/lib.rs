#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
trait S { const N: usize; }
trait C<'a> { type S: S; }
struct M<T: S>([u8; T::N]) where [(); T::N]:;
impl<T: S> M<T> where [(); T::N]: { fn new<'a, U>() -> Self where U: C<'a, S = T> { Self([0; T::N]) } }
fn ice<'a, U: C<'a>>() -> M<U::S> where [(); U::S::N]: { M::new::<U>() }
