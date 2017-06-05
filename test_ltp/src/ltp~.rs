
use std::ops::{Deref, DerefMut};
use std::borrow::{Cow, ToOwned};
use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
use std::marker::PhantomData;

struct UR<'a, T>(pub T, PhantomData<&'a ()>);
impl<'a, T> UR<'a, T> { fn new(t: T) -> UR<'a, T> { UR(t, PhantomData) } }
impl<'a, T> AsRef<T> for UR<'a, T> { fn as_ref(&self) -> &T { &self.0 } }
impl<'a, T> Deref for UR<'a, T> { type Target = T; fn deref(&self) -> &Self::Target { &self.0 } }
impl<'a, T> DerefMut for UR<'a, T> { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 } }

pub trait LTP {
    type _Ref : Deref<Target = ()>;
}

impl<'a, T : 'a> LTP for &'a T { type _Ref = &'a (); }
impl<'a, T : 'a> LTP for &'a [T] { type _Ref = &'a (); }
impl<'a, T : 'a> LTP for &'a mut T { type _Ref = &'a (); }
impl<'a, T : 'a> LTP for &'a mut [T] { type _Ref = &'a (); }
impl<'a, T : 'a  + ToOwned> LTP for Cow<'a, T> { type _Ref = &'a (); }
impl<'a, T : 'a> LTP for MutexGuard<'a, T> { type _Ref = &'a (); }
impl<'a, T : 'a> LTP for RwLockReadGuard<'a, T> { type _Ref = &'a (); }
impl<'a, T : 'a> LTP for RwLockWriteGuard<'a, T> { type _Ref = &'a (); }
impl<'a, T> LTP for UR<'a, T> { type _Ref = &'a (); }

pub trait LTPG<'a> : LTP {
    type Current : LTP;
    type Replaced : LTP;
}

impl<'a, 'b, T : 'a + 'b> LTPG<'a> for &'b T where Self : LTP<_Ref = &'b ()> {
    type Current = &'a T;
    type Replaced = &'b T;
}

impl<'a, 'b, T : 'a + 'b> LTPG<'a> for &'b T where Self : LTP<_Ref = &'b ()> {
    type Current = &'a T;
    type Replaced = &'b T;
}

impl<'a, 'b, T : 'a + 'b> LTPG<'a> for &'b mut T where Self : LTP<_Ref = &'b ()> {
    type Current = &'a mut T;
    type Replaced = &'b mut T;
}

impl<'a, 'b, T : 'a + 'b  + ToOwned> LTPG<'a> for Cow<'b, T> where Self : LTP<_Ref = &'b ()> {
    type Current = Cow<'a, T>;
    type Replaced = Cow<'b, T>;
}

impl<'a, 'b, T : 'a + 'b> LTPG<'a> for MutexGuard<'b, T> where Self : LTP<_Ref = &'b ()> {
    type Current = MutexGuard<'a, T>;
    type Replaced = MutexGuard<'b, T>;
}

impl<'a, 'b, T : 'a + 'b> LTPG<'a> for RwLockReadGuard<'b, T> where Self : LTP<_Ref = &'b ()> {
    type Current = RwLockReadGuard<'a, T>;
    type Replaced = RwLockReadGuard<'b, T>;
}

impl<'a, 'b, T : 'a + 'b> LTPG<'a> for RwLockWriteGuard<'b, T> where Self : LTP<_Ref = &'b ()> {
    type Current = RwLockWriteGuard<'a, T>;
    type Replaced = RwLockWriteGuard<'b, T>;
}

impl<'a, 'b, T> LTPG<'a> for UR<'b, T> where Self : LTP<_Ref = &'b ()> {
    type Current = UR<'a, T>;
    type Replaced = UR<'b, T>;
}
