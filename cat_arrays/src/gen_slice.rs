
use traits::*;
use iter::*;
use dummy::*;
use gen_reshape::*;

#[derive(Clone)]
struct Mat(Vec<Vec<isize>>, Vec<isize>);

impl Mat {
    fn new(arr : &[(isize, Option<isize>, isize)]) -> Self {
        let mut rows = Vec::with_capacity(arr.len());
        let mut cons = Vec::with_capacity(arr.len());
        for (lhs, n) in arr.iter().zip(0 ..) {
            cons.push(lhs.0);
            let mut row = Vec::with_capacity(arr.len());
            for i in 0 .. arr.len() {
                row.push( if n == i { lhs.2 } else { 0 } )
            }
            rows.push(row)
        }
        Mat(rows, cons)
    }

    fn merge(mut self, row1 : usize, row2 : usize) -> Self {
        for v in &mut self.0 {
            v[row1] += v[row2];
            v.remove(row2);
        }
        self
    }

    fn scale(mut self, shape : &[usize]) -> Self {
        for v in &mut self.0 {
            let mut mul = 1isize;
            for (a, b) in v.iter_mut().rev().zip(self.1.iter()) {
                *a *= mul;
                mul *= *b as isize;
            }
        }
        self
    }
}

#[derive(Clone)]
pub struct GenSlice<T, A : Clone>(A, Mat, Vec<usize>, usize) where A : ArrayLike<Entry = T>;

pub struct GenSliceRef<'a, T, A : 'a>(&'a A, Mat, Vec<usize>, usize) where A : ArrayLike<Entry = T>;

pub struct GenSliceMutRef<'a, T, A : 'a>(&'a mut A, Mat, Vec<usize>, usize) where A : ArrayLike<Entry = T>;

#[allow(unused_variables)]
impl<A : Clone, T : Clone> ArrayLike for GenSlice<T, A> where A : ArrayLike<Entry = T> {
  type Entry = T;
  type Shape = Vec<usize>;
  type Flat = GenReshape<T, GenSlice<T, A>>;
  type Reshape = GenReshape<T, GenSlice<T, A>>;
  type Slice = DummyArray<T>;
  type Fixate = DummyArray<T>;
  type Transpose = DummyArray<T>;
  type Reverse = DummyArray<T>;
  type Diagonal = DummyArray<T>;
  type Tile = DummyArray<T>;
  
  fn rank(&self) -> usize { self.2.len() }
  fn len(&self) -> usize { self.3 }
  fn shape(&self) -> &Self::Shape { &self.2 }

  fn get<I>(&self, coord : &I) -> &Self::Entry
  where I : ArrayLike<Entry = isize> {
    get_set_coord_check!(GenSlice::get, self, coord);
    let mut new_coord = Vec::with_capacity(self.1 .0.len());
    
    new_coord.extend(
      self.1 .0.iter()
               .map(|row|
                row.iter()
                   .zip(FlatIter::new(coord))
                   .map(|ab| ab.0 * *ab.1)
                   .fold(0, |a, b| a + b)
              ).zip(self.1 .1.iter())
               .map(|ab| ab.0 + ab.1)
    );

    self.0.get(&new_coord)
  }

  fn flat(&self) -> Self::Flat { 
    GenReshape::new(self.clone(), vec![self.len()])
  }
  fn reshape<I>(&self, shape : &I) -> Self::Reshape
  where I : ArrayLike<Entry = usize> { 
    reshape_check!(GenSlice::reshape, self, shape);
    GenReshape::new(self.clone(), FlatIter::new(shape).map(|a| *a).collect())
  }
  fn slice<I>(&self, cuboid : &I) -> Self::Slice
  where I : ArrayLike<Entry = (isize, Option<isize>, isize)> { unimplemented!() }
  fn fixate(&self, ax : usize, at : isize) -> Self::Fixate { unimplemented!() }
  fn transpose(&self, ax1 : usize, ax2 : usize) -> Self::Transpose { unimplemented!() }
  fn reverse(&self, ax1 : usize) -> Self::Reverse { unimplemented!() }
  fn diagonal(&self, ax1 : usize, ax2 : usize) -> Self::Diagonal { unimplemented!() }
  fn tile(&self, len : usize) -> Self::Tile { unimplemented!() }
}
