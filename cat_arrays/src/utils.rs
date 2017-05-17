
use macros;

use traits::ArrayLike;

struct DummyArray<T>(::std::marker::PhantomData<T>);

impl<T> ArrayLike for DummyArray<T> {
  type Entry = T;
  type Shape = DummyArray<usize>;
  type Flat = DummyArray<T>;
  type Reshape = DummyArray<T>;
  type Slice = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Repeat = DummyArray<T>;
  
  fn dim(&self) -> usize { unimplemented!() }
  fn len(&self) -> usize { unimplemented!() }
  fn shape(&self) -> Self::Shape { unimplemented!() }
  fn at<I>(&self, _coordinate : I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> { unimplemented!() }

  fn flat(&self) -> Self::Flat { unimplemented!() }
  fn reshape<I>(&self, _shape : I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> { unimplemented!() }
  fn slice<I>(&self, _cuboid : I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { unimplemented!() }
  fn fixate(&self, _ax : usize, _at : isize) -> Self::Fixate { unimplemented!() }
  fn transpose(&self, _ax1 : usize, _ax2 : usize) -> Self::Transpose { unimplemented!() }
  fn diagonal(&self, _ax1 : usize, _ax2 : usize) -> Self::Diagonal { unimplemented!() }
  fn repeat(&self, _len : usize) -> Self::Repeat { unimplemented!() }
}

struct FlatIter<A, T>
where A : ArrayLike<Entry = T> {
  arr : A,
  index : usize,
}

struct FlatIterMut<A, T>
where A : ArrayLike<Entry = T> + ArrayLikeMut {
  arr : A,
  index : usize,
}

struct IntoFlatIter<A, T>
where A : ArrayLike<Entry = T> + ArrayLikeMut + IntoFlat {
  arr : <A as IntoFlat>::Flattened,
  index : usize,
}

fn compute_index(base : usize, index : isize) -> Option<usize> {
    if index < 0 && index >= -(base as isize) {
        Some((index + (base as isize)) as usize)
    } else if (index as usize) < base {
        Some(index as usize)
    } else {
        None
    }
}

struct Scalar<T> {
    pub val : T,
    dims : usize,
}

impl<T> Scalar<T> {
    fn new1(val : T) -> Self { Scalar { val, dims : 1 } }
    fn new(val : T, dims : usize) -> Self { Scalar { val, dims } }
}

struct Same<T> {
    pub val : T,
    count : usize,
}

impl<T> Same<T> {
    fn new(val : T, count : usize) -> Self { Same { val, count } }
}

impl<T> ArrayLike for Scalar<T> {
    type Entry = T;
    type Shape = Same<usize>;
    type Flat = Scalar<T>;
    type Reshape = Scalar<T>;
    type Slice = Scalar<T>;
    type Transpose = Scalar<T>;
    type Fixate = Scalar<T>;
    type Diagonal = Scalar<T>;
    type Repeat = DummyArray<T>;
    
    fn dim(&self) -> usize { self.dims }
    fn len(&self) -> usize { 1 }
    fn shape(&self) -> Self::Shape { Same { val : 1usize, count : self.dims } }

    fn at<I>(&self, coordinate : I) -> Self::Entry {
        at_coordinate_check!(coordinate, self);

    }
}

impl<T> ArrayLike for Same<T> {
    type Entry = T;
    type Shape = Scalar<usize>;
    type Flat = Same<T>;
    type Reshape = Same<T>;
    type Slice = Same<T>;
    type Transpose = Same<T>;
    type Fixate = Same<T>;
    type Diagonal = Same<T>;
    type Repeat = DummyArray<T>;

    fn dim(&self) -> usize { 1 }
    fn len(&self) -> usize { self.count }
    fn shape(&self) -> Self::Shape { Scalar { val : self.count, dims : 1 } }
}


