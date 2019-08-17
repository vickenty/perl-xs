//! Traits for converting to and from Perl scalars.

use crate::context::Context;
use crate::raw;
use crate::SV;
use std::fmt::Display;

/// Fast unsafe conversion from raw SV pointer.
pub trait FromSV {
    /// Perform the conversion.
    unsafe fn from_sv(perl: raw::Interpreter, raw: *mut raw::SV) -> Self;
}

/// Construct new `SV` from `self`.
pub trait IntoSV {
    /// Perform the conversion.
    fn into_sv(self, perl: raw::Interpreter) -> SV;
}

impl<T> IntoSV for Option<T>
where
    T: IntoSV,
{
    fn into_sv(self, perl: raw::Interpreter) -> SV {
        match self {
            Some(inner) => inner.into_sv(perl),
            None => unsafe { SV::from_raw_owned(perl, perl.ouroboros_sv_undef()) },
        }
    }
}

/// Attempt unsafe conversion from a raw SV pointer.
pub trait TryFromSV: Sized {
    /// The type returned in the event of a conversion error.
    type Error: Display;
    /// Perform the conversion.
    unsafe fn try_from_sv(perl: raw::Interpreter, raw: *mut raw::SV) -> Result<Self, Self::Error>;
}

impl<T> TryFromSV for T
where
    T: FromSV,
{
    type Error = &'static str;
    unsafe fn try_from_sv(perl: raw::Interpreter, raw: *mut raw::SV) -> Result<T, Self::Error> {
        Ok(T::from_sv(perl, raw))
    }
}

/// Attempt to fetch a value from the stack.
pub trait TryFromContext<'a>: Sized + 'a {
    /// The type returned in the event of a conversion error.
    type Error: Display;
    /// Perform the conversion.
    fn try_from_context<'b: 'a>(context: &'b mut Context, name: &str, index: &mut isize) -> Result<Self, Self::Error>;
}

impl<'a, T> TryFromContext<'a> for T
where
    T: TryFromSV + 'a,
{
    type Error = String;

    fn try_from_context<'b: 'a>(ctx: &'b mut Context, name: &str, index: &mut isize) -> Result<T, Self::Error> {
        let out = match ctx.st_try_fetch::<T>(*index) {
            Some(Ok(v)) => Ok(v),
            Some(Err(e)) => Err(format!("Invalid argument '{}' ({})", name, e)),
            None => Err(format!("Missing argument '{}'", name)),
        };

        *index += 1;
        out
    }
}

impl<'a, T> TryFromContext<'a> for Option<T>
where
    T: TryFromSV + 'a,
{
    type Error = String;

    fn try_from_context<'b: 'a>(ctx: &'b mut Context, name: &str, index: &mut isize) -> Result<Option<T>, Self::Error> {
        let out = match ctx.st_try_fetch::<T>(*index) {
            Some(Ok(v)) => Ok(Some(v)),
            Some(Err(e)) => Err(format!("Invalid argument '{}' ({})", name, e)),
            None => Ok(None),
        };

        *index += 1;
        out
    }
}

// For situations where you want to implement your own argument unpacking
impl<'a> TryFromContext<'a> for &'a mut Context {
    type Error = &'static str;

    fn try_from_context<'b: 'a>(ctx: &'b mut Context, _name: &str, _index: &mut isize) -> Result<&'a mut Context, Self::Error> {
        Ok(ctx)
    }
}
