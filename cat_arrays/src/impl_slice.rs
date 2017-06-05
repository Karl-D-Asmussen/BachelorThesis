
use traits::*;
use dummy::DummyArray;
use unit::Unit;
use utils::compute_index;
use std::ops::{Deref, DerefMut};
use std::mem::transmute;

#[allow(unused_variables)]
impl<A, T> ArrayLike for A
    where A: Clone + Deref<Target = [T]>
{
    type Entry = T;
    type Shape = Unit<usize>;
    type Flat = Self;

    type Reshape = DummyArray<T>;
    type Slice = DummyArray<T>;
    type Fixate = DummyArray<T>;
    type Transpose = DummyArray<T>;
    type Reverse = DummyArray<T>;
    type Diagonal = DummyArray<T>;
    type Tile = DummyArray<T>;

    fn rank(&self) -> usize {
        1
    }
    fn len(&self) -> usize {
        self.deref().len()
    }
    fn shape(&self) -> &Self::Shape {
        unsafe { transmute(&self.len()) }
    }
    fn get<I>(&self, coord: &I) -> &Self::Entry
        where I: ArrayLike<Entry = isize>
    {
        get_set_coord_check!(DerefSlice::get, self, coord);
        &self[compute_index(self.len(), *coord.get(&Unit(0)))]
    }

    fn flat(&self) -> Self::Flat {
        (*self).clone()
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

#[allow(unused_variables)]
impl<A, T> ArrayLikeMut for A
    where A: Clone + DerefMut<Target = [T]>
{
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
        get_set_coord_check!(DerefSlice::get, self, coord);
        self[compute_index(self.len(), *coord.get(&Unit(0)))] = val;
        self
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
