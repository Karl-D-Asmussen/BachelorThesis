
macro_rules! vector_check {
  ($type:ident :: $func:ident, $self:ident, $coord:ident) => {
    if $coord.rank() != 1 {
      panic!("{}:{}:{}: given array not rank-1 in {}::{}",
        file!(), line!(), column!(), stringify!($type), stringify!($func))
    }
  }
}

macro_rules! rank_check {
  ($type:ident :: $func:ident, $self:ident, $coord:ident) => {
    if $coord.len() != $self.rank() {
      panic!("{}:{}:{}: mismatched dimensionality of coordinate in {}::{}",
        file!(), line!(), column!(), stringify!($type), stringify!($func))
    }
  }
}

macro_rules! bounds_check {
  ($type:ident :: $func:ident, $self:ident, $coord:ident) => {
    {
        use iter::FlatIter;
        use utils::check_index;
        if ! FlatIter::new(&$self.shape()).zip(FlatIter::new($coord))
           .all(|a| check_index(*a.0, *a.1)) {
        panic!("{}:{}:{}: coordinate out of bounds in {}::{}",
            file!(), line!(), column!(), stringify!($type), stringify!($func))
        }
    }
  }
}

macro_rules! coord_check {
  ($type:ident :: $func:ident, $self:ident, $coord:ident) => {
    vector_check!($type :: $func, $self, $coord);
    rank_check!($type :: $func, $self, $coord);
    bounds_check!($type :: $func, $self, $coord);
  }
}

macro_rules! len_check {
  ($type:ident :: $func:ident, $self:ident, $shape:ident) => {
    if ! FlatIter::new(&$shape).fold(1, |a, b| a * b) != self.len() {
      panic!("{}:{}:{}: given shape not reshape compatible in {}::{}",
        file!(), line!(), column!(), stringify!($type), stringify!($func))
    }
  }
}
