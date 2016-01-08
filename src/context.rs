use std;
use std::os::raw::c_char;

use raw::*;
use pushable::Pushable;

pub struct Context {
    pthx: PerlContext,
    stack: OuroborosStack,
}

macro_rules! wrapper {
    ($name:ident : $func:ident $( -$flag:ident )* ($( $arg:ident : $ty:ty ),*))
        => (wrapper! { $name : $func $( -$flag )* ( $( $arg : $ty ),* ) -> () });

    ($name:ident : $func:ident ($( $arg:ident : $ty:ty ),*) -> $rt:ty)
        => (wrapper! { $name : $func ( $( $arg: $ty ),* ) self (self.pthx) -> $rt });

    ($name:ident : $func:ident -stack ( $( $arg:ident: $ty:ty ),*) -> $rt:ty)
        => (wrapper! { $name : $func ( $( $arg: $ty ),* ) self (self.pthx, &mut self.stack) -> $rt });

    ($name:ident : $func:ident ( $( $arg:ident: $ty:ty ),* ) $slf:ident ( $( $def:expr ),* ) -> $rt:ty)
        => (pub fn $name(&mut $slf, $( $arg: $ty ),*) -> $rt { unsafe { $func($( $def ),*, $( $arg ),*) } });
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

    wrapper! { prepush: ouroboros_stack_prepush -stack () }
    wrapper! { putback: ouroboros_stack_putback -stack () }

    pub fn push<T>(&mut self, value: T) where T: Pushable {
        value.push_extend(self.pthx, &mut self.stack);
    }

    pub unsafe fn push_unsafe<T>(&mut self, value: T) where T: Pushable {
        value.push_unsafe(self.pthx, &mut self.stack);
    }

    pub fn extend(&mut self, len: Size_t) {
        unsafe {
            ouroboros_stack_extend(self.pthx, &mut self.stack, len);
        }
    }
}
