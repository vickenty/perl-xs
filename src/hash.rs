use std::marker::PhantomData;
use std::mem;
use std::slice::from_raw_parts;

use crate::SV;
use crate::convert::{FromSV, TryFromSV};
use crate::handle::Owned;
use crate::raw;

/// Perl hash object.
pub struct HV(Owned<raw::HV>);

impl HV {
    fn pthx(&self) -> raw::Interpreter {
        self.0.pthx()
    }
    fn as_ptr(&self) -> *mut raw::HV {
        self.0.as_ptr()
    }

    method! {
        /// Frees the all the elements of a hash, leaving it empty.
        ///
        /// See [`hv_clear`](http://perldoc.perl.org/perlapi.html#hv_clear).
        simple fn clear() = hv_clear()
    }

    method! {
        /// Clears any placeholders from a hash.
        ///
        /// See [`hv_clear_placeholders`](http://perldoc.perl.org/perlapi.html#hv_clear_placeholders).
        simple fn clear_placeholders() = hv_clear_placeholders()
    }

    method! {
        /// Deletes a key/value pair in the hash. The value's SV is removed from the hash, made
        /// mortal, and returned to the caller.
        ///
        /// See [`hv_delete`](http://perldoc.perl.org/perlapi.html#hv_delete).
        getter fn delete(key: &str) = hv_delete(key.as_ptr() as *const _, -(key.len() as raw::I32), 0)
    }

    method! {
        /// Deletes a key/value pair in the hash. The value's SV is removed from the hash, made
        /// mortal, and returned to the caller.
        ///
        /// See [`hv_delete`](http://perldoc.perl.org/perlapi.html#hv_delete).
        simple fn discard(key: &str) = hv_delete(key.as_ptr() as *const _, -(key.len() as raw::I32), raw::G_DISCARD as raw::I32)
    }

    method! {
        /// Returns a boolean indicating whether the specified hash key
        /// exists.
        ///
        /// See [`hv_exists`](http://perldoc.perl.org/perlapi.html#hv_exists).
        simple fn exists(key: &str) -> bool = hv_exists(key.as_ptr() as *const _, -(key.len() as raw::I32)) != 0
    }

    method! {
        /// Returns the SV which corresponds to the specified key in the hash.
        ///
        /// See [`hv_fetch`](http://perldoc.perl.org/perlapi.html#hv_fetch).
        getptr fn fetch(key: &str) = hv_fetch(key.as_ptr() as *const _, -(key.len() as raw::I32), 0)
    }

    method! {
        /// Returns the SV which corresponds to the specified key in the hash. Stores new SV in the
        /// hash and returns it if key was not part of the hash.
        ///
        /// See [`hv_fetch`](http://perldoc.perl.org/perlapi.html#hv_fetch).
        getptr fn fetch_lvalue(key: &str) = hv_fetch(key.as_ptr() as *const _, -(key.len() as raw::I32), 1)
    }

    /// Stores an SV in a hash.
    ///
    /// See [`hv_store`](http://perldoc.perl.org/perlapi.html#hv_store).
    #[inline]
    pub fn store(&self, key: &str, val: SV) {
        unsafe {
            let raw = val.into_raw();
            let svpp = self.pthx().hv_store(
                self.as_ptr(),
                key.as_ptr() as *const _,
                -(key.len() as raw::I32),
                raw,
                0,
            );
            if svpp.is_null() {
                self.pthx().ouroboros_sv_refcnt_dec(raw)
            }
        }
    }

    /// Construct new HV from a raw pointer without incrementing reference counter (raw pointer
    /// already "owns" one incref).
    #[inline]
    pub unsafe fn from_raw_owned(pthx: raw::Interpreter, raw: *mut raw::HV) -> HV {
        HV(Owned::from_raw_owned(pthx, raw))
    }

    /// Construct new HV from a raw pointer and increment its reference counter (raw pointer is
    /// "borrowed" from another structure that owns one incref).
    #[inline]
    pub unsafe fn from_raw_borrowed(pthx: raw::Interpreter, raw: *mut raw::HV) -> HV {
        HV(Owned::from_raw_borrowed(pthx, raw))
    }

    /// Get an iterator over the hash.
    #[inline]
    pub fn iter<T: FromSV>(&self) -> Iter<T> {
        Iter::new(self)
    }

    /// Get an iterator over the hash values.
    #[inline]
    pub fn values<T: FromSV>(&self) -> Values<T> {
        Values::new(self)
    }

    /// Get an iterator over the hash keys.
    #[inline]
    pub fn keys(&self) -> Keys {
        Keys::new(self)
    }
}

impl TryFromSV for HV {
    type Error = &'static str;

    unsafe fn try_from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> Result<HV, Self::Error> {
        if pthx.ouroboros_sv_rok(raw) == 0 {
            return Err("not a hash reference");
        }

        Ok(HV::from_raw_borrowed(
            pthx,
            pthx.ouroboros_sv_rv(raw) as *mut _,
        ))
    }
}

pub struct Iter<'a, T> {
    hv: &'a HV,
    ty: PhantomData<T>,
}

impl<'a, T> Iter<'a, T> {
    fn new(hv: &'a HV) -> Self {
        unsafe { hv.pthx().hv_iterinit(hv.as_ptr()) };
        Iter {
            hv: hv,
            ty: PhantomData,
        }
    }
}

impl<'a, T: FromSV> Iterator for Iter<'a, T> {
    type Item = (&'a [u8], T);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let pthx = self.hv.pthx();
            let hv_ptr = self.hv.as_ptr();

            let he = pthx.hv_iternext(hv_ptr);
            if he.is_null() {
                None
            } else {
                let mut klen: raw::I32 = mem::uninitialized();
                let k_ptr = pthx.hv_iterkey(he, &mut klen as *mut _) as *const u8;
                let k = from_raw_parts(k_ptr, klen as usize);
                let v = pthx.hv_iterval(hv_ptr, he);
                Some((k, T::from_sv(pthx, v)))
            }
        }
    }
}

pub struct Values<'a, T> {
    hv: &'a HV,
    ty: PhantomData<T>,
}

impl<'a, T> Values<'a, T> {
    fn new(hv: &'a HV) -> Self {
        unsafe { hv.pthx().hv_iterinit(hv.as_ptr()) };
        Values {
            hv: hv,
            ty: PhantomData,
        }
    }
}

impl<'a, T: FromSV> Iterator for Values<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let pthx = self.hv.pthx();
            let hv_ptr = self.hv.as_ptr();

            let he = pthx.hv_iternext(hv_ptr);
            if he.is_null() {
                None
            } else {
                let v = pthx.hv_iterval(hv_ptr, he);
                Some(T::from_sv(pthx, v))
            }
        }
    }
}

pub struct Keys<'a>(&'a HV);

impl<'a> Keys<'a> {
    fn new(hv: &'a HV) -> Self {
        unsafe { hv.pthx().hv_iterinit(hv.as_ptr()) };
        Keys(hv)
    }
}

impl<'a> Iterator for Keys<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let pthx = self.0.pthx();
            let hv_ptr = self.0.as_ptr();

            let he = pthx.hv_iternext(hv_ptr);
            if he.is_null() {
                None
            } else {
                let mut klen: raw::I32 = mem::uninitialized();
                let k_ptr = pthx.hv_iterkey(he, &mut klen as *mut _) as *const u8;
                let k = from_raw_parts(k_ptr, klen as usize);
                Some(k)
            }
        }
    }
}
