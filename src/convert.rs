use raw;
use SV;

pub trait FromSV {
    unsafe fn from_sv(pthx: raw::Interpreter, raw: *mut raw::SV) -> Self;
}

pub trait IntoSV {
    fn into_sv(self, pthx: raw::Interpreter) -> SV;
}
