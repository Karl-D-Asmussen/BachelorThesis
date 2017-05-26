
use traits::ArrayLike;
use dummy::DummyArray;
use same::{Same, SameRef};
use unit::{Unit, UnitRef};

#[derive(Clone, Copy)]
pub struct Singleton<T : Copy>(pub T, pub usize);

pub struct SingletonRef<'a, T : 'a>(pub &'a T, pub usize);

impl<T : Copy> ArrayLike for Singleton<T> {
  type Entry = T;
  type Shape = Same<usize>;
  type Flat = Unit<T>;
  type Reshape = Singleton<T>;
  type Slice = Singleton<T>;
  type Reverse = Singleton<T>;
  type Diagonal = Singleton<T>;
  type Fixate = Singleton<T>;

  type Tile = DummyArray<T>;
  type Transpose = DummyArray<T>;
  
  fn rank(&self) -> usize { self.1 }
  fn len(&self) -> usize { 1 }
  fn shape(&self) -> Self::Shape { Same(1, self.1) }
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

impl<'a, T : 'a> ArrayLike for SingletonRef<'a, T> {
  type Entry = T;
  type Shape = Same<usize>;
  type Flat = UnitRef<'a, T>;
  type Reshape = SingletonRef<'a, T>;
  type Slice = SingletonRef<'a, T>;
  type Reverse = SingletonRef<'a, T>;
  type Diagonal = SingletonRef<'a, T>;
  type Fixate = SingletonRef<'a, T>;

  type Tile = DummyArray<T>;
  type Transpose = DummyArray<T>;
  
  fn rank(&self) -> usize { self.1 }
  fn len(&self) -> usize { 1 }
  fn shape(&self) -> Self::Shape { Same(1, self.1) }
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

