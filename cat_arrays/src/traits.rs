
pub trait ArrayLike {
  type Entry;
  type Shape : ArrayLike<Entry = usize>;
  type Flat : ArrayLike<Entry = Self::Entry>;
  type Reshape : ArrayLike<Entry = Self::Entry>;
  type Slice : ArrayLike<Entry = Self::Entry>;
  type Fixate : ArrayLike<Entry = Self::Entry>;
  type Transpose : ArrayLike<Entry = Self::Entry>;
  type Diagonal : ArrayLike<Entry = Self::Entry>;
  type Repeat : ArrayLike<Entry = Self::Entry>;

  fn dim(&self) -> usize;
  fn len(&self) -> usize;
  fn shape(&self) -> Self::Shape;
  fn at<I>(&self, coordinate : I) -> &Self::Entry
    where I : ArrayLike<Entry = isize>;

  fn flat(&self) -> Self::Flat;

  fn reshape<I>(&self, shape : I) -> Self::Reshape
    where I : ArrayLike<Entry = usize>;
  fn slice<I>(&self, cuboid : I) -> Self::Slice
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate;
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose;
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal;
  fn repeat(&self, len : usize) -> Self::Repeat;
}

pub trait ArrayLikeMut : ArrayLike {
  type FlatMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  type ReshapeMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  type SliceMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  type FixateMut : ArrayLike<Entry = Self::Entry> + ArrayLikeMut;
  type TransposeMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  type DiagonalMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  type RepeatMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;

  fn at_mut<I>(&mut self, coord : I) -> &mut <Self as ArrayLike>::Entry
    where I : ArrayLike<Entry = isize>;
  
  fn flat_mut(&mut self) -> Self::FlatMut;

  fn reshape_mut<I>(&mut self, shape : I) -> Self::ReshapeMut
    where I : ArrayLike<Entry = usize>;
  fn slice_mut<I>(&mut self, cuboid : I) -> Self::SliceMut
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;
  fn fixate(&mut self, ax : usize, at : isize) -> Self::FixateMut;
  fn transpose_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::TransposeMut;
  fn diagonal_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::DiagonalMut;
  fn repeat_mut(&mut self, len : usize) -> Self::RepeatMut;
}

pub trait IntoShape : ArrayLike {
  type Shaped : ArrayLike<Entry = usize>;
  fn into_shape(self) -> Self::Shaped;
}

pub trait IntoFlat : ArrayLike {
  type Flattened : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_flat(self) -> Self::Flattened;
}

pub trait IntoReshape : ArrayLike {
  type Reshaped : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_reshape<I>(self, shape : I) -> Self::Reshaped
    where I : ArrayLike<Entry = usize>;
}

pub trait IntoSlice : ArrayLike {
  type Sliced : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_sliced<I>(self, cuboid : I) -> Self::Sliced
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;
}

pub trait IntoFixate : ArrayLike {
  type Fixated : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_fixate(self, ax : usize, at : isize) -> Self::Fixated;
}

pub trait IntoTranspose : ArrayLike {
  type Transposed : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_transpose(self, ax1 : usize, ax2 : usize) -> Self::Transposed;
}

pub trait IntoDiagonal : ArrayLike {
  type Diagnoalized : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_diagonal(self, ax1 : usize, ax2 : usize) -> Self::Diagnoalized;
}

pub trait IntoRepeat : ArrayLike {
  type Repeating : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_repeat(self, len : usize) -> Self::Repeating;
}
