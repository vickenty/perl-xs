//! Minimal wrapper around internal Perl API.

use std::mem;
use std::any;
use std::panic;
use std::os::raw::{ c_int, c_char };

use perl_sys::fn_wrappers::*;

pub use perl_sys::types::*;
pub use perl_sys::consts::*;

pub type Stack = OuroborosStack;

#[derive(Copy, Clone, PartialEq)]
pub struct Interpreter (PerlThreadContext);

impl Interpreter {
    pub unsafe fn new(pthx: PerlThreadContext) -> Interpreter { Interpreter(pthx) }
}

// Contains result of failed JMPENV_PUSH() while propagating through Rust.
struct Xcpt(c_int);

macro_rules! method {
    (
        fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) = $imp:ident
    ) => (
        #[inline]
        pub unsafe fn $name ( &self, $( $pname : $ptype ),* ) {
            let rc: c_int = pthx!( $imp ( self.0, $( $pname ),* ));
            if rc != 0 {
                croak!(Xcpt(rc));
            }
        }
    );

    (
        fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) -> $rtype:ty = $imp:ident
    ) => (
        #[inline]
        pub unsafe fn $name ( &self, $( $pname : $ptype ),* ) -> $rtype {
            let mut rv: $rtype = mem::zeroed();
            let rc: c_int = pthx!( $imp ( self.0, &mut rv, $( $pname ),* ));
            if rc != 0 {
                croak!(Xcpt(rc));
            }
            rv
        }
    );
}

impl Interpreter {
    pub unsafe fn catch_unwind<F, T>(&self, f: F) -> T
        where F: FnOnce() -> T + panic::UnwindSafe
    {
        let res = panic::catch_unwind(f);
        match res {
            Ok(v) => v,
            Err(e) => self.rethrow_panic(e),
        }
    }

    unsafe fn rethrow_panic(&self, e: Box<any::Any>) -> ! {
        use std::mem;
        use std::ptr;

        if let Some(&Xcpt(rc)) = e.downcast_ref() {
            mem::drop(e);
            self.rethrow_xcpt(rc);
        }

        let mut errsv: *mut SV = ptr::null_mut();

        if let Some(&msg) = e.downcast_ref::<&str>() {
            errsv = self.make_error_sv(msg);
        }

        if let Some(msg) = e.downcast_ref::<String>() {
            errsv = self.make_error_sv(msg);
        }

        mem::drop(e);

        if !errsv.is_null() {
            self.croak_sv(errsv);
        }

        let msg = b"unknown typed panic inside Rust code\0";
        pthx!(croak(self.0, msg.as_ptr() as *const c_char));
        unreachable!();
    }

    unsafe fn croak_sv(&self, sv: *mut SV) -> ! {
        use perl_sys::fn_bindings::croak_sv;
        pthx!(croak_sv(self.0, sv));
        unreachable!();
    }

    unsafe fn rethrow_xcpt(&self, rc: c_int) -> ! {
        pthx!(ouroboros_xcpt_rethrow(self.0, rc));
        unreachable!();
    }

    unsafe fn make_error_sv<T>(&self, e: T) -> *mut SV where T: AsRef<str> {
        use perl_sys::fn_bindings::newSVpvn_flags;
        let s = e.as_ref();
        pthx!(newSVpvn_flags(
            self.0,
            s.as_ptr() as *const _,
            s.len() as STRLEN,
            SVs_TEMP | SVf_UTF8))
    }

    method! { fn stack_init(arg0: *mut Stack) = ouroboros_stack_init }
    method! { fn stack_items(arg0: *mut Stack) -> c_int = ouroboros_stack_items }
    method! { fn stack_putback(arg0: *mut Stack) = ouroboros_stack_putback }
    method! { fn stack_fetch(arg0: *mut Stack, arg1: SSize_t) -> *mut SV = ouroboros_stack_fetch }
    method! { fn stack_store(arg0: *mut Stack, arg1: SSize_t, arg2: *mut SV) = ouroboros_stack_store }
    method! { fn stack_extend(arg0: *mut Stack, arg1: SSize_t) = ouroboros_stack_extend }
    method! { fn stack_pushmark(arg0: *mut Stack) = ouroboros_stack_pushmark }
    method! { fn stack_spagain(arg0: *mut Stack) = ouroboros_stack_spagain }
    method! { fn stack_xpush_sv(arg0: *mut Stack, arg1: *mut SV) = ouroboros_stack_xpush_sv }
    method! { fn stack_xpush_sv_mortal(arg0: *mut Stack, arg1: *mut SV) = ouroboros_stack_xpush_sv_mortal }
    method! { fn stack_xpush_iv(arg0: *mut Stack, arg1: IV) = ouroboros_stack_xpush_iv }
    method! { fn stack_xpush_uv(arg0: *mut Stack, arg1: UV) = ouroboros_stack_xpush_uv }
    method! { fn stack_xpush_nv(arg0: *mut Stack, arg1: NV) = ouroboros_stack_xpush_nv }
    method! { fn stack_xpush_pv(arg0: *mut Stack, arg1: *const c_char, arg2: STRLEN) = ouroboros_stack_xpush_pv }
    method! { fn stack_push_sv(arg0: *mut Stack, arg1: *mut SV) = ouroboros_stack_push_sv }
    method! { fn stack_push_sv_mortal(arg0: *mut Stack, arg1: *mut SV) = ouroboros_stack_push_sv_mortal }
    method! { fn stack_push_iv(arg0: *mut Stack, arg1: IV) = ouroboros_stack_push_iv }
    method! { fn stack_push_uv(arg0: *mut Stack, arg1: UV) = ouroboros_stack_push_uv }
    method! { fn stack_push_nv(arg0: *mut Stack, arg1: NV) = ouroboros_stack_push_nv }
    method! { fn stack_push_pv(arg0: *mut Stack, arg1: *const c_char, arg2: STRLEN) = ouroboros_stack_push_pv }
    method! { fn sv_iv(arg0: *mut SV) -> IV = ouroboros_sv_iv }
    method! { fn sv_uv(arg0: *mut SV) -> UV = ouroboros_sv_uv }
    method! { fn sv_nv(arg0: *mut SV) -> NV = ouroboros_sv_nv }
    method! { fn sv_pv(arg0: *mut SV, arg1: *mut STRLEN) -> *const c_char = ouroboros_sv_pv }
    method! { fn sv_pv_nolen(arg0: *mut SV) -> *const c_char = ouroboros_sv_pv_nolen }
    method! { fn sv_ok(arg0: *mut SV) -> U32 = ouroboros_sv_ok }
    method! { fn sv_iok(arg0: *mut SV) -> U32 = ouroboros_sv_iok }
    method! { fn sv_uok(arg0: *mut SV) -> U8 = ouroboros_sv_uok }
    method! { fn sv_nok(arg0: *mut SV) -> U32 = ouroboros_sv_nok }
    method! { fn sv_pok(arg0: *mut SV) -> U32 = ouroboros_sv_pok }
    method! { fn sv_rok(arg0: *mut SV) -> U32 = ouroboros_sv_rok }
    method! { fn sv_rv(arg0: *mut SV) -> *mut SV = ouroboros_sv_rv }
    method! { fn sv_type(arg0: *mut SV) -> svtype = ouroboros_sv_type }
    method! { fn gv_sv(arg0: *mut GV) -> *mut SV = ouroboros_gv_sv }
    method! { fn gv_av(arg0: *mut GV) -> *mut AV = ouroboros_gv_av }
    method! { fn gv_hv(arg0: *mut GV) -> *mut HV = ouroboros_gv_hv }
    method! { fn gv_cv(arg0: *mut CV) -> *mut CV = ouroboros_gv_cv }
    method! { fn sv_refcnt(arg0: *mut SV) -> U32 = ouroboros_sv_refcnt }
    method! { fn sv_refcnt_inc(arg0: *mut SV) -> *mut SV = ouroboros_sv_refcnt_inc }
    method! { fn sv_refcnt_inc_nn(arg0: *mut SV) -> *mut SV = ouroboros_sv_refcnt_inc_nn }
    method! { fn sv_refcnt_inc_void(arg0: *mut SV) = ouroboros_sv_refcnt_inc_void }
    method! { fn sv_refcnt_inc_void_nn(arg0: *mut SV) = ouroboros_sv_refcnt_inc_void_nn }
    method! { fn sv_refcnt_dec(arg0: *mut SV) = ouroboros_sv_refcnt_dec }
    method! { fn sv_refcnt_dec_nn(arg0: *mut SV) = ouroboros_sv_refcnt_dec_nn }
    method! { fn sv_flags(arg0: *mut SV) -> UV = ouroboros_sv_flags }
    method! { fn sv_utf8(arg0: *mut SV) -> c_bool = ouroboros_sv_utf8 }
    method! { fn sv_utf8_on(arg0: *mut SV) = ouroboros_sv_utf8_on }
    method! { fn sv_utf8_off(arg0: *mut SV) = ouroboros_sv_utf8_off }
    method! { fn enter() = ouroboros_enter }
    method! { fn leave() = ouroboros_leave }
    method! { fn savetmps() = ouroboros_savetmps }
    method! { fn freetmps() = ouroboros_freetmps }
    method! { fn he_hash(arg0: *mut HE) -> U32 = ouroboros_he_hash }
    method! { fn he_pv(arg0: *mut HE, arg1: *mut STRLEN) -> *const c_char = ouroboros_he_pv }
    method! { fn he_svkey(arg0: *mut HE) -> *mut SV = ouroboros_he_svkey }
    method! { fn he_svkey_force(arg0: *mut HE) -> *mut SV = ouroboros_he_svkey_force }
    method! { fn he_svkey_set(arg0: *mut HE, arg1: *mut SV) -> *mut SV = ouroboros_he_svkey_set }
    method! { fn he_val(arg0: *mut HE) -> *mut SV = ouroboros_he_val }

    method! { fn av_clear(av: *mut AV) = av_clear }
    method! { fn av_delete(av: *mut AV, key: SSize_t, flags: I32) -> *mut SV = av_delete }
    method! { fn av_exists(av: *mut AV, key: SSize_t) -> c_bool = av_exists }
    method! { fn av_extend(av: *mut AV, key: SSize_t) = av_extend }
    method! { fn av_fetch(av: *mut AV, key: SSize_t, flags: I32) -> *mut *mut SV = av_fetch }
    method! { fn av_fill(av: *mut AV, fill: SSize_t) = av_fill }
    method! { fn av_len(av: *mut AV) -> SSize_t = av_len }
    method! { fn av_make(size: SSize_t, strp: *mut *mut SV) -> *mut AV = av_make }
    method! { fn av_pop(av: *mut AV) -> *mut SV = av_pop }
    method! { fn av_push(av: *mut AV, val: *mut SV) = av_push }
    method! { fn av_shift(av: *mut AV) -> *mut SV = av_shift }
    method! { fn av_store(av: *mut AV, key: SSize_t, sv: *mut SV) -> *mut *mut SV = av_store }
    method! { fn av_undef(av: *mut AV) = av_undef }
    method! { fn av_unshift(av: *mut AV, num: SSize_t) = av_unshift }

    method! { fn hv_clear(hv: *mut HV) = hv_clear }
    method! { fn hv_clear_placeholders(hv: *mut HV) = hv_clear_placeholders }
    method! { fn hv_delete(hv: *mut HV, key: *const c_char, klen: I32, flags: I32) -> *mut SV = hv_delete }
    method! { fn hv_delete_ent(hv: *mut HV, keysv: *mut SV, flags: I32, hash: U32) -> *mut SV = hv_delete_ent }
    method! { fn hv_exists(hv: *mut HV, key: *const c_char, klen: I32) -> c_bool = hv_exists }
    method! { fn hv_exists_ent(hv: *mut HV, keysv: *mut SV, hash: U32) -> c_bool = hv_exists_ent }
    method! { fn hv_fetch(hv: *mut HV, key: *const c_char, klen: I32, lval: I32) -> *mut *mut SV = hv_fetch }
    method! { fn hv_fetch_ent(hv: *mut HV, keysv: *mut SV, lval: I32, hash: U32) -> *mut HE = hv_fetch_ent }
    method! { fn hv_fill(hv: *mut HV) -> STRLEN = hv_fill }
    method! { fn hv_iterinit(hv: *mut HV) -> I32 = hv_iterinit }
    method! { fn hv_iterkey(entry: *mut HE, retlen: *mut I32) -> *mut c_char = hv_iterkey }
    method! { fn hv_iterkeysv(entry: *mut HE) -> *mut SV = hv_iterkeysv }
    method! { fn hv_iternext(hv: *mut HV) -> *mut HE = hv_iternext }
    method! { fn hv_iternextsv(hv: *mut HV, key: *mut *mut c_char, retlen: *mut I32) -> *mut SV = hv_iternextsv }
    method! { fn hv_iterval(hv: *mut HV, entry: *mut HE) -> *mut SV = hv_iterval }
    method! { fn hv_magic(hv: *mut HV, gv: *mut GV, how: c_int) = hv_magic }
    method! { fn hv_scalar(hv: *mut HV) -> *mut SV = hv_scalar }
    method! { fn hv_store(hv: *mut HV, key: *const c_char, klen: I32, val: *mut SV, hash: U32) -> *mut *mut SV = hv_store }
    method! { fn hv_store_ent(hv: *mut HV, key: *mut SV, val: *mut SV, hash: U32) -> *mut HE = hv_store_ent }

    method! { fn call_pv(name: *const i8, flags: I32) -> I32 = call_pv }

    method! { fn new_xs(name: *const i8, func: XSUBADDR_t, file: *const i8) -> *mut CV = newXS }
    method! { fn new_sv(len: STRLEN) -> *mut SV = newSV }
    method! { fn new_sv_iv(val: IV) -> *mut SV = newSViv }
    method! { fn new_sv_uv(val: UV) -> *mut SV = newSVuv }
    method! { fn new_sv_nv(val: NV) -> *mut SV = newSVnv }
    method! { fn new_sv_pvn(val: *const i8, len: STRLEN, flags: U32) -> *mut SV = newSVpvn_flags }

    method! { fn get_av(name: *const i8, flags: I32) -> *mut AV = get_av }
    method! { fn sv_yes() -> *mut SV = ouroboros_sv_yes }
    method! { fn sv_no() -> *mut SV = ouroboros_sv_no }
    method! { fn sv_undef() -> *mut SV = ouroboros_sv_undef }
}
