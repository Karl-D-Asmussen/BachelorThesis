
use ltp::*;

trait ArrayLike {
    type Element; 
    type Entry : Deref<Target = Self::Element> + LTPG<'static>;
    type ShapeDesc : ArrayLike<Element = usize> + LTPG<'static>;

    fn at_ref(&self) -> Entry;
    fn shape(&self) -> 
}

