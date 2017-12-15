//! Helper functions for the croak! macro.

use std::any::Any;
use std::fmt::{Arguments, Write};
use std::panic;

#[doc(hidden)]
#[inline(never)]
pub fn croak<T>(payload: T) -> !
where
    T: Any + Send,
{
    panic::resume_unwind(Box::new(payload));
}

#[doc(hidden)]
#[inline(never)]
pub fn croak_fmt(args: &Arguments) -> ! {
    let mut s = String::new();
    let _ = s.write_fmt(*args);
    croak(s);
}
