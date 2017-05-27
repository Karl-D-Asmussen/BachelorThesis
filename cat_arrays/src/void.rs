
use traits::ArrayLike;
use std::marker::PhantomData;
use dummy::DummyArray;
use unit::Unit;
use iter::FlatIter;
use utils;

pub struct Void<T>(PhantomData<T>);

#[allow(unused_variables)]
impl<T> ArrayLike for Void<T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = Void<T>;
  type Slice = Void<T>;
  type Reverse = Void<T>;

  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Tile = DummyArray<T>;
  type Reshape = DummyArray<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { 0 }
  fn shape(&self) -> &Self::Shape { &Unit(0) }

  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> {
      coord_check!(Void::get, self, coord);
      panic!("{}:{}:{}: Void not ::get() -able", file!(), line!(), column!())
  }

  fn flat(&self) -> Self::Flat { Void(PhantomData) }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { unimplemented!() }

  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> { unimplemented!() }
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate { unimplemented!() }
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose { unimplemented!() }
  fn reverse(&self, ax1 : usize) -> Self::Reverse { unimplemented!() }
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal { unimplemented!() }
  fn tile(&self, len : usize) -> Self::Tile { unimplemented!() }
}

