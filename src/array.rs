use handle::Owned;
use raw;
use raw::{ SSize_t };
use SV;

pub struct AV(Owned<raw::AV>);

impl AV {
    pub unsafe fn from_raw_owned(pthx: raw::Interpreter, raw: *mut raw::AV) -> AV {
        AV(Owned::from_raw_owned(pthx, raw))
    }

    pub unsafe fn from_raw_borrowed(pthx: raw::Interpreter, raw: *mut raw::AV) -> AV {
        AV(Owned::from_raw_borrowed(pthx, raw))
    }

    fn pthx(&self) -> raw::Interpreter { self.0.pthx() }

    fn as_ptr(&self) -> *mut raw::AV { self.0.as_ptr() }

    method! { simple fn clear() = av_clear() }
    method! { getter fn delete(key: SSize_t) = av_delete(key, 0) }
    method! { simple fn discard(key: SSize_t) = av_delete(key, raw::G_DISCARD as raw::I32) }
    method! { simple fn exists(key: SSize_t) -> bool = av_exists(key) != 0 }
    method! { simple fn extend(key: SSize_t) = av_extend(key) }
    method! { getptr fn fetch(key: SSize_t) = av_fetch(key, 0) }
    method! { getptr fn fetch_lvalue(key: SSize_t) = av_fetch(key, 1) }
    method! { simple fn fill(fill: SSize_t) = av_fill(fill) }
    method! { simple fn top_index() -> SSize_t = av_len() }
    method! { getter fn pop() = av_pop() }
    method! { simple fn push(val: SV) = av_push(val.into_raw()) }
    method! { getter fn shift() = av_shift() }
    method! { simple fn undef() = av_undef() }
    method! { simple fn unshift(num: SSize_t) = av_unshift(num) }

    pub fn store(&self, key: SSize_t, val: SV) {
        unsafe {
            let svpp = self.pthx().av_store(self.as_ptr(), key, val.into_raw());
            if !svpp.is_null() { self.pthx().sv_refcnt_dec(*svpp) }
        }
    }
}
