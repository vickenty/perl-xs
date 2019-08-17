#![feature(concat_idents)]
#![deny(missing_docs)]

//! Perl XS API for Rust.

#[macro_use]
extern crate perl_sys;

#[doc(hidden)]
pub use ctor::*;

#[doc(hidden)]
mod registry;

#[doc(hidden)]
pub mod boot;

type Subptr = extern "C" fn(pthx: *mut ::perl_sys::types::PerlInterpreter, cv: *mut crate::raw::CV);

#[doc(hidden)]
#[derive(Debug)]
pub struct Symbol {
    pub module: &'static str,
    pub package: Option<&'static str>,
    pub name: &'static str,
    pub ptr: Subptr,
}

#[doc(hidden)]
#[derive(Debug)]
pub struct Package {
    pub module: &'static str,
    pub package: &'static str,
}

#[doc(hidden)]
pub static SYMBOL_REGISTRY: crate::registry::Registry<Symbol> = crate::registry::Registry::<Symbol>::new();
#[doc(hidden)]
pub static PACKAGE_REGISTRY: crate::registry::Registry<Package> = crate::registry::Registry::<Package>::new();

#[macro_use]
mod macros;
#[macro_use]
mod helper_macros;

mod handle;
#[allow(missing_docs)]
pub mod raw;

mod array;
pub mod context;
pub mod convert;
pub mod error;
mod hash;
mod scalar;

#[doc(hidden)]
pub mod croak;

pub use perl_xs_macro_derive::DeriveTryFromContext;
pub use perl_xs_macro_function::*;

pub use crate::array::AV;
pub use crate::context::Context;
pub use crate::convert::TryFromContext;
pub use crate::hash::HV;
pub use crate::raw::{SSize_t, Size_t, IV, NV, STRLEN, UV};
pub use crate::raw::{G_DISCARD, G_VOID};
pub use crate::scalar::{DataRef, SV};
