
use traits::*;
use dummy::DummyArray;
use unit::{Unit, UNIT_SHAPE};
use std::marker::PhantomData;
use iter::FlatIter;
use utils::slice_steps;

#[derive(Clone)]
pub struct Const<T : Clone>(T, Vec<usize>, usize);
#[derive(Clone)]
pub struct ConstRef<'a, T : 'a + Clone>(&'a T, Vec<usize>, usize);
#[derive(Clone, Copy)]
pub struct ConstSl<'a, T : Clone>(T, &'a [usize], usize);
#[derive(Clone, Copy)]
pub struct ConstRefSl<'a, T : 'a + Clone>(&'a T, &'a [usize], usize);

impl<T : Clone> Const<T> {
  fn new(val : T, shape : Vec<usize>) -> Self {
    let len = { shape.iter().fold(1, |a, b| a * b) };
    Const(val, shape, len)
  }
  fn to_ref_sl<'a>(&'a self) -> ConstRefSl<'a, T> {
    ConstRefSl(&self.0, &self.1[..], self.2)
  }
  fn to_ref<'a>(&'a self) -> ConstRef<'a, T> {
    ConstRef(&self.0, self.1.clone(), self.2)
  }
  fn to_sl<'a>(&'a self) -> ConstSl<'a, T> {
    ConstSl((self.0).clone(), &self.1[..], self.2)
  }
}

impl<'a, T : 'a + Clone> ConstRef<'a, T> {
  fn new(val : &'a T, shape : Vec<usize>) -> Self {
    let len = { shape.iter().fold(1, |a, b| a * b) };
    ConstRef(val, shape, len)
  }
  fn to_(&'a self) -> Const<T> {
    Const(self.0.clone(), self.1.clone(), self.2)
  }
  fn to_ref_sl(&'a self) -> ConstRefSl<'a, T> {
    ConstRefSl(self.0, &self.1[..], self.2)
  }
  fn to_sl(&'a self) -> ConstSl<'a, T> {
    ConstSl(self.0.clone(), &self.1[..], self.2)
  }
}

impl<'a, T : 'a + Clone> ConstSl<'a, T> {
  fn new(val : T, shape : &'a [usize]) -> Self {
    let len = { shape.iter().fold(1, |a, b| a * b) };
    ConstSl(val, shape, len)
  }
  fn to_(&self) -> Const<T> {
    Const(self.0.clone(), self.1.to_owned(), self.2)
  }
  fn to_ref(&'a self) -> ConstRef<'a, T> {
    ConstRef(&self.0, self.1.to_owned(), self.2)
  }
  fn to_ref_sl(&'a self) -> ConstRefSl<'a, T> {
    ConstRefSl(&self.0, self.1, self.2)
  }
}

impl<'a, T : 'a + Clone> ConstRefSl<'a, T> {
  fn new(val : &'a T, shape : &'a [usize]) -> Self {
    let len = { shape.iter().fold(1, |a, b| a * b) };
    ConstRefSl(val, shape, len)
  }
  fn to_(&self) -> Const<T> {
    Const(self.0.clone(), self.1.to_owned(), self.2)
  }
  fn to_ref(&'a self) -> ConstRef<'a, T> {
    ConstRef(self.0, self.1.to_owned(), self.2)
  }
  fn to_sl(&'a self) -> ConstSl<'a, T> {
    ConstSl(self.0.clone(), self.1, self.2)
  }
}

impl<T> ArrayLike for Const<T> where T : Clone {
  type Entry = T;
  type Shape = Vec<usize>;
  type Flat = Const<T>;
  type Reshape = Const<T>;

  type Slice = Const<T>;
  type Fixate = Const<T>;
  type Transpose = Const<T>;
  type Reverse = Const<T>;
  type Diagonal = Const<T>;
  type Tile = Const<T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { &self.1 }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(Constant::get, self, coord);
    &self.0
  }

  fn flat(&self) -> Self::Flat { self.clone().into_flat() }

  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> {
    self.clone().into_reshape(shape)
  }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    self.clone().into_slice(cuboid)
  }

  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate {
    self.clone().into_fixate(ax, at)
  }

  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose {
    self.clone().into_transpose(ax1, ax2)
  }

  fn reverse(&self, ax : usize) -> Self::Reverse {
    self.clone().into_reverse(ax)
  }

  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal {
    self.clone().into_diagonal(ax1, ax2)
  }

  fn tile(&self, len : usize) -> Self::Tile {
    self.clone().into_tile(len)
  }
}

impl<T : Clone> IntoFlat for Const<T> {
  type To = Const<T>;
  fn into_flat(mut self) -> Self::To {
    let len = self.len();
    self.1 = vec![len];
    self
  }
}

impl<T : Clone> IntoSlice for Const<T> {
  type To = Const<T>;
  fn into_slice<I>(mut self, cuboid : &I) -> Self::To
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    slice_check!(Const::slice, self, cuboid);
    for (ax, sl) in self.1.iter_mut().zip(FlatIter::new(cuboid)) {
      let v = *ax;
      *ax = slice_steps(v, sl).expect("zero-step slice passed in Const::into_slice");
    }
    self.2 = self.1.iter().fold(1, |a, b| a * b);
    self
  }
}

impl<T : Clone> IntoReshape for Const<T> {
  type To = Const<T>;
  fn into_reshape<I>(mut self, shape : &I) -> Self::To
    where I : ArrayLike<Entry = usize> {
    reshape_check!(Const::into_reshape, self, shape);
    for (ax, k) in self.1.iter_mut().zip(FlatIter::new(shape)) {
      *ax = *k
    }
    self
  }
}

impl<T : Clone> IntoFixate for Const<T> {
  type To = Const<T>;
  fn into_fixate(mut self, ax : usize, at : isize) -> Self::To {
    fixate_check!(Const::into_fixate, self, ax, at);
    self.1.remove(ax);
    self
  }
}

impl<T : Clone> IntoTranspose for Const<T> {
  type To = Const<T>;
  fn into_transpose(mut self, ax1 : usize, ax2 : usize) -> Self::To {
    transpose_check!(Const::into_transpose, self, ax1, ax2);
    self.1.swap(ax1, ax2);
    self
  }
}

impl<T : Clone> IntoReverse for Const<T> {
  type To = Const<T>;
  fn into_reverse(mut self, ax : usize) -> Self::To {
    axis_valid_check!(Const::into_reverse, self, ax);
    self
  }
}

impl<T : Clone> IntoDiagonal for Const<T> {
  type To = Const<T>;
  fn into_diagonal(mut self, ax1 : usize, ax2 : usize) -> Self::To {
    diagonal_check!(Const::diagonal, self, ax1, ax2);    
    self.1.remove(ax2);
    self
  }
}

impl<T : Clone> IntoTile for Const<T> {
  type To = Const<T>;
  fn into_tile(mut self, len : usize) -> Self::To {
    self.1.insert(0, len);
    self
  }
}

impl<'a, T> ArrayLike for ConstRef<'a, T> where T : Clone + 'a {
  type Entry = T;
  type Shape = Vec<usize>;
  type Flat = ConstRef<'a, T>;
  type Reshape = ConstRef<'a, T>;

  type Slice = ConstRef<'a, T>;
  type Fixate = ConstRef<'a, T>;
  type Transpose = ConstRef<'a, T>;
  type Reverse = ConstRef<'a, T>;
  type Diagonal = ConstRef<'a, T>;
  type Tile = ConstRef<'a, T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { &self.1 }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(Constant::get, self, coord);
    self.0
  }

  fn flat(&self) -> Self::Flat { self.clone().into_flat() }

  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> {
    self.clone().into_reshape(shape)
  }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    self.clone().into_slice(cuboid)
  }

  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate {
    self.clone().into_fixate(ax, at)
  }

  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose {
    self.clone().into_transpose(ax1, ax2)
  }

  fn reverse(&self, ax : usize) -> Self::Reverse {
    self.clone().into_reverse(ax)
  }

  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal {
    self.clone().into_diagonal(ax1, ax2)
  }

  fn tile(&self, len : usize) -> Self::Tile {
    self.clone().into_tile(len)
  }
}

impl<'a, T : Clone + 'a> IntoFlat for ConstRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_flat(mut self) -> Self::To {
    let len = self.len();
    self.1 = vec![len];
    self
  }
}

impl<'a, T : Clone + 'a> IntoSlice for ConstRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_slice<I>(mut self, cuboid : &I) -> Self::To
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    slice_check!(Const::slice, self, cuboid);
    for (ax, sl) in self.1.iter_mut().zip(FlatIter::new(cuboid)) {
      let v = *ax;
      *ax = slice_steps(v, sl).expect("zero-step slice passed in Const::into_slice");
    }
    self.2 = self.1.iter().fold(1, |a, b| a * b);
    self
  }
}

impl<'a, T : Clone + 'a> IntoReshape for ConstRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_reshape<I>(mut self, shape : &I) -> Self::To
    where I : ArrayLike<Entry = usize> {
    reshape_check!(Const::into_reshape, self, shape);
    for (ax, k) in self.1.iter_mut().zip(FlatIter::new(shape)) {
      *ax = *k
    }
    self
  }
}

impl<'a, T : Clone + 'a> IntoFixate for ConstRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_fixate(mut self, ax : usize, at : isize) -> Self::To {
    fixate_check!(Const::into_fixate, self, ax, at);
    self.1.remove(ax);
    self
  }
}

impl<'a, T : Clone + 'a> IntoTranspose for ConstRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_transpose(mut self, ax1 : usize, ax2 : usize) -> Self::To {
    transpose_check!(Const::into_transpose, self, ax1, ax2);
    self.1.swap(ax1, ax2);
    self
  }
}

impl<'a, T : Clone + 'a> IntoReverse for ConstRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_reverse(mut self, ax : usize) -> Self::To {
    axis_valid_check!(Const::into_reverse, self, ax);
    self
  }
}

impl<'a, T : Clone + 'a> IntoDiagonal for ConstRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_diagonal(mut self, ax1 : usize, ax2 : usize) -> Self::To {
    diagonal_check!(Const::diagonal, self, ax1, ax2);    
    self.1.remove(ax2);
    self
  }
}

impl<'a, T : Clone + 'a> IntoTile for ConstRef<'a, T> {
  type To = ConstRef<'a, T>;
  fn into_tile(mut self, len : usize) -> Self::To {
    self.1.insert(0, len);
    self
  }
}

impl<'a, T> ArrayLike for ConstRefSl<'a, T> where T : Clone + 'a {
  type Entry = T;
  type Shape = [usize];
  type Flat = Const<T>;
  type Reshape = Const<T>;

  type Slice = Const<T>;
  type Fixate = Const<T>;
  type Transpose = Const<T>;
  type Reverse = Const<T>;
  type Diagonal = Const<T>;
  type Tile = Const<T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { &self.1 }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(Constant::get, self, coord);
    self.0
  }

  fn flat(&self) -> Self::Flat { self.to_().into_flat() }

  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> {
    self.to_().into_reshape(shape)
  }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    self.to_().into_slice(cuboid)
  }

  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate {
    self.to_().into_fixate(ax, at)
  }

  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose {
    self.to_().into_transpose(ax1, ax2)
  }

  fn reverse(&self, ax : usize) -> Self::Reverse {
    self.to_().into_reverse(ax)
  }

  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal {
    self.to_().into_diagonal(ax1, ax2)
  }

  fn tile(&self, len : usize) -> Self::Tile {
    self.to_().into_tile(len)
  }
}

impl<'a, T> ArrayLike for ConstSl<'a, T> where T : Clone + 'a {
  type Entry = T;
  type Shape = [usize];
  type Flat = Const<T>;
  type Reshape = Const<T>;

  type Slice = Const<T>;
  type Fixate = Const<T>;
  type Transpose = Const<T>;
  type Reverse = Const<T>;
  type Diagonal = Const<T>;
  type Tile = Const<T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { &self.1[..] }
  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(Constant::get, self, coord);
    &self.0
  }

  fn flat(&self) -> Self::Flat { self.to_().into_flat() }

  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> {
    self.to_().into_reshape(shape)
  }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> {
    self.to_().into_slice(cuboid)
  }

  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate {
    self.to_().into_fixate(ax, at)
  }

  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose {
    self.to_().into_transpose(ax1, ax2)
  }

  fn reverse(&self, ax : usize) -> Self::Reverse {
    self.to_().into_reverse(ax)
  }

  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal {
    self.to_().into_diagonal(ax1, ax2)
  }

  fn tile(&self, len : usize) -> Self::Tile {
    self.to_().into_tile(len)
  }
}
