use std::{ mem, slice, string };
use handle::Owned;
use raw;
use raw::{ IV, UV, NV };
use convert::{ IntoSV, FromSV };

pub struct SV(Owned<raw::SV>);

impl SV {
    method! {
        /// Coerce the given SV to an integer and return it.
        ///
        /// Perl macro: [`SvIV`](http://perldoc.perl.org/perlapi.html#SvIV).
        simple fn iv() -> IV = sv_iv()
    }
    method! {
        /// Coerce the given SV to an unsigned integer and return it.
        ///
        /// Perl macro: [`SvUV`](http://perldoc.perl.org/perlapi.html#SvUV).
        simple fn uv() -> UV = sv_uv()
    }
    method! {
        /// Coerce the given SV to a floating point value and return it.
        ///
        /// Perl macro: [`SvNV`](http://perldoc.perl.org/perlapi.html#SvNV).
        simple fn nv() -> NV = sv_nv()
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

    /// Return a copy of string in the SV as vector of bytes.
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
