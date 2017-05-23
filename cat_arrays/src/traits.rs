
/// Trait for things with have Array nature and are immutable
pub trait ArrayLike {

  /// Rank of array, accessible as scalar: (♯♯A)([0])
  fn rank(&self) -> usize;

  /// Number of entries in array, accessible as scalar: (♯♭A)([0])
  fn len(&self) -> usize;

  /// Indexing of an array by coordinates in its domain: A(…)
  fn at<I>(&self, coord : &I) -> &Self::Entry
    where I : ArrayLike<Entry = isize>;
  /// The type of elements of the array
  type Entry;

  /// Shape of the array, as an array: ♯A
  fn shape(&self) -> Self::Shape;
  /// The type of the shape of the array
  type Shape : ArrayLike<Entry = usize>;

  /// The type of the flat traversal of the array
  type Flat : ArrayLike<Entry = Self::Entry>;
  /// Flat traversal of the array, as an array: ♭A
  fn flat(&self) -> Self::Flat;

  /// The type of the reshape of the array
  type Reshape : ArrayLike<Entry = Self::Entry>;
  /// Reshape operation on arrays: ϱ(A, s)
  fn reshape<I>(&self, shape : &I) -> Self::Reshape
    where I : ArrayLike<Entry = usize>;

  /// The type of the slices on the array
  type Slice : ArrayLike<Entry = Self::Entry>;
  /// Slice operation on arrays: A ∘ M
  fn slice<I>(&self, cuboid : &I) -> Self::Slice
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;

  /// The type of fixating an axis to a set value
  type Fixate : ArrayLike<Entry = Self::Entry>;
  /// Fixate operation on arrays: A ∘ Kn
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate;

  /// The type of the transpose operation
  type Transpose : ArrayLike<Entry = Self::Entry>;
  /// Transpose operation on arrays: A ∘ σ
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose;

  /// The type of the reverse operation
  type Reverse : ArrayLike<Entry = Self::Entry>;
  /// Reverse operation on arrays
  fn reverse(&self, ax : usize) -> Self::Reverse;

  /// The type of the diagonal operation
  type Diagonal : ArrayLike<Entry = Self::Entry>;
  /// Diagonal operation on arrays: diag A
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal;

  /// The type of the tiling operation
  type Tile : ArrayLike<Entry = Self::Entry>;
  /// Tile operation on arrays: tile A
  fn tile(&self, len : usize) -> Self::Tile;
}

/// Trait for array types which allow mutation of the entries within.
pub trait ArrayLikeMut : ArrayLike {

  /// Mutable indexing
  fn at_mut<I>(&mut self, coord : &I) -> &mut <Self as ArrayLike>::Entry
    where I : ArrayLike<Entry = isize>;

  /// Works the same as ArrayLike::flat
  fn flat_mut(&mut self) -> Self::FlatMut;
  /// Mutable flattened array
  type FlatMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;

  /// Works the same as ArrayLike::reshape
  fn reshape_mut<I>(&mut self, shape : &I) -> Self::ReshapeMut
    where I : ArrayLike<Entry = usize>;
  /// Mutable reshaped array
  type ReshapeMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;

  /// Mutable slice
  type SliceMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  /// Works the same as ArrayLike::slice
  fn slice_mut<I>(&mut self, cuboid : &I) -> Self::SliceMut
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;

  /// Mutable fixed index array
  type FixateMut : ArrayLike<Entry = Self::Entry> + ArrayLikeMut;
  /// Works the same as ArrayLike::fixate
  fn fixate_mut(&mut self, ax : usize, at : isize) -> Self::FixateMut;

  /// Mutable transpose
  type TransposeMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  /// Works the same as ArrayLike::transpose
  fn transpose_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::TransposeMut;

  /// Mutable reverse
  type ReverseMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  /// Works the same as ArrayLike::transpose
  fn reverse_mut(&mut self, ax : usize) -> Self::ReverseMut;
  
  /// Mutable diagonal
  type DiagonalMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
  /// Works the same as ArrayLike::diagonal
  fn diagonal_mut(&mut self, ax1 : usize, ax2 : usize) -> Self::DiagonalMut;

  /// Works the same as ArrayLike::tile
  fn tile_mut(&mut self, len : usize) -> Self::TileMut;
  /// Mutable tiling
  type TileMut : ArrayLike<Entry = <Self as ArrayLike>::Entry> + ArrayLikeMut;
}

/// For lazy arrays to create immediate copies
pub trait Force : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn force(self) -> Self::To;
}

/// For types that can into their ArrayLike::Flat type
pub trait IntoFlat : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_flat(self) -> Self::To;
}

/// For types that can into their ArrayLike::Reshape type
pub trait IntoReshape : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_reshape<I>(self, shape : &I) -> Self::To
    where I : ArrayLike<Entry = usize>;
}

/// For types that can into their ArrayLike::Slice type
pub trait IntoSlice : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_sliced<I>(self, cuboid : &I) -> Self::To
    where I : ArrayLike<Entry = (isize, Option<isize>, isize)>;
}

/// For types that can into their ArrayLike::Fixate type
pub trait IntoFixate : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_fixate(self, ax : usize, at : isize) -> Self::To;
}

/// For types that can into their ArrayLike::Transpose type
pub trait IntoTranspose : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_transpose(self, ax1 : usize, ax2 : usize) -> Self::To;
}

/// For types that can into their ArrayLike::Transpose type
pub trait IntoReverse : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_reverse(self, ax : usize) -> Self::To;
}

/// For types that can into their ArrayLike::Diagonal type
pub trait IntoDiagonal : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_diagonal(self, ax1 : usize, ax2 : usize) -> Self::To;
}

/// For types that can into their ArrayLike::Tile type
pub trait IntoTile : ArrayLike {
  type To : ArrayLike<Entry = <Self as ArrayLike>::Entry>;
  fn into_tile(self, len : usize) -> Self::To;
}
