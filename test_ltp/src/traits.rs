
use std::ops::*;
use ltp::*;

trait Array {
    type Element;
    fn rank(&self) -> usize;
    fn len(&self) -> usize;
}

trait ArrayRef<'a> : Array + LTP<'b> {
    type Entry : LTP<'static> + Deref<Target = Self::Element>;
    fn at_ref<'a, I>(&'a self, coord : I) -> <Self::Entry as LTP<'static>>::X
        where Self::Entry : LT<R = &'a ()>,
              I : Array<Element = usize>;

    type Shape : ArrayRef<'b> + <Element = usize>;
    fn shape<'a>(&'a self) -> <Self::Shape as LTP<'static>>::X where Self::Shape : LT<R = &'a ()>;

    type Flat : ArrayRef<'b> + Array<Element = Self::Element>;
    fn flat<'a>(&'a self) -> <Self::Flat as LTP<'static>>::X where Self::Flat : LT<R = &'a ()>;
}
