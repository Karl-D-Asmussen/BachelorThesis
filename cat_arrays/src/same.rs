
use traits::ArrayLike;
use unit::Unit;
use dummy::DummyArray;

#[derive(Clone, Copy)]
pub struct Same<T : Copy>(pub T, pub usize);

pub struct SameRef<'a, T : 'a>(pub &'a T, pub usize);

impl<T : Copy> ArrayLike for Same<T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = Same<T>;
  type Slice = Same<T>;

  type Tile = DummyArray<T>;
  type Reshape = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { self.1 }
  fn shape(&self) -> Self::Shape { Unit(self.1) }
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


impl<'a, T : 'a> ArrayLike for SameRef<'a, T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = SameRef<'a, T>;
  type Slice = SameRef<'a, T>;

  type Tile = DummyArray<T>;
  type Reshape = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { self.1 }
  fn shape(&self) -> Self::Shape { Unit(self.1) }
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

