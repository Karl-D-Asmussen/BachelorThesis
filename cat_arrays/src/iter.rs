
use traits::ArrayLike;
use std::iter::Iterator;
use unit::Unit;

pub struct FlatIter<'a, A : 'a, T : 'a>
where A : ArrayLike<Entry = T> {
  arr : &'a A,
  index : usize,
}

impl<'a, A, T> FlatIter<'a, A, T> where A : ArrayLike<Entry = T> + 'a {
  pub fn new(arr : &'a A) -> Self {
    FlatIter { arr, index : 0 }
  }
}

impl<'a, A : 'a, T : 'a> Iterator for FlatIter<'a, A, T> where A : ArrayLike<Entry = T> {
  type Item = &'a T;
  fn next(&mut self) -> Option<Self::Item> {
    let ix = self.index;
    if ix < self.arr.len() {
      self.index += 1;
      Some(self.arr.get(&Unit(ix as isize)))
    } else {
      None
    }
  }
}
