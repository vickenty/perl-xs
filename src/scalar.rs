use std::{ mem, slice, string };
use handle::Owned;
use raw;
use raw::{ IV, UV, NV };
use raw::{ SVt_PVAV, SVt_PVHV, SVt_PVCV, SVt_PVGV };
use array::{ AV };
use convert::{ IntoSV, FromSV };

pub struct SV(Owned<raw::SV>);

impl SV {
    method! {
        /// Return true if SV is a real scalar value.
        simple fn is_scalar() -> bool = sv_type() < SVt_PVAV
    }
    method! {
        /// Return true if SV contains array.
        simple fn is_array() -> bool = sv_type() == SVt_PVAV
    }
    method! {
        /// Return true if SV contains hash.
        simple fn is_hash() -> bool = sv_type() == SVt_PVHV
    }
    method! {
        /// Return true if SV contains subroutine.
        simple fn is_code() -> bool = sv_type() == SVt_PVCV
    }
    method! {
        /// Return true if SV contains glob.
        simple fn is_glob() -> bool = sv_type() == SVt_PVGV
    }
    method! {
        /// Return true if SV contains a signed integer.
        ///
        /// Perl macro: ['SvIOK'](http://perldoc.perl.org/perlapi.html#SvIOK).
        simple fn iv_ok() -> bool = sv_iok() != 0
    }
    method! {
        /// Coerce the given SV to an integer and return it.
        ///
        /// Perl macro: [`SvIV`](http://perldoc.perl.org/perlapi.html#SvIV).
        simple fn iv() -> IV = sv_iv()
    }
    method! {
        /// Return true if SV contains a unsigned integer.
        ///
        /// Perl macro: ['SvUOK'](http://perldoc.perl.org/perlapi.html#SvUOK).
        simple fn uv_ok() -> bool = sv_uok() != 0
    }
    method! {
        /// Coerce the given SV to an unsigned integer and return it.
        ///
        /// Perl macro: [`SvUV`](http://perldoc.perl.org/perlapi.html#SvUV).
        simple fn uv() -> UV = sv_uv()
    }
    method! {
        /// Return true if SV contains a floating point value.
        ///
        /// Perl macro: ['SvNOK'](http://perldoc.perl.org/perlapi.html#SvNOK).
        simple fn nv_ok() -> bool = sv_nok() != 0
    }
    method! {
        /// Coerce the given SV to a floating point value and return it.
        ///
        /// Perl macro: [`SvNV`](http://perldoc.perl.org/perlapi.html#SvNV).
        simple fn nv() -> NV = sv_nv()
    }
    method! {
        /// Return true if SV contains a string.
        ///
        /// Perl macro: ['SvPOK'](http://perldoc.perl.org/perlapi.html#SvPOK).
        simple fn pv_ok() -> bool = sv_pok() != 0
    }
    method! {
        /// Return UTF8 flag on the SV.
        ///
        /// You should use this after a call to `pv()` or `str()`, in case any call to string
        /// overloading updates the internal flag.
        ///
        /// Perl macro: [`SvUTF8`](http://perldoc.perl.org/perlapi.html#SvUTF8).
        simple fn utf8() -> bool = sv_utf8() != 0
    }

    /// Return a copy of string in the SV as a vector of bytes.
    ///
    /// Perl macro: [`SvPV`](http://perldoc.perl.org/perlapi.html#SvPV).
    pub fn pv(&self) -> Vec<u8> {
        unsafe {
            let mut len = 0;
            let ptr = self.pthx().sv_pv(self.as_ptr(), &mut len);
            slice::from_raw_parts(ptr as *const u8, len as usize).to_owned()
        }
    }

    /// Return a copy of string in the SV.
    ///
    /// Perl macro: [`SvPV`](http://perldoc.perl.org/perlapi.html#SvPV).
    pub fn str(&self) -> Result<String, string::FromUtf8Error> {
        String::from_utf8(self.pv())
    }

    method! {
        /// Return true if SV contains a Perl reference.
        ///
        /// Perl macro: ['SvROK'](http://perldoc.perl.org/perlapi.html#SvROK).
        simple fn rv_ok() -> bool = sv_rok() != 0
    }

    unsafe fn deref_raw(&self) -> *mut raw::SV {
        self.pthx().sv_rv(self.as_ptr())
    }

    /// Dereference RV.
    ///
    /// Return `None` if `self` is not a valid Perl reference.
    pub fn deref(&self) -> Option<SV> {
        if self.rv_ok() {
            Some(unsafe { SV::from_raw_borrowed(self.pthx(), self.deref_raw()) })
        } else {
            None
        }
    }

    /// Cast SV into AV.
    pub fn as_av(self) -> Option<AV> {
        if self.is_array() {
            Some(unsafe { AV::from_raw_owned(self.pthx(), self.into_raw() as *mut _) })
        } else {
            None
        }
    }

    /// Consume SV and convert into raw pointer.
    ///
    /// Does not decrement reference count. Returned pointer must be correctly disposed of to avoid
    /// memory leaks.
    pub fn into_raw(self) -> *mut raw::SV {
        let raw = self.0.as_ptr();
        mem::forget(self);
        raw
    }

    /// Construct new instance from a raw SV pointer without incrementing reference counter.
    ///
    /// Owned SV pointers are returned by assorted
    /// [`newSV`](http://perldoc.perl.org/perlapi.html#newSV) functions.
    pub unsafe fn from_raw_owned(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
        SV(Owned::from_raw_owned(pthx, raw))
    }

    /// Construct new instance from a raw SV pointer and increment reference counter.
    ///
    /// Borrowed SV pointers exist on stack and are returned by functions like
    /// ['av_fetch`](http://perldoc.perl.org/perlapi.html#av_fetch) or
    /// ['hv_fetch'](http://perldoc.perl.org/perlapi.html#av_fetch).
    pub unsafe fn from_raw_borrowed(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
        SV(Owned::from_raw_borrowed(pthx, raw))
    }

    fn pthx(&self) -> raw::Interpreter { self.0.pthx() }

    fn as_ptr(&self) -> *mut raw::SV { self.0.as_ptr() }
}

impl FromSV for IV {
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> IV {
        pthx.sv_iv(raw)
    }
}

impl FromSV for SV {
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
        SV::from_raw_borrowed(pthx, raw)
    }
}

impl IntoSV for IV {
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe { SV::from_raw_owned(pthx, pthx.new_sv_iv(self)) }
    }
}

impl IntoSV for UV {
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe { SV::from_raw_owned(pthx, pthx.new_sv_uv(self)) }
    }
}

impl IntoSV for NV {
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe { SV::from_raw_owned(pthx, pthx.new_sv_nv(self)) }
    }
}

impl IntoSV for bool {
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe {
            let raw = if self { pthx.sv_yes() } else { pthx.sv_no() };
            SV::from_raw_owned(pthx, raw)
        }
    }
}

impl<'a> IntoSV for &'a str {
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe {
            let svp = pthx.new_sv_pvn(self.as_ptr() as *const i8,
                                      self.len() as raw::STRLEN,
                                      raw::SVf_UTF8 as raw::U32);
            SV::from_raw_owned(pthx, svp)
        }
    }
}
