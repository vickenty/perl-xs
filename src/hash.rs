use crate::SV;
use crate::convert::TryFromSV;
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
