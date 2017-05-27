
use traits::ArrayLike;
use dummy::DummyArray;
use unit::Unit;
use std::ops::{Deref, DerefMut};

impl<A, T> ArrayLike for A where A : Deref<Target = [T]> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = DummyArray<T>;
  type Reshape = DummyArray<T>;
  type Slice = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Tile = DummyArray<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { self.deref().len() }
  fn shape(&self) -> &Self::Shape { &Unit(self.len()) }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { unimplemented!() }

  fn flat(&self) -> Self::Flat { unimplemented!() }
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
