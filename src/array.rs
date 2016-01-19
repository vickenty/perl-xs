use raw::*;
use handle::{ From, Temp };

pub trait Array {
    fn get_pthx(&self) -> PerlContext;
    fn get_raw_ptr(&self) -> *mut AV;

    fn fetch_raw(&self, idx: Size_t) -> *mut *mut SV {
        unsafe { Perl_av_fetch(self.get_pthx(), self.get_raw_ptr(), idx, 0) }
    }

    fn fetch<T>(&self, idx: Size_t) -> T where T: From<Option<Temp<SV>>> {
        let svpp = self.fetch_raw(idx);
        let temp = if svpp.is_null() {
            None
        } else {
            Some(Temp::new(self.get_pthx(), unsafe{ *svpp }))
        };
        T::from(temp)
    }
}
