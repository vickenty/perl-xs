#[macro_use]
extern crate perl_xs;

mod stack;
mod scalar;
mod array;

xs! {
    bootstrap boot_XSTest;
    use stack;
    use scalar;
    use array;
}
