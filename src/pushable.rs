use std::os::raw::c_char;
use raw::*;

pub trait Pushable {
    fn push_extend(self, ctx: PerlContext, stack: &mut OuroborosStack);
    fn push_unsafe(self, ctx: PerlContext, stack: &mut OuroborosStack);
}

macro_rules! imp_inner {
    ($extend:ident, $quick:ident, $this:ident, $( $arg:expr ),*) => {
        fn push_extend($this, ctx: PerlContext, stack: &mut OuroborosStack) {
            unsafe { $extend(ctx, stack, $( $arg ),*) }
        }
        fn push_unsafe($this, ctx: PerlContext, stack: &mut OuroborosStack) {
            unsafe { $quick(ctx, stack, $( $arg ),*) }
        }
    }
}

macro_rules! imp {
    ($ty:ty, $extend:ident, $quick:ident, $this:ident, $( $arg:expr ),*) => {
        impl Pushable for $ty {
            imp_inner! { $extend, $quick, $this, $( $arg ),* }
        }
    }
}

imp! { IV, ouroboros_stack_xpush_iv, ouroboros_stack_push_iv, self, self }
imp! { UV, ouroboros_stack_xpush_uv, ouroboros_stack_push_uv, self, self }
imp! { NV, ouroboros_stack_xpush_nv, ouroboros_stack_push_nv, self, self }

impl<'a> Pushable for &'a str {
    imp_inner! {
        ouroboros_stack_xpush_pv, ouroboros_stack_push_pv,
        self, self.as_ptr() as *const c_char, self.len() as STRLEN
    }
}
