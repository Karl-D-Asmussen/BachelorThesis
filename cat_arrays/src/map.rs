
use traits::*;
use std::cell::Cell;

struct Map<'a, F, A, T, U>(F, Cell<Option<U>>, &'a A) where A : ArrayLike<Entry = T>, F : FnMut(T) -> U;
struct Map2<'a, F, A, T, U, V>(F, &'a A) where A : ArrayLike<Entry = T>, F : FnMut(T, V) -> U;

#[allow(unused_variables)]
impl<'a, F, A, T, U> ArrayLike for Map<'a, F, A, T, U>
where A : ArrayLike<Entry = T>, F : FnMut(T) -> U {
  type Entry = U;
  type Shape = A::Shape;

  type Flat = DummyArray<T>;
  type Reshape = DummyArray<T>;
  type Slice = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Tile = DummyArray<T>;
  
  fn rank(&self) -> usize { self.1.rank() }
  fn len(&self) -> usize { self.1.len() }
  fn shape(&self) -> &Self::Shape { self.1.shape() }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> {
    get_set_coord_check!(Map::get, self, coord);
    self.0(self.1.get(coord))
  }

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

