
use std::ops::{Deref};
use std::borrow::{Cow, ToOwned};
use std::cell::{Ref, RefMut};
use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
use std::marker::{PhantomData};

trait LT { type R : Deref<Target = ()>; }
trait LTP<'a> { type X; }

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


