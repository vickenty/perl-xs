use raw;
use raw::{ PerlContext };
use scalar::Scalar;

pub struct Bare<T: ?Sized> (*mut T);

impl<T: ?Sized> Bare<T> {
    pub unsafe fn new(raw: *mut T) -> Bare<T> {
        Bare(raw)
    }

    fn into_raw(self) -> *mut T {
        self.0
    }
}

impl<T: ?Sized> Drop for Bare<T> {
    fn drop(&mut self) {
        panic!("refcnt leaked");
    }
}

pub struct Full<T: ?Sized> (PerlContext, *mut T);

impl<T: ?Sized> Full<T> {
    pub unsafe fn new(pthx: PerlContext, raw: *mut T) -> Self {
        Full(pthx, raw)
    }

    pub fn from_bare(pthx: PerlContext, bare: Bare<T>) -> Self {
        unsafe { Self::new(pthx, bare.into_raw()) }
    }

    pub fn into_bare(self) -> Bare<T> {
        unsafe { Bare::new(self.1) }
    }
}

impl<T: ?Sized> Drop for Full<T> {
    fn drop(&mut self) {
        unsafe {
            raw::ouroboros_sv_refcnt_dec_nn(self.0, self.1 as *mut raw::SV);
        }
    }
}

impl Scalar for Full<raw::SV> {
    fn get_pthx(&self) -> PerlContext { self.0 }
    fn get_raw_ptr(&self) -> *mut raw::SV { self.1 }
}

pub struct Transient<T: ?Sized>(PerlContext, *mut T);

impl<T: ?Sized> Transient<T> {
    pub fn new(pthx: PerlContext, raw: *mut T) -> Transient<T> {
        Transient(pthx, raw)
    }
}

impl Scalar for Transient<raw::SV> {
    fn get_pthx(&self) -> PerlContext { self.0 }
    fn get_raw_ptr(&self) -> *mut raw::SV { self.1 }
}

impl From<Transient<raw::SV>> for raw::IV {
    fn from(src: Transient<raw::SV>) -> raw::IV {
        src.to_iv()
    }
}

impl From<Transient<raw::SV>> for Full<raw::SV> {
    fn from(src: Transient<raw::SV>) -> Full<raw::SV> {
        src.copy()
    }
}

pub type SV = Full<raw::SV>;
pub type AV = Full<raw::AV>;
pub type HV = Full<raw::HV>;

pub type BareSV = Bare<raw::SV>;
pub type BareAV = Bare<raw::AV>;
pub type BareHV = Bare<raw::HV>;

