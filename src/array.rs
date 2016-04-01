use raw::*;
use handle::{ From, Temp };

pub trait Array {
    fn get_pthx(&self) -> PerlContext;
    fn get_raw_ptr(&self) -> *mut AV;

    fn fetch_raw(&self, idx: Size_t) -> *mut *mut SV {
        unsafe { Perl_av_fetch(self.get_pthx(), self.get_raw_ptr(), idx, 0) }
    }

    fn fetch<T>(&self, idx: Size_t) -> Option<T> where T: From<Temp<SV>> {
        let svpp = self.fetch_raw(idx);
        if svpp.is_null() {
            None
        } else {
            let temp = Temp::new(self.get_pthx(), unsafe{ *svpp });
            Some(T::from(temp))
        }
    }
}
