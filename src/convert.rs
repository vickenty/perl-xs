//! Traits for converting to and from Perl scalars.

use std::fmt::Display;
use raw;
use SV;

/// Fast unsafe conversion from raw SV pointer.
pub trait FromSV {
    /// Perform the conversion.
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> Self;
}

/// Construct new `SV` from `self`.
pub trait IntoSV {
    /// Perform the conversion.
    fn into_sv(self, pthx: raw::Interpreter) -> SV;
}

/// Attempt unsafe conversion from a raw SV pointer.
pub trait TryFromSV: Sized {
    /// The type returned in the event of a conversion error.
    type Error: Display;
    /// Perform the conversion.
    unsafe fn try_from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> Result<Self, Self::Error>;
}

impl<T> TryFromSV for T where T: FromSV {
    type Error = &'static str;
    unsafe fn try_from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> Result<T, Self::Error>
    {
        Ok(T::from_sv(pthx, raw))
    }
}
