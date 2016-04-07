use handle::Owned;
use raw;
use raw::{ SSize_t };
use convert::FromRaw;

pub struct AV(Owned<raw::AV>);

impl AV {
    pub unsafe fn from_raw_owned(pthx: raw::Interpreter, raw: *mut raw::AV) -> AV {
        AV(Owned::from_raw_owned(pthx, raw))
    }
    pub unsafe fn from_raw_borrowed(pthx: raw::Interpreter, raw: *mut raw::AV) -> AV {
        AV(Owned::from_raw_borrowed(pthx, raw))
    }

    pub fn fetch<T>(&self, key: SSize_t) -> Option<T> where T: FromRaw<raw::SV> {
        let r = &*self.0;
        let svpp = unsafe { r.pthx().av_fetch(r.as_ptr(), key) };
        if svpp.is_null() {
            None
        } else {
            Some(unsafe { T::from_raw(r.pthx(), *svpp) })
        }
    }
}
