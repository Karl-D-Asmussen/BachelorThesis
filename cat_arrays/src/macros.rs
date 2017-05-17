
macro_rules! at_coordinate_check {
    ($coordinate:ident, $self:ident) => {
        if $coordinate.dim() != 1 || $coordinate.len() != $self.dim() {
            panic!(":{}:{}:{}: Mismatched dimensionality of coordinate", file!(), line!(), column!())
        }
    }
}

