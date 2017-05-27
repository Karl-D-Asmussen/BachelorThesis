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
    }
  }
}

macro_rules! transpose_check {
  ($type:ident :: $func:ident, $self:ident, $ax1:expr, $ax2:expr) => {
    axis_valid_check!($type::$func, $self, $ax1);
    axis_valid_check!($type::$func, $self, $ax2);
  }
}

macro_rules! diagonal_check {
  ($type:ident :: $func:ident, $self:ident, $ax1:expr, $ax2:expr) => {
    axis_valid_check!($type::$func, $self, $ax1);
    axis_valid_check!($type::$func, $self, $ax2);
    if *$self.shape().get(&Unit($ax1 as isize)) != *$self.shape().get(&Unit($ax2 as isize)) {
      panic!("given axes not equal in {}::{}", 
          stringify!($type), stringify!($funttc))
    }
  }
}
