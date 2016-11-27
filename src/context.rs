//! Context for XS subroutine calls.
use std;
use raw;
use { AV, SV };
use convert::{ FromSV, IntoSV, TryFromSV };
use std::ffi::CStr;

/// XS call context.
pub struct Context {
    pthx: raw::Interpreter,
    stack: raw::Stack,
}

const EMPTY: &'static [i8] = &[ 0 ];

impl Context {
    /// Invoke closure with the context of Perl subroutine call.
    ///
    /// This function must be called exactly once per subroutine: it consumes one stack frame
    /// prepared for us by the Perl interpreter. Multiple use will leave the stack state in bad
    /// shape.
    ///
    /// This function called automatically by subroutines defined with the `xs!` macro.
    ///
    /// Any panics that happen inside the closure will be converted to Perl exceptions and
    /// re-thrown. Panics with `String` or `&str` argument will use that as the exception text,
    /// other types will result in the default text message.
    ///
    /// Perl exceptions in code called by the closure will be captured and turned into panics, to be
    /// re-thrown again as perl exceptions after executing Rust destructors. Note, that Perl
    /// extension API does not allow such exceptions to be handled by the programmer (see paragraph
    /// on [Exception Handling](http://perldoc.perl.org/perlguts.html#Exception-Handling) in the
    /// Perl documentation).
    #[inline]
    pub fn wrap<F>(pthx: raw::PerlThreadContext, f: F)
        where F: FnOnce(&mut Self) + std::panic::UnwindSafe
    {
        unsafe {
            let perl = raw::Interpreter::new(pthx);

            perl.catch_unwind(|| {
                let mut ctx = Context {
                    pthx: perl,
                    stack: std::mem::uninitialized(),
                };

                perl.stack_init(&mut ctx.stack);

                f(&mut ctx)
            });
        }
    }

    // STACK

    /// Rewind stack pointer to the base of current frame.
    #[inline]
    pub fn st_prepush(&mut self) {
        unsafe { self.pthx.stack_prepush(&mut self.stack) };
    }

    /// Copy local stack pointer back to Perl.
    ///
    /// See: [`PUTBACK`](http://perldoc.perl.org/perlapi.html#PUTBACK).
    #[inline]
    pub fn st_putback(&mut self) {
        unsafe { self.pthx.stack_putback(&mut self.stack) };
    }

    /// Return number of items on the argument stack.
    ///
    /// [`items`](http://perldoc.perl.org/perlapi.html#items).
    #[inline]
    pub fn st_items(&mut self) -> isize {
        unsafe { self.pthx.stack_items(&mut self.stack) as isize }
    }

    unsafe fn st_fetch_raw(&mut self, idx: isize) -> Option<*mut raw::SV> {
        if idx >= self.st_items() {
            return None;
        }
        let svp = self.pthx.stack_fetch(&mut self.stack, idx as raw::SSize_t);
        if svp.is_null() {
            return None;
        }

        Some(svp)
    }

    /// Fetch value from the Perl stack.
    ///
    /// See: [`ST`](http://perldoc.perl.org/perlapi.html#ST).
    #[inline]
    pub fn st_fetch<T>(&mut self, idx: isize) -> Option<T> where T: FromSV
    {
        unsafe {
            self.st_fetch_raw(idx).map(|svp| T::from_sv(self.pthx, svp))
        }
    }

    /// Fetch value from the Perl stack and try to convert to `T`.
    #[inline]
    pub fn st_try_fetch<T>(&mut self,idx: isize) -> Option<Result<T, T::Error>>
        where T: TryFromSV
    {
        unsafe {
            self.st_fetch_raw(idx).map(|svp| T::try_from_sv(self.pthx, svp))
        }
    }

    /// Push value onto Perl stack.
    ///
    /// See: [`mXPUSHs`](http://perldoc.perl.org/perlapi.html#mXPUSHs).
    #[inline]
    pub fn st_push<T>(&mut self, val: T) where T: IntoSV {
        let sv = val.into_sv(self.pthx);
        unsafe { self.pthx.stack_xpush_sv_mortal(&mut self.stack, sv.into_raw()) };
    }

    // XSUB

    /// Register new Perl xsub.
    ///
    /// See: [`newXS`](http://perldoc.perl.org/perlapi.html#newXS).
    #[inline]
    pub fn new_xs(&mut self, name: &CStr, xsaddr: raw::XSUBADDR_t) {
        unsafe { self.pthx.new_xs(name.as_ptr(), xsaddr, EMPTY.as_ptr()) };
    }

    // GLOBALS

    /// Return the AV of the specified Perl global or package array.
    ///
    /// See: [`get_av`](http://perldoc.perl.org/perlapi.html#get_av).
    #[inline]
    pub fn get_av(&mut self, name: &CStr) -> Option<AV> {
        let avp = unsafe { self.pthx.get_av(name.as_ptr(), 0) };
        if avp.is_null() {
            None
        } else {
            Some(unsafe { AV::from_raw_borrowed(self.pthx, avp) })
        }
    }

    /// Call subroutine by name.
    ///
    /// See: [`call_pv`](http://perldoc.perl.org/perlapi.html#call_pv).
    #[inline]
    pub fn call_pv(&mut self, name: &CStr, flags: raw::U32) {
        unsafe { self.pthx.call_pv(name.as_ptr(), flags as raw::I32) };
    }

    // SCALARS

    /// Allocate new SV of type appropriate to store `T`
    #[inline]
    pub fn new_sv<T>(&mut self, val: T) -> SV where T: IntoSV {
        val.into_sv(self.pthx)
    }
}
