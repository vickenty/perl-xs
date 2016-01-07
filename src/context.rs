use std;

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
}
