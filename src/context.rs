use std;
use std::os::raw::c_char;
use std::ffi::CStr;

use raw::*;
use pushable::Pushable;
use scalar::{ Scalar };
use handle;
use handle::From;

pub struct Context<'a> {
    pthx: PerlContext,
    stack: OuroborosStack,
    marker: std::marker::PhantomData<&'a PerlContext>
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

impl<'a> Context<'a> {
    pub fn new(pthx: &'a PerlContext) -> Self {
        unsafe {
            let mut ctx = Context {
                pthx: *pthx,
                stack: std::mem::uninitialized(),
                marker: std::marker::PhantomData,
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

    //

    wrapper! { st_prepush: ouroboros_stack_prepush -stack () }
    wrapper! { st_putback: ouroboros_stack_putback -stack () }
    wrapper! { st_extend: ouroboros_stack_extend -stack (len: Size_t) }

    pub fn st_push<T>(&mut self, value: T) where T: Pushable {
        value.push_extend(self.pthx, &mut self.stack);
    }

    pub unsafe fn st_push_unsafe<T>(&mut self, value: T) where T: Pushable {
        value.push_unsafe(self.pthx, &mut self.stack);
    }

    //

    fn new_temp<T>(&mut self, ptr: *mut T) -> handle::Temp<T> {
        handle::Temp::new(self.pthx, ptr)
    }

    // Stack ops

    pub unsafe fn st_fetch_raw(&mut self, idx: SSize_t) -> *mut SV {
        ouroboros_stack_fetch(self.pthx, &mut self.stack, idx)
    }

    pub fn st_fetch<T>(&mut self, idx: SSize_t) -> T where T: From<handle::Temp<SV>> {
        let svp = unsafe { self.st_fetch_raw(idx) };
        T::from(self.new_temp(svp))
    }

    // GV ops

    pub fn get_av<T>(&mut self, name: &CStr) -> T where T: From<Option<handle::Temp<AV>>> {
        let avp = unsafe { Perl_get_av(self.pthx, name.as_ptr(), 0) };
        let opt = if avp.is_null() { None } else { Some(self.new_temp(avp)) };

        T::from(opt)
    }

    // SV ops
    pub fn sv_iv(&mut self, sv: handle::BareSV) -> IV {
        handle::SV::from_bare(self.pthx, sv).to_iv()
    }
}
