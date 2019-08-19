#[macro_use]
extern crate cstr;
#[macro_use]
extern crate perl_xs;
#[macro_use]
extern crate perl_sys;

package!("XSTest");

mod array;
mod data;
mod derive;
mod hash;
mod panic;
mod param;
mod scalar;
mod stack;
