
use traits::ArrayLike;
use dummy::DummyArray;
use unit::{Unit, UnitRef};
use iter::FlatIter;
use std::borrow::Cow;

#[derive(Clone, Copy)]
pub struct Constant<'a, T : 'a>(Cow<'a, T>, Cow<'a, [usize]>, usize);

impl<'a, T : 'a> Constant<'a, T> {
  fn new(val : &'a T, shape : &'a [usize]) -> Self {
    non_null_shape_check!(Constant::new, shape);
    Constant(val.into(), sh.into(), sh.iter().fold(1, |a, b| a * b))
  }

  fn from(val : T, shape : &'a [usize]) -> Self {
    non_null_shape_check!(Constant::new, shape);
    Constant(val.into(), sh.into(), sh.iter().fold(1, |a, b| a * b))
  }

  fn new_gen<S>(val : &'a T, shape : S) -> Self
    where S : ArrayLike<Entry = usize> {
    non_null_shape_check!(Constant::new, shape);
    let mut sh = Vec::with_capacity(S.len());
    let mut len = 1;
    for ax in FlatIter::new(&shape) {
        sh.push(ax);
        len *= ax;
    }
    Constant(val, sh.into(), len)
  }

  fn from_gen<S>(val : T, shape : S) -> Self 
    where S : ArrayLike<Entry = usize> {
    non_null_shape_check!(Constant::new, shape);
    let mut sh = Vec::with_capacity(S.len());
    let mut len = 1;
    for ax in FlatIter::new(&shape) {
        sh.push(ax);
        len *= ax;
    }
    Constant(val, sh.into(), len)
  }
}

impl<'a, T : 'a> ArrayLike for Constant<'a, T> {
  type Entry = T;
  type Shape = &[usize];
  type Flat = Unit<T>;

  type Reshape = Constant<T>;
  type Slice = Constant<T>;
  type Reverse = Constant<T>;
  type Diagonal = Constant<T>;
  type Fixate = Constant<T>;
  type Tile = Constant<T>;
  type Transpose = Constant<T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { self.1 }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> {
      get_set_coord_check!(Constant::get, self, coord);
      self.0
  }

  fn flat(&self) -> Self::Flat { Unit(self.0) }

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

impl<'a, T : 'a, S> ArrayLike for ConstantRef<'a, T, S> where S : ArrayLike<Entry = usize> {
  type Entry = T;
  type Shape = S;
  type Flat = UnitRef<'a, T>;
  type Reshape = ConstantRef<'a, T>;
  type Slice = ConstantRef<'a, T>;
  type Reverse = ConstantRef<'a, T>;
  type Diagonal = ConstantRef<'a, T>;
  type Fixate = ConstantRef<'a, T>;

  type Tile = DummyArray<T>;
  type Transpose = DummyArray<T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { self.1 }
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
