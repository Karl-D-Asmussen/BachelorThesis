
use traits::*;
use dummy::DummyArray;
use iter::FlatIter;

#[derive(Clone, Copy)]
pub struct Unit<T : Clone>(pub T);

#[derive(Copy)]
pub struct UnitRef<'a, T : 'a>(pub &'a T);
impl<'a, T : 'a> Clone for UnitRef<'a, T> { fn clone(&self) -> Self { UnitRef(self.0) } }

pub struct UnitMutRef<'a, T : 'a>(pub &'a mut T);

pub const UNIT_SHAPE : &'static Unit<usize> = &Unit(1);

#[allow(unused_variables)]
impl<T : Clone> ArrayLike for Unit<T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = Unit<T>;

  type Reshape = Unit<T>;
  type Slice = Unit<T>;
  type Tile = Unit<T>;
  type Fixate = Unit<T>;
  type Transpose = Unit<T>;
  type Reverse = Unit<T>;
  type Diagonal = Unit<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { 1 }
  fn shape(&self) -> &Self::Shape { UNIT_SHAPE }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(Unit::get, self, coord);
    &self.0
  }
  fn flat(&self) -> Self::Flat { (*self).clone().into_flat() }

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

impl<T : Clone> IntoFlat for Unit<T> {
  type To = Unit<T>;
  fn into_flat(self) -> Self::To { self }
}

impl<T : Clone> IntoReshape for Unit<T> {
  type To = Unit<T>;
  fn into_reshape<I>(self, shape : &I) -> Self::To
  where I : ArrayLike<Entry = usize> {
    reshape_check!(Unit::into_reshape, self, shape);
    self
  }
}

#[allow(unused_variables)]
impl<'a, T : 'a> ArrayLike for UnitRef<'a, T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = UnitRef<'a, T>;
  type Reshape = UnitRef<'a, T>;

  type Slice = DummyArray<T>;
  type Tile = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { 1 }
  fn shape(&self) -> &Self::Shape { &UNIT_SHAPE }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(Unit::get, self, coord);
    self.0
  }
  fn flat(&self) -> Self::Flat { (*self).clone().into_flat() }

  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> { (*self).clone().into_reshape(shape) }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { unimplemented!() }
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate { unimplemented!() }
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose { unimplemented!() }
  fn reverse(&self, ax1 : usize) -> Self::Reverse { unimplemented!() }
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal { unimplemented!() }
  fn tile(&self, len : usize) -> Self::Tile { unimplemented!() }
}

impl<'a, T : 'a> IntoFlat for UnitRef<'a, T> {
  type To = UnitRef<'a, T>;
  fn into_flat(self) -> Self::To { self }
}

impl<'a, T : 'a> IntoReshape for UnitRef<'a, T> {
  type To = UnitRef<'a, T>;
  fn into_reshape<I>(self, shape : &I) -> Self::To
  where I : ArrayLike<Entry = usize> {
    reshape_check!(Unit::into_reshape, self, shape);
    self
  }
}
