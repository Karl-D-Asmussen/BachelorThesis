
#![allow(unused_variables)]

use traits::*;
use std::marker::PhantomData;

pub struct DummyArray<T>(pub PhantomData<T>);

impl<T> DummyArray<T> {
    #[allow(dead_code)]
    fn new() -> Self {
        DummyArray(PhantomData)
    }
}

impl<T> ArrayLike for DummyArray<T> {
    type Entry = T;
    type Shape = DummyArray<usize>;
    type Flat = DummyArray<T>;
    type Reshape = DummyArray<T>;
    type Slice = DummyArray<T>;
    type Fixate = DummyArray<T>;
    type Transpose = DummyArray<T>;
    type Reverse = DummyArray<T>;
    type Diagonal = DummyArray<T>;
    type Tile = DummyArray<T>;

    fn rank(&self) -> usize {
        unimplemented!()
    }
    fn len(&self) -> usize {
        unimplemented!()
    }
    fn shape(&self) -> &Self::Shape {
        unimplemented!()
    }
    fn get<I>(&self, coord: &I) -> &Self::Entry
        where I: ArrayLike<Entry = isize>
    {
        unimplemented!()
    }

    fn flat(&self) -> Self::Flat {
        unimplemented!()
    }
    fn reshape<I>(&self, shape: &I) -> Self::Reshape
        where I: ArrayLike<Entry = usize>
    {
        unimplemented!()
    }
    fn slice<I>(&self, cuboid: &I) -> Self::Slice
        where I: ArrayLike<Entry = (isize, Option<isize>, isize)>
    {
        unimplemented!()
    }
    fn fixate(&self, ax: usize, at: isize) -> Self::Fixate {
        unimplemented!()
    }
    fn transpose(&self, ax1: usize, ax2: usize) -> Self::Transpose {
        unimplemented!()
    }
    fn reverse(&self, ax1: usize) -> Self::Reverse {
        unimplemented!()
    }
    fn diagonal(&self, ax1: usize, ax2: usize) -> Self::Diagonal {
        unimplemented!()
    }
    fn tile(&self, len: usize) -> Self::Tile {
        unimplemented!()
    }
}

impl<T> ArrayLikeMut for DummyArray<T> {
    type FlatMut = DummyArray<T>;
    type ReshapeMut = DummyArray<T>;
    type SliceMut = DummyArray<T>;
    type FixateMut = DummyArray<T>;
    type TransposeMut = DummyArray<T>;
    type ReverseMut = DummyArray<T>;
    type DiagonalMut = DummyArray<T>;

    fn set<I>(&mut self, coord: &I, val: <Self as ArrayLike>::Entry) -> &mut Self
        where I: ArrayLike<Entry = isize>
    {
        unimplemented!()
    }

    fn flat_mut(&mut self) -> Self::FlatMut {
        unimplemented!()
    }
    fn reshape_mut<I>(&mut self, shape: &I) -> Self::ReshapeMut
        where I: ArrayLike<Entry = usize>
    {
        unimplemented!()
    }
    fn slice_mut<I>(&mut self, cuboid: &I) -> Self::SliceMut
        where I: ArrayLike<Entry = (isize, Option<isize>, isize)>
    {
        unimplemented!()
    }
    fn fixate_mut(&mut self, ax: usize, at: isize) -> Self::FixateMut {
        unimplemented!()
    }
    fn transpose_mut(&mut self, ax1: usize, ax2: usize) -> Self::TransposeMut {
        unimplemented!()
    }
    fn reverse_mut(&mut self, ax1: usize) -> Self::ReverseMut {
        unimplemented!()
    }
    fn diagonal_mut(&mut self, ax1: usize, ax2: usize) -> Self::DiagonalMut {
        unimplemented!()
    }
}

impl<T> IntoFlat for DummyArray<T> {
    type To = DummyArray<T>;
    fn into_flat(self) -> Self::To {
        unimplemented!()
    }
}

impl<T> IntoReshape for DummyArray<T> {
    type To = DummyArray<T>;
    fn into_reshape<I>(self, shape: &I) -> Self::To
        where I: ArrayLike<Entry = usize>
    {
        unimplemented!()
    }
}

impl<T> IntoSlice for DummyArray<T> {
    type To = DummyArray<T>;
    fn into_slice<I>(self, cuboid: &I) -> Self::To
        where I: ArrayLike<Entry = (isize, Option<isize>, isize)>
    {
        unimplemented!()
    }
}

impl<T> IntoFixate for DummyArray<T> {
    type To = DummyArray<T>;
    fn into_fixate(self, ax: usize, at: isize) -> Self::To {
        unimplemented!()
    }
}

impl<T> IntoTranspose for DummyArray<T> {
    type To = DummyArray<T>;
    fn into_transpose(self, ax1: usize, ax2: usize) -> Self::To {
        unimplemented!()
    }
}

impl<T> IntoReverse for DummyArray<T> {
    type To = DummyArray<T>;
    fn into_reverse(self, ax: usize) -> Self::To {
        unimplemented!()
    }
}

impl<T> IntoDiagonal for DummyArray<T> {
    type To = DummyArray<T>;
    fn into_diagonal(self, ax1: usize, ax2: usize) -> Self::To {
        unimplemented!()
    }
}

impl<T> IntoTile for DummyArray<T> {
    type To = DummyArray<T>;
    fn into_tile(self, len: usize) -> Self::To {
        unimplemented!()
    }
}
