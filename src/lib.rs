#![deny(missing_docs)]

//! Perl XS API for Rust.

#[macro_use]
extern crate perl_sys;

#[macro_use]
mod macros;
#[macro_use]
mod helper_macros;

#[allow(missing_docs)]
pub mod raw;
mod handle;

mod scalar;
mod array;
mod hash;
pub mod context;
pub mod convert;
pub mod error;

#[doc(hidden)]
pub mod croak;

pub use perl_xs_macro_derive::FromPerlKV;

pub use crate::array::AV;
pub use crate::context::Context;
pub use crate::convert::FromPerlKV;
pub use crate::hash::HV;
pub use crate::raw::{G_DISCARD, G_VOID};
pub use crate::raw::{SSize_t, Size_t, IV, NV, STRLEN, UV};
pub use crate::scalar::{DataRef, SV};

