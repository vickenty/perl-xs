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
//pub mod derive;

#[doc(hidden)]
pub mod croak;

pub use raw::{ IV, UV, NV, Size_t, SSize_t, STRLEN };
pub use raw::{ G_DISCARD, G_VOID };
pub use scalar::{SV, DataRef};
pub use array::AV;
pub use hash::HV;
//pub use derive::;
