
use std::ops::{Deref, DerefMut};
use std::borrow::{Cow, ToOwned};
use std::cell::{Ref, RefMut};
use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
use std::marker::{PhantomData};
use std::convert::{From};

pub trait LT { type R : Deref<Target = ()>; }
pub trait LTP<'a> { type X; }

impl<'a, T> LT for &'a T { type R = &'a (); }
impl<'a, T> LT for &'a mut T { type R = &'a (); }
impl<'a, T> LT for &'a [T] { type R = &'a (); }
impl<'a, T> LT for &'a mut [T] { type R = &'a (); }

impl<'a, T : ToOwned> LT for Cow<'a, T> { type R = &'a (); }
impl<'a, T> LT for MutexGuard<'a, T> { type R = &'a (); }
impl<'a, T> LT for RwLockReadGuard<'a, T> { type R = &'a (); }
impl<'a, T> LT for RwLockWriteGuard<'a, T> { type R = &'a (); }
impl<'a, T> LT for Ref<'a, T> { type R = &'a (); }
impl<'a, T> LT for RefMut<'a, T> { type R = &'a (); }

impl<'a, 'b, T : 'a> LTP<'b> for &'b T where Self : LT<R = &'a()> { type X = &'a T; }
impl<'a, 'b, T : 'a> LTP<'b> for &'b mut T where Self : LT<R = &'a()> { type X = &'a mut T; }
impl<'a, 'b, T : 'a> LTP<'b> for &'b [T] where Self : LT<R = &'a()> { type X = &'a [T]; }
impl<'a, 'b, T : 'a> LTP<'b> for &'b mut [T] where Self : LT<R = &'a()> { type X = &'a mut [T]; }

impl<'a, 'b, T : 'a + ToOwned> LTP<'b> for Cow<'b, T> where Self : LT<R = &'a()> { type X = Cow<'a, T>; }
impl<'a, 'b, T : 'a> LTP<'b> for MutexGuard<'b, T> where Self : LT<R = &'a()> { type X = MutexGuard<'a, T>; }
impl<'a, 'b, T : 'a> LTP<'b> for RwLockReadGuard<'b, T> where Self : LT<R = &'a()> { type X = RwLockReadGuard<'a, T>; }
impl<'a, 'b, T : 'a> LTP<'b> for RwLockWriteGuard<'b, T> where Self : LT<R = &'a()> { type X = RwLockWriteGuard<'a, T>; }
impl<'a, 'b, T : 'a> LTP<'b> for Ref<'b, T> where Self : LT<R = &'a()> { type X = Ref<'a, T>; }
impl<'a, 'b, T : 'a> LTP<'b> for RefMut<'b, T> where Self : LT<R = &'a()> { type X = RefMut<'a, T>; }

pub struct It<'a, T> {
    _ph : PhantomData<&'a ()>,
    pub it : T
}

impl<'a, T> LT for It<'a, T> { type R = &'a (); }
impl<'a, 'b, T> LTP<'b> for It<'b, T> where Self : LT<R = &'a ()> { type X = It<'a, T>; }

impl<'a, T> It<'a, T> {
    pub fn new(it : T) -> Self { It { it, _ph : PhantomData } }
    pub fn into_inner(self) -> T { self.it }
}

impl<'a, T> From<T> for It<'a, T> {
    fn from(it : T) -> Self { Self::new(it) }
}

impl<'a, T> AsRef<T> for It<'a, T> {
    fn as_ref(&self) -> &T { &self.it }
}

impl<'a, T> AsMut<T> for It<'a, T> {
    fn as_mut(&mut self) -> &mut T { &mut self.it }
}

impl<'a, T> Deref for It<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.it }
}

impl<'a, T> DerefMut for It<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.it }
}
