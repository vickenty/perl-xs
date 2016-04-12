use std::mem;

use handle::Owned;
use raw;
use raw::{ IV, UV, NV };
use convert::{ IntoSV, FromRaw };

pub struct SV(Owned<raw::SV>);

impl SV {
    pub fn iv(&self) -> IV {
        let r = &*self.0;
        unsafe { r.pthx().sv_iv(r.as_ptr()) }
    }

    pub fn uv(&self) -> UV {
        let r = &*self.0;
        unsafe { r.pthx().sv_uv(r.as_ptr()) }
    }

    pub fn nv(&self) -> NV {
        let r = &*self.0;
        unsafe { r.pthx().sv_nv(r.as_ptr()) }
    }

    pub fn into_raw(self) -> *mut raw::SV {
        let raw = self.0.as_ptr();
        mem::forget(self);
        raw
    }

    pub unsafe fn from_raw_owned(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
        SV(Owned::from_raw_owned(pthx, raw))
    }
    
    pub unsafe fn from_raw_borrowed(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
        SV(Owned::from_raw_borrowed(pthx, raw))
    }
}

impl FromRaw<raw::SV> for IV {
    unsafe fn from_raw(pthx: raw::Interpreter, raw: *mut raw::SV) -> IV {
        pthx.sv_iv(raw)
    }
}

impl FromRaw<raw::SV> for SV {
    unsafe fn from_raw(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
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

impl<'a> IntoSV for &'a str {
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        use std::ffi::CString;
        let buf = CString::new(self).unwrap();
        unsafe {
            let svp = pthx.new_sv_pvn(buf.as_ptr(), self.len() as raw::STRLEN, raw::SVf_UTF8 as raw::U32);
            SV::from_raw(pthx, svp)
        }
    }
}
