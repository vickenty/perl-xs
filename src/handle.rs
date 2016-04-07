//! Smart wrappers for pointers managed by Perl.

use std::ops::Deref;
use raw::{ Interpreter, SV, AV, HV };

/// Marker trait for types that are reference counted by Perl
pub trait HasRefCnt {}

impl HasRefCnt for SV {}
impl HasRefCnt for AV {}
impl HasRefCnt for HV {}

pub struct Ref<T: HasRefCnt>(Interpreter, *mut T);

impl<T: HasRefCnt> Ref<T> {
    pub unsafe fn from_raw(perl: Interpreter, ptr: *mut T) -> Ref<T> {
        Ref(perl, ptr)
    }

    pub fn pthx(&self) -> Interpreter { self.0 }

    pub fn as_ptr(&self) -> *mut T { self.1 }

    pub fn incref(&self) {
        unsafe { self.0.sv_refcnt_inc(self.1 as *mut SV) }
    }

    pub fn decref(&self) {
        unsafe { self.0.sv_refcnt_dec(self.1 as *mut SV) }
    }

    pub fn to_owned(&self) -> Owned<T> {
        let svr = Ref(self.0, self.1);
        svr.incref();
        Owned(svr)
    }
}

pub struct Owned<T: HasRefCnt>(Ref<T>);

impl<T: HasRefCnt> Owned<T> {
    pub unsafe fn from_raw_owned(pthx: Interpreter, raw: *mut T) -> Owned<T> {
        Owned(Ref::from_raw(pthx, raw))
    }

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

    fn deref(&self) -> &Ref<T> {
        &self.0
    }
}
