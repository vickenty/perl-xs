#[macro_use]
extern crate cstr;
#[macro_use]
extern crate perl_xs;
#[macro_use]
extern crate perl_sys;

package!("XSTest");

//xs! {
//    bootstrap boot_XSTest;
//    use stack;
//    use scalar;
//    use array;
//    use hash;
//    use panic;
//    use param;
//    use data;
//    use derive;
//}

mod array;
mod data;
mod derive;
mod hash;
mod panic;
mod param;
mod scalar;
mod stack;
