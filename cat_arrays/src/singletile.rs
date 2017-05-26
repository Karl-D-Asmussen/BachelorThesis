
use dummy::DummyArray;
use traits::ArrayLike;
use same::SameRef;
use std::borrow::{Cow, Borrow};

pub struct SingletileRef<'a, T : 'a>(pub &'a T, pub Cow<'a, [usize]>);

impl<'a, T : 'a> ArrayLike for SingletileRef<'a, T> {
  type Entry = T;
  type Shape = Vec<usize>;
  type Flat = SameRef<'a, T>;
  type Reshape = SingletileRef<'a, T>;
  type Slice = SingletileRef<'a, T>;
  type Fixate = SingletileRef<'a, T>;
  type Transpose = SingletileRef<'a, T>;
  type Reverse = SingletileRef<'a, T>;
  type Diagonal = SingletileRef<'a, T>;
  type Tile = SingletileRef<'a, T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.1.iter().fold(1, |a, b| a * b) }
  fn shape(&self) -> Self::Shape { self.1.to_vec() }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> {
      bounds_check!(SingletileRef :: get, self, coord);
      self.0
  }

  fn flat(&self) -> Self::Flat { SameRef(self.0, self.len()) }

  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> { unimplemented!() }
  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { unimplemented!() }
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate { unimplemented!() }
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose { unimplemented!() }
  fn reverse(&self, ax1 : usize) -> Self::Reverse { unimplemented!() }
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal { unimplemented!() }
  fn tile(&self, len : usize) -> Self::Tile { unimplemented!() }
}
