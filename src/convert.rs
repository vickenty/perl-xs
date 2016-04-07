use raw;
use SV;

pub trait FromRaw<T: ?Sized> {
    unsafe fn from_raw(pthx: raw::Interpreter, raw: *mut T) -> Self;
}

pub trait IntoSV {
    fn into_sv(self, pthx: raw::Interpreter) -> SV;
}
