use std;
use std::os::raw::c_char;

use raw::*;
use pushable::Pushable;

pub struct Context {
    pthx: PerlContext,
    stack: OuroborosStack,
}

macro_rules! simple_wrapper {
    ($meth:ident, $func:ident, $( $arg:ident: $ty:ty ),*) => {
        pub fn $meth(&mut self, $( $arg: $ty ),*) {
            unsafe { $func(self.pthx, &mut self.stack, $( $arg ),*) }
        }
    }
}

impl Context {
    pub fn new(pthx: PerlContext) -> Context {
        unsafe {
            let mut ctx = Context {
                pthx: pthx,
                stack: std::mem::uninitialized(),
            };
            ouroboros_stack_init(ctx.pthx, &mut ctx.stack);
            ctx
        }
    }

    pub fn new_xs(&mut self, name: &str, func: XSUBADDR_t) {
        let cname = std::ffi::CString::new(name).unwrap();
        unsafe {
            Perl_newXS(self.pthx, cname.as_ptr(), func, b"\0" as *const _ as *const c_char);
        }
    }

    simple_wrapper! { prepush, ouroboros_stack_prepush, }
    simple_wrapper! { putback, ouroboros_stack_putback, }

    pub fn push<T>(&mut self, value: T) where T: Pushable {
        value.push_extend(self.pthx, &mut self.stack);
    }
}
