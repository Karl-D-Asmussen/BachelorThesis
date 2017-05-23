
macro_rules! rank_check {
    ($type:ident :: $func:ident, $self:ident, $coord:ident) => {
        if $coord.rank() != 1 || $coord.len() != $self.rank() {
            panic!("{}:{}:{}: mismatched dimensionality of coordinate in {}::{}",
                   file!(), line!(), column!(), stringify!($type), stringify!($func))
        }
    }
}

macro_rules! bounds_check {
    ($type:ident :: $func:ident, $self:ident, $coord:ident) => {

    }
}

