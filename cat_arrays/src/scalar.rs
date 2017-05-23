pub struct Scalar<T> {
    pub val : T,
    rank : usize,
}

impl<T> Scalar<T> {
    fn new1(val : T) -> Self { Scalar { val, rank : 1 } }
    fn new(val : T, rank : usize) -> Self { Scalar { val, rank } }
}
