
use traits::*;
use std::default::Default;

type Reduce<'a, F, T, A> = Fold<'a, F, T, T, A>
where T : Default,
      F : FnMut(T, &'a T) -> T,
      A : ArrayLike<Entry = T>
;

pub struct Fold<'a, F, T, U, A>(&'a A, F, U, Vec<usize>, usize)
where F : FnMut(U, &'a T) -> U,
      U : Clone,
      A : ArrayLike<Entry = T>
;

impl<'a, F, T, U, A> Fold<'a, F, T, U, A>
where F : FnMut(U, &'a T) -> U,
      U : Clone,
      A : ArrayLike<Entry = T> {

    fn new(

}


impl<'a, F, T, U, A> ArrayLike for Fold<'a, F, T, U, A> where A : ArrayLike<Entry = T> {
  type Entry = T;
  type Shape = A::Shape;
  type Flat = DummyArray<T>;
  type Reshape = DummyArray<T>;
  type Slice = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Tile = DummyArray<T>;
  
  fn rank(&self) -> usize { self.1.rank() - 1 }
  fn len(&self) -> usize { self.1.shape() }
  fn shape(&self) -> &Self::Shape { unimplemented!() }
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


