
use traits::{ArrayLike, ArrayLikeMut, IntoFlat};
use std::iter::Iterator;
use std::marker::PhantomData;
use singleton::Singleton;

pub struct FlatIter<'a, A : 'a, T : 'a>
where A : ArrayLike<Entry = T> {
  arr : <A as ArrayLike>::Flat,
  index : usize,
  phantom : PhantomData<&'a T>
}

impl<'a, A, T> FlatIter<'a, A, T> where A : ArrayLike<Entry = T> + 'a {
  fn new_flat(arr : &'a A) -> Self {
    FlatIter { arr : arr.flat(), index : 0, phatom : PhantomData }
  }
}

impl<'a, A : 'a, T : 'a> Iterator for FlatIter<'a, A, T> where A : ArrayLike<Entry = T> + 'a {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    let ix = self.index;
    if ix < self.arr.len() {
      self.index += 1;
      Some(self.arr.at(&Singleton(ix)))
    } else {
      None
    }
  }
}

pub struct FlatIterMut<'a, A : 'a, T : 'a>
where A : ArrayLike<Entry = T> + ArrayLikeMut {
  arr : <A as ArrayLikeMut>::FlatMut,
  index : usize,
  phantom : PhantomData<&'a mut T>
}

impl<'a, A, T> FlatIterMut<'a, A, T> where A : ArrayLike<Entry = T> {
  fn new(arr : &'a mut A) {
    FlatIter { arr : arr.flat_mut(), index : 0 }
  }
}

pub struct IntoFlatIter<A, T>
where A : ArrayLike<Entry = T> + ArrayLikeMut + IntoFlat {
  arr : <A as IntoFlat>::To,
  index : usize,
}

impl<'a, A, T> FlatIterMut<'a, A, T> where A : ArrayLike<Entry = T> + ArrayLikeMut {
  fn new(arr : A) {
    FlatIter { arr : arr.into_flat(), index : 0 }
  }
}
