macro_rules! rank1_check {
  ($type:ident :: $func:ident, $coord:expr) => {
    if $coord.rank() != 1 {
      panic!("{}:{}:{}: given array not rank-1 in {}::{}",
        file!(), line!(), column!(), stringify!($type), stringify!($func))
    }
  }
}

macro_rules! get_set_coord_check {
  ($type:ident :: $func:ident, $self:ident, $coord:expr) => {
    rank1_check!($type::$func, $coord);
    if $coord.len() != $self.rank() {
      panic!("{}:{}:{}: mismatched dimensionality of coordinate in {}::{}",
        file!(), line!(), column!(), stringify!($type), stringify!($func))
    }
    {
      use iter::FlatIter;
      use utils::check_index;
      if ! FlatIter::new($self.shape()).zip(FlatIter::new($coord))
          .all(|a| check_index(*a.0, *a.1)) {
      panic!("{}:{}:{}: coordinate out of bounds in {}::{}",
          file!(), line!(), column!(), stringify!($type), stringify!($func))
      }
    }
  }
}

macro_rules! non_null_shape_check {
  ($type:ident :: $func:ident, $shape:expr) => {
    rank1_check!($type :: $func, $shape);
    if FlatIter::new($shape).any(|x| x == 0) {
        panic!("{}:{}:{}: nullary shape in {}::{}",
               file!(), line!(), column!(), stringify!($type), stringify!($func))
    }
  }
}

macro_rules! reshape_check {
  ($type:ident :: $func:ident, $self:ident, $shape:expr) => {
    rank1_check!($type :: $func, $shape);
    if ! FlatIter::new(&$shape).fold(1, |a, b| a * b) != self.len() {
      panic!("{}:{}:{}: given shape not reshape compatible in {}::{}",
        file!(), line!(), column!(), stringify!($type), stringify!($func))
    }
  }
}
