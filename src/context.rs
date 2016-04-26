//! Context for XS subroutine calls.
use std;
use raw;
use { IV, AV };
use convert::{ FromSV, IntoSV };
use std::ffi::CStr;

/// XS call context.
pub struct Context<'a> {
    pthx: raw::Interpreter,
    stack: raw::Stack,
    marker: std::marker::PhantomData<&'a raw::PerlThreadContext>
}

const EMPTY: &'static [i8] = &[ 0 ];

impl<'a> Context<'a> {
    /// Set up call context.
    ///
    /// See: ['dXSARGS'](http://perldoc.perl.org/perlapi.html#dXSARGS).
    pub fn new(pthx: &'a raw::PerlThreadContext) -> Self {
        unsafe {
            let mut ctx = Context {
                pthx: raw::Interpreter::new(*pthx),
                stack: std::mem::uninitialized(),
                marker: std::marker::PhantomData,
            };

            ctx.pthx.stack_init(&mut ctx.stack);

            ctx
        }
    }

    // STACK

    /// Rewind stack pointer to the base of current frame.
    pub fn st_prepush(&mut self) {
        unsafe { self.pthx.stack_prepush(&mut self.stack) };
    }

    /// Copy local stack pointer back to Perl.
    ///
    /// See: ['PUTBACK'](http://perldoc.perl.org/perlapi.html#PUTBACK).
    pub fn st_putback(&mut self) {
        unsafe { self.pthx.stack_putback(&mut self.stack) };
    }

    /// Fetch value from the Perl stack.
    ///
    /// See: ['ST'](http://perldoc.perl.org/perlapi.html#ST).
    pub fn st_fetch<T>(&mut self, idx: isize) -> T where T: FromSV
    {
        /* FIXME: panic if idx > items */
        let svp = unsafe { self.pthx.stack_fetch(&mut self.stack, idx as raw::SSize_t) };
        unsafe { T::from_sv(self.pthx, svp) }
    }

    /// Push value onto Perl stack.
    ///
    /// See: ['mXPUSHs'](http://perldoc.perl.org/perlapi.html#mXPUSHs).
    pub fn st_push<T>(&mut self, val: T) where T: IntoSV {
        let sv = val.into_sv(self.pthx);
        unsafe { self.pthx.stack_xpush_sv_mortal(&mut self.stack, sv.into_raw()) };
    }

    // XSUB

    /// Register new Perl xsub.
    ///
    /// See: ['newXS'](http://perldoc.perl.org/perlapi.html#newXS).
    pub fn new_xs(&mut self, name: &CStr, xsaddr: raw::XSUBADDR_t) {
        unsafe { self.pthx.new_xs(name.as_ptr(), xsaddr, EMPTY.as_ptr()) };
    }

    // GLOBALS

    /// Return the AV of the specified Perl global or package array.
    ///
    /// See: ['get_av'](http://perldoc.perl.org/perlapi.html#get_av).
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
    /// See: ['call_pv'](http://perldoc.perl.org/perlapi.html#call_pv).
    pub fn call_pv(&mut self, name: &CStr, flags: IV) {
        unsafe { self.pthx.call_pv(name.as_ptr(), flags as raw::I32) };
    }
}
