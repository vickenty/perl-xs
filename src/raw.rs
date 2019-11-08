//! Minimal wrapper around internal Perl API.

use std::any;
use std::mem;
use std::panic;
use std::ptr;

use perl_sys;
pub use perl_sys::consts::*;
pub use perl_sys::initialize;
pub use perl_sys::types::*;

pub type Interpreter = perl_sys::Perl;
pub type Stack = OuroborosStack;

pub unsafe fn catch_unwind<F, T>(perl: Interpreter, f: F) -> T
where
    F: FnOnce() -> T + panic::UnwindSafe,
{
    let res = panic::catch_unwind(f);
    match res {
        Ok(v) => v,
        Err(e) => rethrow_panic(perl, e),
    }
}

unsafe fn rethrow_panic(perl: Interpreter, e: Box<dyn any::Any>) -> ! {
    let e = perl_sys::try_rethrow(perl, e);

    let mut errsv: *mut SV = ptr::null_mut();

    if let Some(&msg) = e.downcast_ref::<&str>() {
        errsv = make_error_sv(perl, msg);
    }

    if let Some(msg) = e.downcast_ref::<String>() {
        errsv = make_error_sv(perl, msg);
    }

    mem::drop(e);

    if errsv.is_null() {
        errsv = make_error_sv(perl, "unknown typed panic inside Rust code");
    }

    perl.croak_sv(errsv);
    unreachable!();
}

unsafe fn make_error_sv<T>(perl: Interpreter, e: T) -> *mut SV
where
    T: AsRef<str>,
{
    let s = e.as_ref();
    perl.newSVpvn_flags(
        s.as_ptr() as *const _,
        s.len() as STRLEN,
        SVs_TEMP | SVf_UTF8,
    )
}
