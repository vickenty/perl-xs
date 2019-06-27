use std;
use std::{mem, ptr, slice, string};
use std::any::Any;
use std::ops::Deref;
use std::os::raw::{c_char, c_int};

use crate::raw;
use crate::raw::{IV, NV, UV};
use crate::raw::{SVt_PVAV, SVt_PVCV, SVt_PVGV, SVt_PVHV};

use crate::array::AV;
use crate::convert::{FromSV, IntoSV, TryFromSV};
use crate::handle::Owned;
use crate::hash::HV;

/// Perl scalar object.
pub struct SV(Owned<raw::SV>);

impl SV {
    method! {
        /// Return true if SV is a real scalar value.
        simple fn is_scalar() -> bool = ouroboros_sv_type() < SVt_PVAV
    }
    method! {
        /// Return true if SV contains array.
        simple fn is_array() -> bool = ouroboros_sv_type() == SVt_PVAV
    }
    method! {
        /// Return true if SV contains hash.
        simple fn is_hash() -> bool = ouroboros_sv_type() == SVt_PVHV
    }
    method! {
        /// Return true if SV contains subroutine.
        simple fn is_code() -> bool = ouroboros_sv_type() == SVt_PVCV
    }
    method! {
        /// Return true if SV contains glob.
        simple fn is_glob() -> bool = ouroboros_sv_type() == SVt_PVGV
    }
    method! {
        /// Return true if SV is defined.
        ///
        /// [`SvOK`](http://perldoc.perl.org/perlapi.html#SvOK).
        simple fn ok() -> bool = ouroboros_sv_ok() != 0
    }
    method! {
        /// Return true if SV contains a signed integer.
        ///
        /// Perl macro:[`SvIOK`](http://perldoc.perl.org/perlapi.html#SvIOK).
        simple fn iv_ok() -> bool = ouroboros_sv_iok() != 0
    }
    method! {
        /// Coerce the given SV to an integer and return it.
        ///
        /// Perl macro: [`SvIV`](http://perldoc.perl.org/perlapi.html#SvIV).
        simple fn iv() -> IV = ouroboros_sv_iv()
    }
    method! {
        /// Return true if SV contains a unsigned integer.
        ///
        /// Perl macro:[`SvUOK`](http://perldoc.perl.org/perlapi.html#SvUOK).
        simple fn uv_ok() -> bool = ouroboros_sv_uok() != 0
    }
    method! {
        /// Coerce the given SV to an unsigned integer and return it.
        ///
        /// Perl macro: [`SvUV`](http://perldoc.perl.org/perlapi.html#SvUV).
        simple fn uv() -> UV = ouroboros_sv_uv()
    }
    method! {
        /// Return true if SV contains a floating point value.
        ///
        /// Perl macro:[`SvNOK`](http://perldoc.perl.org/perlapi.html#SvNOK).
        simple fn nv_ok() -> bool = ouroboros_sv_nok() != 0
    }
    method! {
        /// Coerce the given SV to a floating point value and return it.
        ///
        /// Perl macro: [`SvNV`](http://perldoc.perl.org/perlapi.html#SvNV).
        simple fn nv() -> NV = ouroboros_sv_nv()
    }
    method! {
        /// Return true if SV contains a string.
        ///
        /// Perl macro:[`SvPOK`](http://perldoc.perl.org/perlapi.html#SvPOK).
        simple fn pv_ok() -> bool = ouroboros_sv_pok() != 0
    }
    method! {
        /// Return UTF8 flag on the SV.
        ///
        /// You should use this after a call to `to_vec` or `to_string`, in case any call to string
        /// overloading updates the internal flag.
        ///
        /// Perl macro: [`SvUTF8`](http://perldoc.perl.org/perlapi.html#SvUTF8).
        simple fn utf8() -> bool = ouroboros_sv_utf8() != 0
    }

    /// Get a slice of the internal string buffer of the SV.
    ///
    /// This method is unsafe, because the buffer may be changed, moved or freed by the interpreter
    /// whenever this SV is accessed or modified. To be safe, code that uses this method should not
    /// access interpreter context, this or any other perl value while the slice is in scope.
    ///
    /// For example, this use is okay:
    ///
    /// ```
    /// # #[macro_use] extern crate perl_xs;
    /// # #[macro_use] extern crate perl_sys;
    /// # use perl_xs::SV;
    /// # xs! {
    /// #   package Dummy;
    /// #   sub foo(ctx, sv: SV) {
    ///         let bytes = unsafe { sv.as_slice().len() };
    /// #   }
    /// # }
    /// # fn main() {}
    /// ```
    ///
    /// And this code may potentially access freed memory:
    ///
    /// ```
    /// # #[macro_use] extern crate perl_xs;
    /// # #[macro_use] extern crate perl_sys;
    /// # use perl_xs::SV;
    /// # xs! {
    /// #   package Dummy;
    /// #   sub foo(ctx, sv1: SV, sv2: SV) {
    /// let msg = unsafe { sv1.as_slice() };
    /// let pos = sv2.iv() as usize;
    /// println!("{:?}", &msg[pos..]);
    /// #   }
    /// # }
    /// # fn main() {}
    /// ```
    ///
    /// Perl macro: [`SvPV`](http://perldoc.perl.org/perlapi.html#SvPV).
    #[inline]
    pub unsafe fn as_slice(&self) -> &[u8] {
        let mut len = 0;
        let ptr = self.pthx().ouroboros_sv_pv(self.as_ptr(), &mut len);
        slice::from_raw_parts(ptr as *const u8, len as usize)
    }

    /// Return a copy of string in the SV as a vector of bytes.
    ///
    /// Perl macro: [`SvPV`](http://perldoc.perl.org/perlapi.html#SvPV).
    #[inline]
    pub fn to_vec(&self) -> Vec<u8> {
        unsafe { self.as_slice().to_owned() }
    }

    /// Return a copy of string in the SV.
    ///
    /// Perl macro: [`SvPV`](http://perldoc.perl.org/perlapi.html#SvPV).
    #[inline]
    pub fn to_string(&self) -> Result<String, string::FromUtf8Error> {
        String::from_utf8(self.to_vec())
    }

    method! {
        /// Return true if SV contains a Perl reference.
        ///
        /// Perl macro:[`SvROK`](http://perldoc.perl.org/perlapi.html#SvROK).
        simple fn rv_ok() -> bool = ouroboros_sv_rok() != 0
    }

    #[inline]
    unsafe fn deref_raw(&self) -> *mut raw::SV {
        self.pthx().ouroboros_sv_rv(self.as_ptr())
    }

    /// Dereference RV.
    ///
    /// Return `None` if `self` is not a valid Perl reference.
    #[inline]
    pub fn deref(&self) -> Option<SV> {
        if self.rv_ok() {
            Some(unsafe { SV::from_raw_borrowed(self.pthx(), self.deref_raw()) })
        } else {
            None
        }
    }

    /// Dereference RV into AV.
    ///
    /// Return `None` if `self` is not an array reference.
    #[inline]
    pub fn deref_av(&self) -> Option<AV> {
        self.deref().and_then(|sv| sv.into_av())
    }

    /// Dereference RV into HV.
    ///
    /// Return `None` if `self` is not a hash reference.
    #[inline]
    pub fn deref_hv(&self) -> Option<HV> {
        self.deref().and_then(|sv| sv.into_hv())
    }

    /// Cast SV into AV.
    #[inline]
    pub fn into_av(self) -> Option<AV> {
        if self.is_array() {
            Some(unsafe { AV::from_raw_owned(self.pthx(), self.into_raw() as *mut _) })
        } else {
            None
        }
    }

    /// Cast SV into HV.
    #[inline]
    pub fn into_hv(self) -> Option<HV> {
        if self.is_hash() {
            Some(unsafe { HV::from_raw_owned(self.pthx(), self.into_raw() as *mut _) })
        } else {
            None
        }
    }

    /// Bless the SV scalar into given package.
    ///
    /// Panics if scalar is not a reference.
    ///
    /// See: [`bless`](http://perldoc.perl.org/perlfunc.html#bless)
    #[inline]
    pub fn bless(self, package: &str) -> SV {
        let pthx = self.pthx();
        unsafe {
            let stash = pthx.gv_stashpvn(
                package.as_ptr() as *const _,
                package.len() as _,
                raw::GV_ADD as _,
            );
            pthx.sv_bless(self.as_ptr(), stash);
        }
        self
    }

    /// Consume SV and convert into raw pointer.
    ///
    /// Does not decrement reference count. Returned pointer must be correctly disposed of to avoid
    /// memory leaks.
    #[inline]
    pub fn into_raw(self) -> *mut raw::SV {
        let raw = self.0.as_ptr();
        mem::forget(self);
        raw
    }

    /// Create a new reference to this scalar, transferring ownership to the reference.
    #[inline]
    pub fn into_ref(self) -> SV {
        let pthx = self.pthx();
        unsafe {
            let rv = pthx.newRV_noinc(self.into_raw());
            SV::from_raw_owned(pthx, rv)
        }
    }

    /// Store a Rust value inside the SV.
    ///
    /// SV takes ownership of the `value`, which will be dropped when the SV reference count drops
    /// to zero. It is possible to attach multiple values to the SV, but currently only one can be
    /// retrieved.
    ///
    /// This relies on [Perl magic](http://perldoc.perl.org/perlguts.html#Magic-Virtual-Tables) to
    /// store the value. Magic is not copied on scalar assignment, so to be useful, scalars with
    /// magic need to be passed around by reference.
    pub fn add_data(&self, value: Box<Any>) {
        let pthx = self.pthx();
        let svp = self.as_ptr();
        let obj = ptr::null_mut();
        let ptr = Box::into_raw(Box::new(value)) as *const c_char;
        let len = 0;
        unsafe {
            pthx.sv_magicext(svp, obj, raw::PERL_MAGIC_ext as _, &VTBL_ANY, ptr, len);
        }
    }

    /// Get a reference to a Rust value stored in the SV.
    ///
    /// `Ref` takes ownership of the SV to protect the value from being dropped while reference is
    /// alive.
    ///
    /// `None` is returned if no value was found in the SV.
    pub fn into_data_ref(self) -> Option<DataRef<Any>> {
        let pthx = self.pthx();
        unsafe {
            let magic = pthx.mg_findext(self.as_ptr(), raw::PERL_MAGIC_ext as _, &VTBL_ANY);
            if !magic.is_null() {
                let ptr = (*magic).mg_ptr as *const Box<Any>;
                Some(DataRef {
                    owner: self,
                    inner: (*ptr).as_ref(),
                })
            } else {
                None
            }
        }
    }

    /// Construct new instance from a raw SV pointer without incrementing reference counter.
    ///
    /// Owned SV pointers are returned by assorted
    /// [`newSV`](http://perldoc.perl.org/perlapi.html#newSV) functions.
    #[inline]
    pub unsafe fn from_raw_owned(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
        SV(Owned::from_raw_owned(pthx, raw))
    }

    /// Construct new instance from a raw SV pointer and increment reference counter.
    ///
    /// Borrowed SV pointers exist on stack and are returned by functions like
    /// [`av_fetch`](http://perldoc.perl.org/perlapi.html#av_fetch) or
    /// [`hv_fetch`](http://perldoc.perl.org/perlapi.html#hv_fetch).
    #[inline]
    pub unsafe fn from_raw_borrowed(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
        SV(Owned::from_raw_borrowed(pthx, raw))
    }

    #[inline]
    fn pthx(&self) -> raw::Interpreter {
        self.0.pthx()
    }

    #[inline]
    fn as_ptr(&self) -> *mut raw::SV {
        self.0.as_ptr()
    }
}

impl Clone for SV {
    fn clone(&self) -> SV {
        unsafe { SV::from_raw_borrowed(self.pthx(), self.as_ptr()) }
    }
}

impl FromSV for IV {
    #[inline]
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> IV {
        pthx.sv_iv(raw)
    }
}

impl FromSV for UV {
    #[inline]
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> UV {
        pthx.sv_uv(raw)
    }
}

impl FromSV for NV {
    #[inline]
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> NV {
        pthx.sv_nv(raw)
    }
}

impl FromSV for SV {
    #[inline]
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> SV {
        SV::from_raw_borrowed(pthx, raw)
    }
}

macro_rules! from_sv_for_option {
    ($($fn:ident, $ty:ty;)*) => ($(
        /// Return `Some(v)` if scalar value is defined, `None` otherwise.
        impl FromSV for Option<$ty> {
            #[inline]
            unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> Option<$ty> {
                if pthx.ouroboros_sv_ok(raw) != 0 {
                    Some(pthx.$fn(raw))
                } else {
                    None
                }
            }
        }
    )*)
}

from_sv_for_option! {
    sv_iv, IV;
    sv_uv, UV;
    sv_nv, NV;
}

impl TryFromSV for String {
    type Error = std::str::Utf8Error;

    #[inline]
    unsafe fn try_from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> Result<Self, Self::Error> {
        let mut len = 0;
        let ptr = pthx.ouroboros_sv_pv(raw, &mut len);
        let bytes = slice::from_raw_parts(ptr as *const u8, len as usize);
        Ok(std::str::from_utf8(bytes)?.to_owned())
    }
}

impl IntoSV for IV {
    #[inline]
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe { SV::from_raw_owned(pthx, pthx.newSViv(self)) }
    }
}

impl IntoSV for UV {
    #[inline]
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe { SV::from_raw_owned(pthx, pthx.newSVuv(self)) }
    }
}

impl IntoSV for NV {
    #[inline]
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe { SV::from_raw_owned(pthx, pthx.newSVnv(self)) }
    }
}

impl IntoSV for bool {
    #[inline]
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe {
            let raw = if self {
                pthx.ouroboros_sv_yes()
            } else {
                pthx.ouroboros_sv_no()
            };
            SV::from_raw_owned(pthx, raw)
        }
    }
}

impl FromSV for bool {
    #[inline]
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> bool {
        pthx.sv_true(raw) != 0
    }
}

impl IntoSV for String {
    #[inline]
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        (&self[..]).into_sv(pthx)
    }
}

impl<'a> IntoSV for &'a str {
    #[inline]
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        unsafe {
            let svp = pthx.newSVpvn_flags(
                self.as_ptr() as *const i8,
                self.len() as raw::STRLEN,
                raw::SVf_UTF8 as raw::U32,
            );
            SV::from_raw_owned(pthx, svp)
        }
    }
}

impl IntoSV for SV {
    #[inline]
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        assert!(self.pthx() == pthx);
        self
    }
}

impl<'a> IntoSV for &'a SV {
    #[inline]
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        assert!(self.pthx() == pthx);
        self.clone()
    }
}

pthx! {
    fn magic_free_any(_pthx, _sv: *mut raw::SV, magic: *mut raw::MAGIC) -> c_int {
        unsafe {
            let ptr = (*magic).mg_ptr as *mut Box<Any>;
            assert!(!ptr.is_null());
            drop(Box::from_raw(ptr));
        }
        0
    }
}

static VTBL_ANY: raw::MGVTBL = raw::MGVTBL {
    svt_free: Some(magic_free_any),
    ..raw::EMPTY_MGVTBL
};

impl IntoSV for Box<Any> {
    fn into_sv(self, pthx: raw::Interpreter) -> SV {
        let sv = unsafe { SV::from_raw_owned(pthx, pthx.newSV(0)) };
        sv.add_data(self);
        sv.into_ref()
    }
}

/// Reference to a Rust value stored inside the SV.
///
/// ```
/// # #[macro_use] extern crate perl_xs;
/// # #[macro_use] extern crate perl_sys;
/// # use perl_xs::{SV, DataRef, convert::TryFromSV};
/// # xs! {
/// #   package Dummy;
/// #   sub foo(ctx) {
/// let value_sv: SV = ctx.new_sv_with_data(String::from("Hello world!"));
/// let data_ref: DataRef<String> = value_sv.into_data_ref().and_then(DataRef::downcast).unwrap();
/// assert_eq!(&**data_ref, "Hello world!");
/// #   }
/// # }
/// # fn main() {}
/// ```
///
/// Because Perl scalars are always shared, `DataRef` can provide only immutable references to the stored
/// value. If mutable access is needed, value can be wrapped in a `RefCell`.
///
/// `DataRef` can be used as a type for a subroutine parameter:
///
/// ```
/// # #[macro_use] extern crate perl_xs;
/// # #[macro_use] extern crate perl_sys;
/// # use std::cell::RefCell;
/// # use perl_xs::{IV, DataRef};
/// # xs! {
/// #   package Dummy;
/// sub increment(_ctx, value: DataRef<RefCell<IV>>, amount: IV) {
///     *value.borrow_mut() += amount;
/// }
/// # }
/// # fn main() {}
/// ```
///
/// If `increment` is then called with a scalar that does not contain a `RefCell<IV>` value,
/// perl exception will be thrown.
pub struct DataRef<T: ?Sized> {
    inner: *const T,
    owner: SV,
}

impl DataRef<Any> {
    /// Attempt to downcast the ref to the concrete type while preserving owning SV.
    pub fn downcast<T: 'static>(self) -> Option<DataRef<T>> {
        let DataRef { inner, owner } = self;
        unsafe {
            (*inner).downcast_ref::<T>().map(|r| {
                DataRef {
                    inner: r,
                    owner: owner,
                }
            })
        }
    }
}

impl<T: ?Sized> Deref for DataRef<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.inner }
    }
}

impl TryFromSV for DataRef<Any> {
    type Error = &'static str;

    unsafe fn try_from_sv(pthx: raw::Interpreter, sv: *mut raw::SV) -> Result<Self, Self::Error> {
        let outer = SV::from_raw_borrowed(pthx, sv);
        let inner = outer.deref().ok_or("not a reference")?;
        inner.into_data_ref().ok_or("invalid value")
    }
}

impl<T: 'static> TryFromSV for DataRef<T> {
    type Error = &'static str;

    unsafe fn try_from_sv(pthx: raw::Interpreter, svp: *mut raw::SV) -> Result<Self, Self::Error> {
        DataRef::<Any>::try_from_sv(pthx, svp)?
            .downcast()
            .ok_or("invalid value")
    }
}
