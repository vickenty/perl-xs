use handle::Owned;
use raw;
use raw::{ SSize_t };
use convert::FromRaw;
use SV;

pub struct AV(Owned<raw::AV>);

macro_rules! call_method {
    ($this:ident.$method:ident($( $args:expr ),*))
        => (unsafe { $this.pthx().$method($this.as_ptr(), $( $args ),*) })
}

macro_rules! convert {
    ($this:ident, $T:ident, $check:expr, $svp:expr)
        => (if $check.is_null() { None } else { Some(unsafe { $T::from_raw($this.pthx(), $svp) }) })
}

impl AV {
    pub unsafe fn from_raw_owned(pthx: raw::Interpreter, raw: *mut raw::AV) -> AV {
        AV(Owned::from_raw_owned(pthx, raw))
    }

    pub unsafe fn from_raw_borrowed(pthx: raw::Interpreter, raw: *mut raw::AV) -> AV {
        AV(Owned::from_raw_borrowed(pthx, raw))
    }

    fn pthx(&self) -> raw::Interpreter { self.0.pthx() }

    fn as_ptr(&self) -> *mut raw::AV { self.0.as_ptr() }


    pub fn clear(&self) {
        unsafe { self.pthx().av_clear(self.as_ptr()) }
    }

    pub fn delete<T>(&self, key: SSize_t) -> Option<T> where T: FromRaw<raw::SV> {
        let svp = call_method!(self.av_delete(key, 0));
        convert!(self, T, svp, svp)
    }

    pub fn discard(&self, key: SSize_t) {
        call_method!(self.av_delete(key, raw::G_DISCARD as raw::I32));
    }

    pub fn exists(&self, key: SSize_t) -> bool {
        call_method!(self.av_exists(key)) != 0
    }

    pub fn extend(&self, key: SSize_t) {
        call_method!(self.av_extend(key));
    }

    pub fn fetch<T>(&self, key: SSize_t) -> Option<T> where T: FromRaw<raw::SV> {
        let svpp = call_method!(self.av_fetch(key, 0));
        convert!(self, T, svpp, *svpp)
    }

    pub fn fetch_lvalue<T>(&self, key: SSize_t) -> T where T: FromRaw<raw::SV> {
        let svpp = call_method!(self.av_fetch(key, 1));
        unsafe { T::from_raw(self.pthx(), *svpp) }
    }

    pub fn fill(&self, fill: SSize_t) {
        call_method!(self.av_fill(fill));
    }

    pub fn len(&self) -> SSize_t {
        call_method!(self.av_len())
    }

    // pub fn make(&self, size: SSize_t, strp: *mut *mut SV) -> *mut AV { av_make(size, strp) }

    pub fn pop<T>(&self) -> Option<T> where T: FromRaw<raw::SV> {
        let svp = call_method!(self.av_pop());
        convert!(self, T, svp, svp)
    }

    pub fn push(&self, val: SV) {
        call_method!(self.av_push(val.into_raw()));
    }

    pub fn shift<T>(&self) -> Option<T> where T: FromRaw<raw::SV> {
        let svp = call_method!(self.av_shift());
        convert!(self, T, svp, svp)
    }

    pub fn store(&self, key: SSize_t, sv: SV) {
        let svpp = call_method!(self.av_store(key, sv.into_raw()));
        if !svpp.is_null() {
            unsafe { self.pthx().sv_refcnt_dec(*svpp) };
        }
    }

    pub fn undef(&self) {
        call_method!(self.av_undef());
    }

    pub fn unshift(&self, num: SSize_t) {
        call_method!(self.av_unshift(num));
    }
}
