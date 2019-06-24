//! Smart wrappers for pointers managed by Perl.

use crate::raw::{Interpreter, AV, HV, SV};
use std::ops::Deref;

/// Marker trait for types that are reference counted by Perl
pub trait HasRefCnt {}

impl HasRefCnt for SV {}
impl HasRefCnt for AV {}
impl HasRefCnt for HV {}

pub struct Ref<T: HasRefCnt>(Interpreter, *mut T);

impl<T: HasRefCnt> Ref<T> {
    #[inline]
    pub unsafe fn from_raw(perl: Interpreter, ptr: *mut T) -> Ref<T> {
        Ref(perl, ptr)
    }

    #[inline]
    pub fn pthx(&self) -> Interpreter {
        self.0
    }

    #[inline]
    pub fn as_ptr(&self) -> *mut T {
        self.1
    }

    #[inline]
    pub fn incref(&self) {
        unsafe { self.0.ouroboros_sv_refcnt_inc_void_nn(self.1 as *mut SV) }
    }

    #[inline]
    pub fn decref(&self) {
        unsafe { self.0.ouroboros_sv_refcnt_dec_nn(self.1 as *mut SV) }
    }

    #[inline]
    pub fn to_owned(&self) -> Owned<T> {
        let svr = Ref(self.0, self.1);
        svr.incref();
        Owned(svr)
    }
}

pub struct Owned<T: HasRefCnt>(Ref<T>);

impl<T: HasRefCnt> Owned<T> {
    #[inline]
    pub unsafe fn from_raw_owned(pthx: Interpreter, raw: *mut T) -> Owned<T> {
        Owned(Ref::from_raw(pthx, raw))
    }

    #[inline]
    pub unsafe fn from_raw_borrowed(pthx: Interpreter, raw: *mut T) -> Owned<T> {
        Ref::from_raw(pthx, raw).to_owned()
    }
}

impl<T: HasRefCnt> Drop for Owned<T> {
    fn drop(&mut self) {
        self.0.decref();
    }
}

impl<T: HasRefCnt> Deref for Owned<T> {
    type Target = Ref<T>;

    #[inline]
    fn deref(&self) -> &Ref<T> {
        &self.0
    }
}
