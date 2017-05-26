
use traits::ArrayLike;
use same::{Same, SameRef};
use dummy::DummyArray;

#[derive(Clone, Copy)]
pub struct Unit<T : Copy>(pub T);

pub struct UnitRef<'a, T : 'a>(pub &'a T);

impl<T : Copy> ArrayLike for Unit<T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = Unit<T>;
  type Reshape = Unit<T>;
  type Slice = Unit<T>;
  type Tile = Same<T>;

  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { 1 }
  fn shape(&self) -> Self::Shape { Unit(1) }
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

impl<'a, T : 'a> ArrayLike for UnitRef<'a, T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = UnitRef<'a, T>;
  type Reshape = UnitRef<'a, T>;
  type Slice = UnitRef<'a, T>;
  type Tile = SameRef<'a, T>;

  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { 1 }
  fn shape(&self) -> Self::Shape { Unit(1) }
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
