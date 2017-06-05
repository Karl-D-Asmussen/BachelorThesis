
use traits::*;
use std::default::Default;
use std::cell::RefCell;
use unit::*;
use dummy::*;
use iter::*;
use std::mem::transmute;

pub struct Fold<'a, F, T, U, A>(&'a A, F, U, Unit<isize>, RefCell<Option<U>>, Vec<usize>, usize)
    where F: Fn(U, &'a T) -> U,
          U: Clone,
          A: ArrayLike<Entry = T> + 'a,
          T: 'a;

impl<'a, F, T, U, A> Fold<'a, F, T, U, A>
    where F: Fn(U, &'a T) -> U,
          U: Clone,
          A: ArrayLike<Entry = T>,
          T: 'a
{
    fn new(arr: &'a A, base: U, func: F, ax: usize) -> Self {
        axis_valid_check!(Fold::new, arr, ax);
        if arr.rank() == 1 {
            panic!("rank-1 array unsupported in Fold::new")
        }
        let mut shape: Vec<_> = FlatIter::new(arr.shape()).map(|a| *a).collect();
        shape.remove(ax);
        let len = shape.iter().fold(1, |a, b| a * b);
        Fold(arr,
             func,
             base,
             Unit(ax as isize),
             RefCell::new(None),
             shape,
             len)
    }
}


impl<'a, F, T: 'a, U, A: 'a> ArrayLike for Fold<'a, F, T, U, A>
    where A: ArrayLike<Entry = T>,
          F: Fn(U, &'a T) -> U,
          U: Clone,
          T: 'a
{
    type Entry = U;
    type Shape = Vec<usize>;
    type Flat = DummyArray<U>;
    type Reshape = DummyArray<U>;
    type Slice = DummyArray<U>;
    type Fixate = DummyArray<U>;
    type Transpose = DummyArray<U>;
    type Reverse = DummyArray<U>;
    type Diagonal = DummyArray<U>;
    type Tile = DummyArray<U>;

    fn rank(&self) -> usize {
        self.0.rank() - 1
    }
    fn len(&self) -> usize {
        self.6
    }
    fn shape(&self) -> &Self::Shape {
        &self.5
    }
    fn get<I>(&self, coord: &I) -> &Self::Entry
        where I: ArrayLike<Entry = isize>
    {
        let mut ix: Vec<_> = self.shape().iter().map(|a| *a as isize).collect();
        ix.insert(self.3.0 as usize, 0);
        let mut acc = self.2.clone();
        for i in 0..*self.0.shape().get(&self.3) {
            ix[self.3.0 as usize] = i as isize;
            acc = self.1(acc, self.0.get(&ix))
        }
        {
            *self.4.borrow_mut() = Some(acc)
        }
        match unsafe { transmute(&*self.4.borrow()) } {
            &Some(ref q) => return q,
            &None => unreachable!(),
        }
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
