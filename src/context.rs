use std;
use std::os::raw::c_char;

use raw::*;

pub struct Context {
    pthx: PerlContext,
    stack: OuroborosStack,
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
}
