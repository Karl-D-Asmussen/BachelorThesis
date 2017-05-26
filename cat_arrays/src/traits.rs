
pub trait ArrayLike {

  fn rank(&self) -> usize;

  fn len(&self) -> usize;

  type Entry;
  fn get<I>(&self, coord : &I) -> &Self::Entry
    where I : ArrayLike<Entry = isize>;

  type Shape : ArrayLike<Entry = usize>;
  fn shape(&self) -> &Self::Shape;

  type Flat : ArrayLike<Entry = Self::Entry>;
  fn flat(&self) -> Self::Flat;

  type Reshape : ArrayLike<Entry = Self::Entry>;
  fn reshape<I>(&self, shape : &I) -> Self::Reshape
    where I : ArrayLike<Entry = usize>;

  type Slice : ArrayLike<Entry = Self::Entry>;
  fn slice<I>(&self, cuboid : &I) -> Self::Slice
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;

  type Fixate : ArrayLike<Entry = Self::Entry>;
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate;

  type Transpose : ArrayLike<Entry = Self::Entry>;
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose;

  type Reverse : ArrayLike<Entry = Self::Entry>;
  fn reverse(&self, ax : usize) -> Self::Reverse;

  type Diagonal : ArrayLike<Entry = Self::Entry>;
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal;

  type Tile : ArrayLike<Entry = Self::Entry>;
  fn tile(&self, len : usize) -> Self::Tile;
}

pub trait ArrayLikeMut : ArrayLike {

  fn set<I>(&mut self, coord : &I, val : <Self as ArrayLike>::Entry) -> &mut Self
    where I : ArrayLike<Entry = isize>;

  type FlatMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  fn flat_mut(&mut self) -> Self::FlatMut;

  type ReshapeMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  fn reshape_mut<I>(&mut self, shape : &I) -> Self::ReshapeMut
    where I : ArrayLike<Entry = usize>;

  type SliceMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  fn slice_mut<I>(&mut self, cuboid : &I) -> Self::SliceMut
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;

  type FixateMut : ArrayLike<Entry = Self::Entry> + ArrayLikeMut;
  fn fixate_mut(&mut self, ax : usize, at : isize) -> Self::FixateMut;

  type TransposeMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  fn transpose_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::TransposeMut;

  type ReverseMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  fn reverse_mut(&mut self, ax : usize) -> Self::ReverseMut;
  
  type DiagonalMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  fn diagonal_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::DiagonalMut;
}

pub trait IntoFlat : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_flat(self) -> Self::To;
}

pub trait IntoReshape : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_reshape<I>(self, shape : &I) -> Self::To
    where I : ArrayLike<Entry = usize>;
}

pub trait IntoSlice : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_sliced<I>(self, cuboid : &I) -> Self::To
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;
}

pub trait IntoFixate : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_fixate(self, ax : usize, at : isize) -> Self::To;
}

pub trait IntoTranspose : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_transpose(self, ax1 : usize, ax2 : usize) -> Self::To;
}

pub trait IntoReverse : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_reverse(self, ax : usize) -> Self::To;
}

pub trait IntoDiagonal : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_diagonal(self, ax1 : usize, ax2 : usize) -> Self::To;
}

pub trait IntoTile : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_tile(self, len : usize) -> Self::To;
}
