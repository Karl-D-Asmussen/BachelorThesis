
use traits::*;
use iter::*;
use utils::*;
use dummy::*;
use std::mem::transmute;

pub struct GenReshape<T, A : Clone>(A, Vec<usize>, usize) where A : ArrayLike<Entry = T>;
pub struct GenReshapeRef<'a, T, A : 'a>(&'a A, Vec<usize>, usize) where A : ArrayLike<Entry = T>;
pub struct GenReshapeMutRef<'a, T, A : 'a>(&'a mut A, Vec<usize>, usize) where A : ArrayLike<Entry = T>;

impl<T : Clone, A : Clone> GenReshape<T, A> where A : ArrayLike<Entry=T> {
    pub fn new(arr : A, shape : Vec<usize>)  -> Self{
        let len = shape.iter().fold(1, |a, b| a * b);
        GenReshape(arr, shape, len)
    }
}

impl<T : Clone, A : Clone> Clone for GenReshape<T, A> where A : ArrayLike<Entry = T> {
    fn clone(&self) -> Self {
        GenReshape(self.0.clone(), self.1.clone(), self.2)
    }
}

impl<'a, T, A : 'a> GenReshapeRef<'a, T, A> where A : ArrayLike<Entry=T> {
    pub fn new(arr : &'a A, shape : Vec<usize>) -> Self {
        let len = shape.iter().fold(1, |a, b| a * b);
        GenReshapeRef(arr, shape, len)
    }
}

impl<'a, T, A : 'a> GenReshapeMutRef<'a, T, A> where A : ArrayLike<Entry=T> + ArrayLikeMut {
    pub fn new(arr : &'a mut A, shape : Vec<usize>) -> Self {
        let len = shape.iter().fold(1, |a, b| a * b);
        GenReshapeMutRef(arr, shape, len)
    }
}

#[allow(unused_variables)]
impl<T : Clone, A> ArrayLike for GenReshape<T, A> where A : ArrayLike<Entry = T> + Clone {
  type Entry = T;
  type Shape = Vec<usize>;
  type Flat = GenReshape<T, A>;
  type Reshape = GenReshape<T, A>;

  type Slice = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Tile = DummyArray<T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { &self.1 }

  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(GenReshape::get, self, coord);
    let flat = from_mixed_radix(FlatIterRev::new(coord), FlatIterRev::new(self.shape()));
    let new_coord = to_mixed_radix(self.len(), FlatIter::new(self.shape()), flat);
    self.0.get(&new_coord)
  }

  fn flat(&self) -> Self::Flat { 
    (*self).clone().into_flat()
  }
  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> { 
    (*self).clone().into_reshape(shape)
  }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { unimplemented!() }
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate { unimplemented!() }
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose { unimplemented!() }
  fn reverse(&self, ax1 : usize) -> Self::Reverse { unimplemented!() }
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal { unimplemented!() }
  fn tile(&self, len : usize) -> Self::Tile { unimplemented!() }
}

impl<T : Clone, A> IntoFlat for GenReshape<T, A> where A : ArrayLike<Entry = T> + Clone {
  type To = Self;
  fn into_flat(mut self) -> Self::To {
    let len = self.len();
    self.1.truncate(1);
    self.1[0] = len;
    self
  }
}

impl<T : Clone, A> IntoReshape for GenReshape<T, A> where A : ArrayLike<Entry = T> + Clone {
  type To = Self;
  fn into_reshape<I>(mut self, shape : &I) -> Self::To 
  where I : ArrayLike<Entry = usize> {
    reshape_check!(GenReshape::into_reshape, self, shape);
    self.1.clear();
    self.1.extend(FlatIter::new(shape).map(|p| *p));
    self
  }
}

impl<'a, T, A : 'a> Clone for GenReshapeRef<'a, T, A> where A : ArrayLike<Entry = T> {
  fn clone(&self) -> Self {
    GenReshapeRef(unsafe { transmute(&*self.0) }, self.1.clone(), self.2)
  }
}

#[allow(unused_variables)]
impl<'a, T, A : 'a> ArrayLike for GenReshapeRef<'a, T, A> where A : ArrayLike<Entry = T>  {
  type Entry = T;
  type Shape = Vec<usize>;
  type Flat = GenReshapeRef<'a, T, A>;
  type Reshape = GenReshapeRef<'a, T, A>;
  type Slice = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Tile = DummyArray<T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { &self.1 }

  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(GenReshape::get, self, coord);
    let flat = from_mixed_radix(FlatIterRev::new(coord), FlatIterRev::new(self.shape()));
    let new_coord = to_mixed_radix(self.len(), FlatIter::new(self.shape()), flat);
    self.0.get(&new_coord)
  }

  fn flat(&self) -> Self::Flat { 
    (*self).clone().into_flat()
  }

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

impl<'a, T, A : 'a> IntoFlat for GenReshapeRef<'a, T, A> where A : ArrayLike<Entry = T> {
  type To = Self;
  fn into_flat(mut self) -> Self::To {
    let len = self.len();
    self.1.truncate(1);
    self.1[0] = len;
    self
  }
}

impl<'a, T, A : 'a> IntoReshape for GenReshapeRef<'a, T, A> where A : ArrayLike<Entry = T> {
  type To = Self;
  fn into_reshape<I>(mut self, shape : &I) -> Self::To 
  where I : ArrayLike<Entry = usize> {
    reshape_check!(GenReshape::into_reshape, self, shape);
    self.1.clear();
    self.1.extend(FlatIter::new(shape).map(|p| *p));
    self
  }
}

impl<'a, T, A : 'a>  GenReshapeMutRef<'a, T, A> where A : ArrayLike<Entry = T>  {
    fn immut(&self) -> GenReshapeRef<'a, T, A> {
        GenReshapeRef(unsafe { transmute(&*self.0) }, self.1.clone(), self.2)
    }
}

#[allow(unused_variables)]
impl<'a, T, A : 'a> ArrayLike for GenReshapeMutRef<'a, T, A> where A : ArrayLike<Entry = T>  {
  type Entry = T;
  type Shape = Vec<usize>;
  type Flat = GenReshapeRef<'a, T, A>;
  type Reshape = GenReshapeRef<'a, T, A>;

  type Slice = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Tile = DummyArray<T>;
  
  fn rank(&self) -> usize { self.1.len() }
  fn len(&self) -> usize { self.2 }
  fn shape(&self) -> &Self::Shape { &self.1 }

  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { 
    get_set_coord_check!(GenReshape::get, self, coord);
    let flat = from_mixed_radix(FlatIterRev::new(coord), FlatIterRev::new(self.shape()));
    let new_coord = to_mixed_radix(self.len(), FlatIter::new(self.shape()), flat);
    self.0.get(&new_coord)
  }

  fn flat(&self) -> Self::Flat { 
      self.immut().into_flat()
  }

  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> {
      // This does not give good errors
      self.immut().into_reshape(shape)
  }

  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { unimplemented!() }
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate { unimplemented!() }
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose { unimplemented!() }
  fn reverse(&self, ax1 : usize) -> Self::Reverse { unimplemented!() }
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal { unimplemented!() }
  fn tile(&self, len : usize) -> Self::Tile { unimplemented!() }
}

#[allow(unused_variables)]
impl<'a, T, A : 'a> ArrayLikeMut for GenReshapeMutRef<'a, T, A> where A : ArrayLike<Entry = T> + ArrayLikeMut {
  type FlatMut = GenReshapeMutRef<'a, T, A>;
  type ReshapeMut = GenReshapeMutRef<'a, T, A>;

  type SliceMut = DummyArray<T>;
  type ReverseMut = DummyArray<T>;
  type FixateMut = DummyArray<T>;
  type TransposeMut = DummyArray<T>;
  type DiagonalMut = DummyArray<T>;
  
  fn set<I>(&mut self, coord : &I, val : <Self as ArrayLike>::Entry) -> &mut Self
  where I : ArrayLike<Entry = isize> {
    get_set_coord_check!(UnitMutRef::set, self, coord);
    let flat = from_mixed_radix(FlatIterRev::new(coord), FlatIterRev::new(self.shape()));
    let new_coord = to_mixed_radix(self.len(), FlatIter::new(self.shape()), flat);
    self.0.set(&new_coord, val);
    self
  }

  fn flat_mut(&mut self) -> Self::FlatMut { 
      GenReshapeMutRef(unsafe { transmute(&mut *self.0) }, vec![self.len()], self.len())
  }
  fn reshape_mut<I>(&mut self, shape : &I) -> Self::ReshapeMut
  where I : ArrayLike<Entry = usize> { 
    reshape_check!(GenReshapeMutRef::reshape_mut, self, shape);
    GenReshapeMutRef(unsafe { transmute(&mut *self.0) }, FlatIter::new(shape).map(|a| *a).collect(), self.len())
  }

  fn slice_mut<I>(&mut self, cuboid : &I) -> Self::SliceMut
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { unimplemented!() }
  fn reverse_mut(&mut self, ax : usize) -> Self::ReverseMut { unimplemented!() }
  fn fixate_mut(&mut self, ax : usize, at : isize) -> Self::FixateMut { unimplemented!() }
  fn transpose_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::TransposeMut { unimplemented!() }
  fn diagonal_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::DiagonalMut { unimplemented!() }
}
