
use std::ops::*;
use ltp::*;

trait Array {
    type Element;

    type Shape : Array<Element = usize>;
    type Flat : Array<Element = Self::Element>;

    fn rank(&self) -> usize;
    fn len(&self) -> usize;
}

trait ArrayRef {
    type Owned : Array;

    type Entry : LTP<'static> + Deref<Target = <Self::Owned as Array>::Element>;
    fn at_ref<'a, IR, I>(&'a self, coord : I) -> <Self::Entry as LTP<'static>>::X
        where Self::Entry : LT<R = &'a ()>,
              IR : ArrayRef<Owned = I>,
              I : Array<Element = isize>;
    
    type ShapeRef : LTP<'static> + ArrayRef;
    fn shape<'a>(&'a self) -> <Self::ShapeRef as LTP<'static>>::X
        where Self::ShapeRef : LT<R = &'a ()>,
              <Self::ShapeRef as ArrayRef>::Owned : Array<Element = usize>;

    type FlatRef : LTP<'static> + ArrayRef;
    fn flat<'a>(&'a self) -> <Self::FlatRef as LTP<'static>>::X
        where Self::FlatRef : LT<R = &'a ()>,
              <Self::FlatRef as ArrayRef>::Owned : Array<Element = <Self::Owned as Array>::Element>;
}

trait ArrayMut : ArrayRef {
    type EntryMut : LTP<'static> + Deref<Target = <Self::Owned as Array>::Element>;
    fn at_mut<'a, IR, I>(&'a self, coord : I) -> <Self::EntryMut as LTP<'static>>::X
        where Self::Entry : LT<R = &'a ()>,
              IR : ArrayRef<Owned = I>,
              I : Array<Element = isize>;
}


trait ArrayInto : Array {
}

struct Ston(usize);
impl Array for Ston {
    type Element = usize;
    type Shape = Ston;
    type Flat = Ston;

    fn rank(&self) -> usize { 1 }
    fn len(&self) -> usize { 1 }
}

impl<'a> ArrayRef for It<'a, Ston> {
    type Entry = It<'static, usize>;
}
