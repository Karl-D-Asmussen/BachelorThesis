
use traits::*;

struct OuterProd<'a, F, A, B, T, U, V>(F, &'a A, &'a B, Vec<usize>, usize)
where F : Fn(&'a T, &'a U) -> V,
      A : ArrayLike<Entry = T> + 'a,
      B : ArrayLike<Entry = U> + 'a;

