use raw::*;
use scalar::Scalar;
use array::Array;

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

    pub unsafe fn new_incref(pthx: PerlContext, raw: *mut T) -> Self {
        let mut sv = Full::new(pthx, raw);
        sv.incref();
        sv
    }

    unsafe fn incref(&mut self) {
        ouroboros_sv_refcnt_inc_void_nn(self.0, self.1 as *mut SV)
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
            ouroboros_sv_refcnt_dec_nn(self.0, self.1 as *mut SV);
        }
    }
}

impl Scalar for Full<SV> {
    fn get_pthx(&self) -> PerlContext { self.0 }
    fn get_raw_ptr(&self) -> *mut SV { self.1 }
}

impl Array for Full<AV> {
    fn get_pthx(&self) -> PerlContext { self.0 }
    fn get_raw_ptr(&self) -> *mut AV { self.1 }
}

pub struct Temp<T: ?Sized>(PerlContext, *mut T);

impl<T: ?Sized> Temp<T> {
    pub fn new(pthx: PerlContext, raw: *mut T) -> Temp<T> {
        Temp(pthx, raw)
    }
}

impl Scalar for Temp<SV> {
    fn get_pthx(&self) -> PerlContext { self.0 }
    fn get_raw_ptr(&self) -> *mut SV { self.1 }
}

/// Same as `std::convert::From`, except does not have `impl<T> From<T> for T`  which is not safe
/// for Temp values. Bonus is that we can implement it for options of our types.
pub trait From<T> {
    fn from(T) -> Self;
}

impl From<Temp<SV>> for IV {
    fn from(src: Temp<SV>) -> IV {
        src.to_iv()
    }
}

impl From<Temp<SV>> for Full<SV> {
    fn from(src: Temp<SV>) -> Full<SV> {
        src.copy()
    }
}

impl From<Temp<AV>> for Full<AV> {
    fn from(src: Temp<AV>) -> Full<AV> {
        unsafe {
            let mut f = Full::new(src.0, src.1);
            f.incref();
            f
        }
    }
}

impl<S, T> From<Option<S>> for Option<T> where T: From<S> {
    fn from(src: Option<S>) -> Self {
        src.map(|val| T::from(val))
    }
}
