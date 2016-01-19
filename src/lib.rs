extern crate perl_sys;

pub mod context;
pub mod pushable;
pub mod handle;
pub mod scalar;
pub mod array;

#[macro_use]
mod macros;

pub mod raw {
    pub use perl_sys::types::*;
    pub use perl_sys::funcs::*;
    pub use perl_sys::consts::*;
}

pub use context::Context;
pub use raw::{ IV, UV, NV };
pub use scalar::Scalar;
pub use array::Array;
pub use handle::{ Full, Bare, Temp };

pub type SV = Full<raw::SV>;
pub type AV = Full<raw::AV>;
pub type HV = Full<raw::HV>;

pub type BareSV = Bare<raw::SV>;
pub type BareAV = Bare<raw::AV>;
pub type BareHV = Bare<raw::HV>;

pub type TempSV = Temp<raw::SV>;
pub type TempAV = Temp<raw::AV>;
pub type TempHV = Temp<raw::HV>;
