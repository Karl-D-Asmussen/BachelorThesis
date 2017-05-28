
use traits::*;
use dummy::DummyArray;
use iter::FlatIter;
use constant::{Const, ConstRef};
use std::mem::transmute;

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
  type Reverse = Unit<T>;
  type Reshape = Unit<T>;
  type Slice = Unit<T>;

  type Tile = Const<T>;

  type Fixate = Unit<T>;
  type Transpose = Unit<T>;
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
  where I : ArrayLike<Entry = usize> { (*self).clone().into_reshape(shape) }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { (*self).clone().into_slice(cuboid) }

  fn tile(&self, len : usize) -> Self::Tile { (*self).clone().into_tile(len)   }

  fn reverse(&self, ax : usize) -> Self::Reverse { (*self).clone().into_reverse(ax) }

  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate {
    panic!("impossible operation Unit::fixate")
  }

  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose { 
    panic!("impossible operation Unit::transpose")
  }

  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal {
    panic!("impossible operation Unit::transpose")
  }
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

impl<T : Clone> IntoSlice for Unit<T> {
  type To = Unit<T>;
  fn into_slice<I>(self, cuboid : &I) -> Self::To
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    slice_check!(Unit::into_slice, self, cuboid);
    self
  }
}

impl<T : Clone> IntoReverse for Unit<T> {
  type To = Unit<T>;
  fn into_reverse(self, ax : usize) -> Self::To {
    axis_valid_check!(Unit::into_reverse, self, ax);
    self
  }
}

impl<T : Clone> IntoTile for Unit<T> {
  type To = Const<T>;
  fn into_tile(self, len : usize) -> Self::To {
    tile_check!(Unit::into_tile, len);
    Const::new(self.0, vec![len, 1])
  }
}

#[allow(unused_variables)]
impl<'a, T : 'a + Clone> ArrayLike for UnitRef<'a, T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = UnitRef<'a, T>;
  type Reshape = UnitRef<'a, T>;

  type Slice = UnitRef<'a, T>;
  type Tile = ConstRef<'a, T>;
  type Reverse = UnitRef<'a, T>;

  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
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
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    (*self).clone().into_slice(cuboid)
  }

  fn reverse(&self, ax : usize) -> Self::Reverse { (*self).clone().into_reverse(ax)  }

  fn tile(&self, len : usize) -> Self::Tile { (*self).clone().into_tile(len) }

  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate {
    panic!("impossible operation UnitRef::fixate")
  }
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose {
    panic!("impossible operation UnitRef::transpose")
  }
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal {
    panic!("impossible operation UnitRef::diagonal")
  }
}

impl<'a, T : 'a + Clone> IntoFlat for UnitRef<'a, T> {
  type To = UnitRef<'a, T>;
  fn into_flat(self) -> Self::To { self }
}

impl<'a, T : 'a + Clone> IntoReshape for UnitRef<'a, T> {
  type To = UnitRef<'a, T>;
  fn into_reshape<I>(self, shape : &I) -> Self::To
  where I : ArrayLike<Entry = usize> {
    reshape_check!(Unit::into_reshape, self, shape);
    self
  }
}

impl<'a, T : 'a + Clone> IntoSlice for UnitRef<'a, T> {
  type To = UnitRef<'a, T>;
  fn into_slice<I>(self, cuboid : &I) -> Self::To
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { 
    slice_check!(UnitRef::into_slice, self, cuboid);
    self
  }
}

impl<'a, T : 'a + Clone> IntoReverse for UnitRef<'a, T> {
  type To = UnitRef<'a, T>;
  fn into_reverse(self, ax : usize) -> Self::To { 
    axis_valid_check!(UnitRef::into_reverse, self, ax);
    self
  }
}

impl<'a, T : 'a + Clone> IntoTile for UnitRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_tile(self, len : usize) -> Self::To {
    tile_check!(UnitRef::into_tile, len);
    ConstRef::new(self.0, vec![len, 1]) 
  }
}

#[allow(unused_variables)]
impl<'a, T : 'a + Clone> ArrayLike for UnitMutRef<'a, T> {
  type Entry = T;
  type Shape = Unit<usize>;
  type Flat = UnitRef<'a, T>;
  type Reshape = UnitRef<'a, T>;
  type Slice = UnitRef<'a, T>;
  type Reverse = UnitRef<'a, T>;
  type Tile = ConstRef<'a, T>;

  type Diagonal = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  
  fn rank(&self) -> usize { 1 }
  fn len(&self) -> usize { 1 }
  fn shape(&self) -> &Self::Shape { UNIT_SHAPE }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(UnitMutRef::get, self, coord);
    &*self.0
  }

  fn flat(&self) -> Self::Flat { UnitRef(unsafe { transmute(&*self.0) }) }
  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> {
    reshape_check!(UnitMutRef::reshape, self, shape);
    UnitRef(unsafe { transmute(&*self.0) })
  }
  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    slice_check!(UnitMutRef::reshape, self, cuboid);
    UnitRef(unsafe { transmute(&*self.0) })
  }
  fn tile(&self, len : usize) -> Self::Tile { 
    tile_check!(UnitMutRef::tile, len);
    ConstRef::new(unsafe { transmute(&*self.0) }, vec![len, 1])
  }
  fn reverse(&self, ax : usize) -> Self::Reverse { 
    axis_valid_check!(UnitMutRef::reverse, self, ax);
    UnitRef(unsafe { transmute(&*self.0) })
  }

  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate { unimplemented!() }
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose { unimplemented!() }
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal { unimplemented!() }
}

#[allow(unused_variables)]
impl<'a, T : 'a + Clone> ArrayLikeMut for UnitMutRef<'a, T> {
  type FlatMut = UnitMutRef<'a, T>;
  type ReshapeMut = UnitMutRef<'a, T>;
  type SliceMut = UnitMutRef<'a, T>;
  type ReverseMut = UnitMutRef<'a, T>;

  type FixateMut = DummyArray<T>;
  type TransposeMut = DummyArray<T>;
  type DiagonalMut = DummyArray<T>;
  
  fn set<I>(&mut self, coord : &I, val : <Self as ArrayLike>::Entry) -> &mut Self
  where I : ArrayLike<Entry = isize> {
    get_set_coord_check!(UnitMutRef::set, self, coord);
    *self.0 = val;
    self
  }

  fn flat_mut(&mut self) -> Self::FlatMut { 
    UnitMutRef(unsafe { transmute(&mut *self.0) })
  }
  fn reshape_mut<I>(&mut self, shape : &I) -> Self::ReshapeMut
  where I : ArrayLike<Entry = usize> { 
    reshape_check!(UnitMutRef::reshape_mut, self, shape);
    UnitMutRef(unsafe { transmute(&mut *self.0) })
  }
  fn slice_mut<I>(&mut self, cuboid : &I) -> Self::SliceMut
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { 
    slice_check!(UnitMutRef::slice_mut, self, cuboid);
    UnitMutRef(unsafe { transmute(&mut *self.0) })
  }
  fn reverse_mut(&mut self, ax : usize) -> Self::ReverseMut { 
    axis_valid_check!(UnitMutRef::reverse_mut, self, ax);
    UnitMutRef(unsafe { transmute(&mut *self.0) })
  }

  fn fixate_mut(&mut self, ax : usize, at : isize) -> Self::FixateMut { unimplemented!() }
  fn transpose_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::TransposeMut { unimplemented!() }
  fn diagonal_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::DiagonalMut { unimplemented!() }
}
