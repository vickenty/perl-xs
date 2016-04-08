use std;
use raw;
use { IV, AV };
use convert::{ FromRaw, IntoSV };
use std::ffi::CStr;

pub struct Context<'a> {
    pthx: raw::Interpreter,
    stack: raw::Stack,
    marker: std::marker::PhantomData<&'a raw::PerlThreadContext>
}

impl<'a> Context<'a> {
    pub fn new(pthx: &'a raw::PerlThreadContext) -> Self {
        unsafe {
            let mut ctx = Context {
                pthx: raw::Interpreter::new(*pthx),
                stack: std::mem::uninitialized(),
                marker: std::marker::PhantomData,
            };

            ctx.pthx.st_init(&mut ctx.stack);

            ctx
        }
    }

    // STACK

    pub fn st_prepush(&mut self) {
        unsafe { self.pthx.st_prepush(&mut self.stack) };
    }

    pub fn st_putback(&mut self) {
        unsafe { self.pthx.st_putback(&mut self.stack) };
    }

    pub fn st_fetch<T>(&mut self, idx: isize) -> T where
        T: FromRaw<raw::SV>
    {
        /* FIXME: panic if idx > items */
        let svp = unsafe { self.pthx.st_fetch(&mut self.stack, idx as raw::SSize_t) };
        unsafe { T::from_raw(self.pthx, svp) }
    }

    pub fn st_push<T>(&mut self, val: T) where T: IntoSV {
        let sv = val.into_sv(self.pthx);
        unsafe { self.pthx.st_push(&mut self.stack, sv.into_raw()) };
    }

    // XSUB

    pub fn new_xs(&mut self, name: &CStr, xsaddr: raw::XSUBADDR_t) {
        let filename = 0i8;
        unsafe { self.pthx.new_xs(name.as_ptr(), xsaddr, &filename) };
    }

    // GLOBALS

    pub fn get_av(&mut self, name: &CStr) -> Option<AV> {
        let avp = unsafe { self.pthx.get_av(name.as_ptr()) };
        if avp.is_null() {
            None
        } else {
            Some(unsafe { AV::from_raw_borrowed(self.pthx, avp) })
        }
    }

    pub fn call_pv(&mut self, name: &CStr, flags: IV) {
        unsafe { self.pthx.call_pv(name.as_ptr(), flags as raw::I32) };
    }
}
