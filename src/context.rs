//! Context for XS subroutine calls.
use crate::{AV, SV};
use crate::convert::{FromSV, IntoSV, TryFromSV};
use crate::raw;
use std;
use std::ffi::CStr;

/// XS call context.
pub struct Context {
    perl: raw::Interpreter,
    stack: raw::Stack,
}

const EMPTY: &'static [i8] = &[0];


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
    pub fn wrap<R, F>(perl: raw::Interpreter, f: F)
    where
        R: Stackable,
        F: FnOnce(&mut Self) -> R + std::panic::UnwindSafe,
    {
        unsafe {
            raw::catch_unwind(perl, || {
                let mut ctx = Context {
                    perl: perl,
                    stack: std::mem::uninitialized(),
                };

                perl.ouroboros_stack_init(&mut ctx.stack);

                let value = f(&mut ctx);

                value.push_to(&mut ctx);

                perl.ouroboros_stack_putback(&mut ctx.stack);
            });
        }
    }

    // STACK

    /// Copy local stack pointer back to Perl.
    ///
    /// See: [`PUTBACK`](http://perldoc.perl.org/perlapi.html#PUTBACK).
    #[inline]
    pub fn st_putback(&mut self) {
        unsafe { self.perl.ouroboros_stack_putback(&mut self.stack) };
    }

    /// Return number of items on the argument stack.
    ///
    /// [`items`](http://perldoc.perl.org/perlapi.html#items).
    #[inline]
    pub fn st_items(&mut self) -> isize {
        unsafe { self.perl.ouroboros_stack_items(&mut self.stack) as isize }
    }

    unsafe fn st_fetch_raw(&mut self, idx: isize) -> Option<*mut raw::SV> {
        if idx >= self.st_items() {
            return None;
        }
        let svp = self.perl
            .ouroboros_stack_fetch(&mut self.stack, idx as raw::SSize_t);
        if svp.is_null() {
            return None;
        }

        Some(svp)
    }

    /// Fetch value from the Perl stack.
    ///
    /// See: [`ST`](http://perldoc.perl.org/perlapi.html#ST).
    #[inline]
    pub fn st_fetch<T>(&mut self, idx: isize) -> Option<T>
    where
        T: FromSV,
    {
        unsafe { self.st_fetch_raw(idx).map(|svp| T::from_sv(self.perl, svp)) }
    }

    /// Fetch value from the Perl stack and try to convert to `T`.
    #[inline]
    pub fn st_try_fetch<T>(&mut self, idx: isize) -> Option<Result<T, T::Error>>
    where
        T: TryFromSV,
    {
        unsafe {
            self.st_fetch_raw(idx)
                .map(|svp| T::try_from_sv(self.perl, svp))
        }
    }

    /// Push value onto Perl stack.
    ///
    /// See: [`mXPUSHs`](http://perldoc.perl.org/perlapi.html#mXPUSHs).
    #[inline]
    pub fn st_push<T>(&mut self, val: T)
    where
        T: IntoSV,
    {
        let sv = val.into_sv(self.perl);
        unsafe {
            self.perl
                .ouroboros_stack_xpush_sv_mortal(&mut self.stack, sv.into_raw())
        };
    }

    // XSUB

    /// Register new Perl xsub.
    ///
    /// See: [`newXS`](http://perldoc.perl.org/perlapi.html#newXS).
    #[inline]
    pub fn new_xs(&mut self, name: &CStr, xsaddr: raw::XSUBADDR_t) {
        unsafe { self.perl.newXS(name.as_ptr(), xsaddr, EMPTY.as_ptr()) };
    }

    // GLOBALS

    /// Return the AV of the specified Perl global or package array.
    ///
    /// See: [`get_av`](http://perldoc.perl.org/perlapi.html#get_av).
    #[inline]
    pub fn get_av(&mut self, name: &CStr) -> Option<AV> {
        let avp = unsafe { self.perl.get_av(name.as_ptr(), 0) };
        if avp.is_null() {
            None
        } else {
            Some(unsafe { AV::from_raw_borrowed(self.perl, avp) })
        }
    }

    /// Call subroutine by name.
    ///
    /// See: [`call_pv`](http://perldoc.perl.org/perlapi.html#call_pv).
    #[inline]
    pub fn call_pv(&mut self, name: &CStr, flags: raw::U32) {
        unsafe { self.perl.call_pv(name.as_ptr(), flags as raw::I32) };
    }

    // SCALARS

    /// Allocate new SV of type appropriate to store `T`
    #[inline]
    pub fn new_sv<T>(&mut self, val: T) -> SV
    where
        T: IntoSV,
    {
        val.into_sv(self.perl)
    }

    /// Create a new SV to store an arbitrary Rust value.
    ///
    /// This function returns a perl reference to a newly allocated SV, that has Rust value attached
    /// via [perl magic](http://perldoc.perl.org/perlguts.html#Magic-Variables).
    ///
    /// Value can be accessed via `SV::into_data_ref()` method or automatic conversions to
    /// `DataRef<T>`.
    ///
    /// ```
    /// # #[macro_use] extern crate perl_xs;
    /// # #[macro_use] extern crate perl_sys;
    /// # use std::cell::RefCell;
    /// # use perl_xs::{IV, DataRef};
    /// xs! {
    ///   package Counter;
    ///   sub new(ctx, class: String, initial: IV) {
    ///     ctx.new_sv_with_data(RefCell::new(initial)).bless(&class)
    ///   }
    ///   sub get(_ctx, this: DataRef<RefCell<IV>>) {
    ///     return *this.borrow();
    ///   }
    ///   sub inc(_ctx, this: DataRef<RefCell<IV>>, amount: Option<IV>) {
    ///     *this.borrow_mut() += amount.unwrap_or(1);
    ///   }
    /// }
    /// # fn main() {}
    /// ```
    #[inline]
    pub fn new_sv_with_data<T: 'static>(&mut self, value: T) -> SV {
        self.new_sv(Box::new(value) as Box<std::any::Any>)
    }

    /// Return an undefined SV.
    pub fn sv_undef(&mut self) -> SV {
        unsafe { SV::from_raw_owned(self.perl, self.perl.ouroboros_sv_undef()) }
    }
}

/// Push the value to the perl stack as one or more scalar values.
///
/// ```
/// # #[macro_use] extern crate perl_xs;
/// # #[macro_use] extern crate perl_sys;
/// # use perl_xs::context::Stackable;
/// # xs! {
/// #   package Dummy;
/// #   sub foo(ctx) {
/// // pushes one scalar
/// "question".push_to(ctx);
/// // pushes two scalars
/// ("question", "answer").push_to(ctx);
/// #   }
/// # }
/// # fn main() {}
/// ```
pub trait Stackable {
    /// Push self onto Perl stack as zero or more individual scalar values.
    fn push_to(self, ctx: &mut Context);
}

impl<T> Stackable for T
where
    T: IntoSV,
{
    #[inline]
    fn push_to(self, ctx: &mut Context) {
        ctx.st_push(self);
    }
}

macro_rules! impl_tuple {
    (= [$($n:tt $i:tt)*] [$($tails:tt)*]) => (
        impl<$($n: IntoSV),*> Stackable for ($($n,)*) {
            #[inline]
            #[allow(unused_variables)]
            fn push_to(self, ctx: &mut Context) {
                $( ctx.st_push(self.$i); )*
            }
        }
        impl_tuple!(> [$($n $i)*] [$($tails)*]);
    );

    (> [$($heads:tt)*] [$n:tt $i:tt $($tails:tt)*]) => (
        impl_tuple!(= [$($heads)* $n $i] [$($tails)*]);
    );

    (> [$($heads:tt)*] [] ) => ();
}

impl_tuple!(= [] [A 0 B 1 C 2 D 3 E 4 F 5 G 6 H 7 I 8 J 9]);
