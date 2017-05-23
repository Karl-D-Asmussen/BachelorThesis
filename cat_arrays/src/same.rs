struct Same<T> {
    pub val : T,
    count : usize,
}

impl<T> Same<T> {
    fn new(val : T, count : usize) -> Self { Same { val, count } }
}
