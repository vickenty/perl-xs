//! Minimal wrapper around internal Perl API.

use std::ptr;
use std::mem;
use std::os::raw::{ c_int };
use perl_sys::funcs::*;

pub use perl_sys::types::*;
pub use perl_sys::consts::*;

pub type Stack = OuroborosStack;

#[derive(Copy, Clone)]
pub struct Interpreter (PerlThreadContext);

impl Interpreter {
    pub unsafe fn new(pthx: PerlThreadContext) -> Interpreter { Interpreter(pthx) }
}

// Contains result of failed JMPENV_PUSH() while propagating through Rust.
struct Xcpt(c_int);

// Wrapper to safely call potentially dying perl functions (which is pretty much all of them because
// magic and ties).
//
// Put actual call into Perl inside a closure. `ouroboros_xcpt_try()` will call the closure while
// capturing Perl exceptions, so they don't skip Rust code except the closure itself, which should
// be fine, since closure is FnMut and has no destructors.
//
// If it returns zero, we assume that closure did not die and `v` was assigned to, making it safe to
// return, otherwise we panic and possibly incorrect value of `v` is not revealed.
//
// This only works with POD return types and non-panicing $body, which should always be the case for
// Perl API.
macro_rules! xcpt_try {
    ( $pthx:expr, $( $body:stmt )* ) => {{
        let mut v = mem::zeroed();
        {
            let mut callback: &mut FnMut() = &mut || ptr::write(&mut v, { $( $body )* });
            let rc = ouroboros_xcpt_try($pthx,
                                        mem::transmute(xcpt_bouncer as extern "C" fn(_)),
                                        mem::transmute(&mut callback));
            if rc != 0 {
                panic!(Xcpt(rc));
            }
        }
        v
    }}
}

// Helper function for calling Rust closure from C.
extern "C" fn xcpt_bouncer(closure: &mut &mut FnMut()) {
    (*closure)();
}

macro_rules! perl_api_inner {
    (
        fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) = $imp:ident;
        $( $rest:tt )*
    ) => (
        perl_api_inner! {
            fn $name ( $( $pname : $ptype ),* ) -> () = $imp;
            $( $rest )*
        }
    );

    (
        fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) -> $rtype:ty = $imp:ident;
        $( $rest:tt )*
    ) => (
        pub unsafe fn $name ( &self, $( $pname : $ptype ),* ) -> $rtype {
            xcpt_try! { self.0, $imp( self.0, $( $pname ),* ) }
        }
        perl_api_inner! { $( $rest )* }
    );

    () => ();
}

macro_rules! perl_api {
    ( $( $tt:tt )* ) => ( impl Interpreter { perl_api_inner! { $( $tt )* } } )
}

perl_api! {
    fn sv_iv(sv: *mut SV) -> IV = ouroboros_sv_iv;
    fn sv_uv(sv: *mut SV) -> UV = ouroboros_sv_uv;
    fn sv_nv(sv: *mut SV) -> NV = ouroboros_sv_nv;

    fn sv_refcnt_inc(sv: *mut SV) = ouroboros_sv_refcnt_inc_void_nn;
    fn sv_refcnt_dec(sv: *mut SV) = ouroboros_sv_refcnt_dec_nn;

    fn av_clear(av: *mut AV) = Perl_av_clear;
    fn av_delete(av: *mut AV, key: SSize_t, flags: I32) -> *mut SV = Perl_av_delete;
    fn av_exists(av: *mut AV, key: SSize_t) -> c_bool = Perl_av_exists;
    fn av_extend(av: *mut AV, key: SSize_t) = Perl_av_extend;
    fn av_fetch(av: *mut AV, key: SSize_t, flags: I32) -> *mut *mut SV = Perl_av_fetch;
    fn av_fill(av: *mut AV, fill: SSize_t) = Perl_av_fill;
    fn av_len(av: *mut AV) -> SSize_t = Perl_av_len;
    fn av_make(size: SSize_t, strp: *mut *mut SV) -> *mut AV = Perl_av_make;
    fn av_pop(av: *mut AV) -> *mut SV = Perl_av_pop;
    fn av_push(av: *mut AV, val: *mut SV) = Perl_av_push;
    fn av_shift(av: *mut AV) -> *mut SV = Perl_av_shift;
    fn av_store(av: *mut AV, key: SSize_t, sv: *mut SV) -> *mut *mut SV = Perl_av_store;
    fn av_undef(av: *mut AV) = Perl_av_undef;
    fn av_unshift(av: *mut AV, num: SSize_t) = Perl_av_unshift;

    fn st_init(stack: &mut Stack) = ouroboros_stack_init;
    fn st_prepush(stack: &mut Stack) = ouroboros_stack_prepush;
    fn st_putback(stack: &mut Stack) = ouroboros_stack_putback;
    fn st_extend(stack: &mut Stack, len: Size_t) = ouroboros_stack_extend;

    fn st_fetch(stack: &mut Stack, idx: SSize_t) -> *mut SV = ouroboros_stack_fetch;
    fn st_push(stack: &mut Stack, val: *mut SV) = ouroboros_stack_push_sv;
    fn st_push_iv(stack: &mut Stack, val: IV) = ouroboros_stack_push_iv;
    fn st_push_uv(stack: &mut Stack, val: UV) = ouroboros_stack_push_uv;
    fn st_push_nv(stack: &mut Stack, val: NV) = ouroboros_stack_push_nv;

    fn call_pv(name: *const i8, flags: I32) -> I32 = Perl_call_pv;

    fn new_xs(name: *const i8, func: XSUBADDR_t, file: *const i8) -> *mut CV = Perl_newXS;
    fn new_sv(len: STRLEN) -> *mut SV = Perl_newSV;
    fn new_sv_iv(val: IV) -> *mut SV = Perl_newSViv;
    fn new_sv_uv(val: UV) -> *mut SV = Perl_newSVuv;
    fn new_sv_nv(val: NV) -> *mut SV = Perl_newSVnv;
    fn new_sv_pvn(val: *const i8, len: STRLEN, flags: U32) -> *mut SV = Perl_newSVpvn_flags;

    fn get_av(name: *const i8, flags: I32) -> *mut AV = Perl_get_av;
}
