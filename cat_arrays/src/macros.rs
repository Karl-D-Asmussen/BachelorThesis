macro_rules! rank1_check {
  ($type:ident :: $func:ident, $coord:expr) => {
    if $coord.rank() != 1 {
      panic!("given array not rank-1 in {}::{}",
        stringify!($type), stringify!($func))
    }
  }
}

macro_rules! get_set_coord_check {
  ($type:ident :: $func:ident, $self:ident, $coord:expr) => {
    rank1_check!($type::$func, $coord);
    if $coord.len() != $self.rank() {
      panic!("mismatched dimensionality of coordinate in {}::{}",
        stringify!($type), stringify!($func))
    }
    {
      use iter::FlatIter;
      use utils::check_index;
      if ! FlatIter::new($self.shape()).zip(FlatIter::new($coord))
          .all(|a| check_index(*a.0, *a.1)) {
      panic!("coordinate out of bounds in {}::{}",
          stringify!($type), stringify!($func))
      }
    }
  }
}

macro_rules! slice_check {
  ($type:ident :: $func:ident, $self:ident, $coord:expr) => {
    rank1_check!($type::$func, $coord);
    if $coord.len() != $self.rank() {
      panic!("mismatched dimensionality of coordinate in {}::{}",
        stringify!($type), stringify!($func))
    }
    {
      use iter::FlatIter;
      use utils::check_slice;
      if ! FlatIter::new($self.shape()).zip(FlatIter::new($coord))
          .all(|a| check_slice(*a.0, a.1)) {
      panic!("coordinate out of bounds in {}::{}", stringify!($type), stringify!($func))
      }
    }
  }
}

macro_rules! non_null_shape_check {
  ($type:ident :: $func:ident, $shape:expr) => {
    rank1_check!($type :: $func, $shape);
    if FlatIter::new($shape).any(|x| *x == 0) {
        panic!("nullary shape in {}::{}",
               stringify!($type), stringify!($func))
    }
  }
}

macro_rules! reshape_check {
  ($type:ident :: $func:ident, $self:ident, $shape:expr) => {
    rank1_check!($type :: $func, $shape);
    if ! FlatIter::new($shape).fold(1, |a, b| a * b) != $self.len() {
      panic!("given shape not reshape compatible in {}::{}",
        stringify!($type), stringify!($func))
    }
  }
}

macro_rules! axis_valid_check {
  ($type:ident :: $func:ident, $self:ident, $ax:expr) => {
    if $ax >= $self.rank() {
      panic!("given axis does not exist in {}::{}",
        stringify!($type), stringify!($func))
    }
  }
}

macro_rules! fixate_check {
  ($type:ident :: $func:ident, $self:ident, $ax:expr, $at:expr) => {
    {
      use utils::check_index;
      axis_valid_check!($type::$func, $self, $ax);
      if check_index(*$self.shape().get(&Unit($ax as isize)), $at) {
        panic!("given index out of bounds in {}::{}",
          stringify!($type), stringify!($funttc))
      }
      if $self.rank() == 1 {
        panic!("would create rank-0 array {}::{}",
          stringify!($type), stringify!($funttc))
      }
    }
  }
}

macro_rules! distinct_axes_check {
  ($type:ident :: $func:ident,  $ax1:expr, $ax2:expr) => {
    if $ax1 == $ax2 {
      panic!("identical axis in {}::{}",
        stringify!($type), stringify!($funttc))
    }
  }
}

macro_rules! transpose_check {
  ($type:ident :: $func:ident, $self:ident, $ax1:expr, $ax2:expr) => {
    axis_valid_check!($type::$func, $self, $ax1);
    axis_valid_check!($type::$func, $self, $ax2);
    distinct_axes_check!($type::$func, $ax1, $ax2);
  }
}

macro_rules! diagonal_check {
  ($type:ident :: $func:ident, $self:ident, $ax1:expr, $ax2:expr) => {
    axis_valid_check!($type::$func, $self, $ax1);
    axis_valid_check!($type::$func, $self, $ax2);
    distinct_axes_check!($type::$func, $ax1, $ax2);
    if *$self.shape().get(&Unit($ax1 as isize)) != *$self.shape().get(&Unit($ax2 as isize)) {
      panic!("given axes not equal in {}::{}", 
          stringify!($type), stringify!($funttc))
    }
  }
}

macro_rules! tile_check {
  ($type:ident :: $func:ident, $len:expr) => {
    if $len == 0 {
      panic!("given length is zero in {}::{}",
          stringify!($type), stringify!($funttc))
    }
  }
}

macro_rules! S {
    ( , ) => { (0, None, 1) } ;
    ( ,; ) => { (0, None, 1) } ;
    ( ,, ) => { (0, Some(-1), 1) } ;
    ( ,,; ) => { (0, Some(-1), 1) } ;
    ($lo:expr , ) => { ($lo, None, 1) } ;
    ($lo:expr ,, ) => { ($lo, Some(-1), 1) } ;
    (, $hi:expr) => { (0, Some($hi), 1) } ;
    (,, $hi:expr) => { (0, Some($hi+1), 1) } ;
    ( ; $st:expr) => { (0, None, $st) } ;
    (, ; $st:expr) => { (0, Some(-1), $st) } ;
    ($lo:expr , $hi:expr ) => { ($lo, Some($hi), 1) } ;
    ($lo:expr ,, $hi:expr ) => { ($lo, Some($hi+1), 1) } ;
    ($lo:expr , ; $st:expr) => { ($lo, None, $st) } ;
    ($lo:expr ,, ; $st:expr) => { ($lo, Some(-1), $st) } ;
    (, $hi:expr ; $st:expr) => { (0, Some($hi), $st) } ;
    (,, $hi:expr ; $st:expr) => { (0, Some($hi+1), $st) } ;
    ($lo:expr , $hi:expr ; $st:expr) => { ($lo, Some($hi), $st) } ;
    ($lo:expr ,, $hi:expr ; $st:expr) => { ($lo, Some($hi+1), $st) } ;
}
