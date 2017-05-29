
use traits::ArrayLike;
use std::iter::Iterator;
use unit::Unit;

pub struct FlatIter<'a, A : 'a + ?Sized, T : 'a>(&'a A, isize)
    where A : ArrayLike<Entry = T>;

impl<'a, A, T> FlatIter<'a, A, T> where A : ArrayLike<Entry = T> + 'a + ?Sized {
  pub fn new(arr : &'a A) -> Self {
    rank1_check!(FlatIter::new, arr);
    FlatIter(arr, 0)
  }
}

impl<'a, A : 'a, T : 'a> Iterator for FlatIter<'a, A, T> where A : ArrayLike<Entry = T> + ?Sized {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    let ix = self.1;
    if ix < (self.0.len() as isize) {
      self.1 += 1;
      Some(self.0.get(&Unit(ix)))
    } else {
      None
    }
  }
}

pub struct FlatIterRev<'a, A : 'a + ?Sized, T : 'a>(&'a A, isize)
    where A : ArrayLike<Entry = T>;

impl<'a, A, T> FlatIterRev<'a, A, T> where A : ArrayLike<Entry = T> + 'a + ?Sized {
  pub fn new(arr : &'a A) -> Self {
    rank1_check!(FlatIter::new, arr);
    FlatIterRev(arr, (arr.len() as isize)-1)
  }
}

impl<'a, A : 'a, T : 'a> Iterator for FlatIterRev<'a, A, T> where A : ArrayLike<Entry = T> + ?Sized {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    let ix = self.1;
    if ix >= 0 {
      self.1 -= 1;
      Some(self.0.get(&Unit(ix)))
    } else {
      None
    }
  }
}
