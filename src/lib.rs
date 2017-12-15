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

pub use array::AV;
pub use context::Context;
pub use convert::FromPerlKV;
pub use hash::HV;
pub use raw::{G_DISCARD, G_VOID};
pub use raw::{SSize_t, Size_t, IV, NV, STRLEN, UV};
pub use scalar::{DataRef, SV};
