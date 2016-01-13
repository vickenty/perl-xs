use raw;
use handle::*;

pub trait Array {
    fn get_pthx(&self) -> raw::PerlContext;
    fn get_raw_ptr(&self) -> *mut raw::AV;

    fn fetch_raw(&self, idx: raw::Size_t) -> *mut *mut raw::SV {
        unsafe { raw::Perl_av_fetch(self.get_pthx(), self.get_raw_ptr(), idx, 0) }
    }

    fn fetch<T>(&self, idx: raw::Size_t) -> T where T: From<Option<Temp<raw::SV>>> {
        let svpp = self.fetch_raw(idx);
        let temp = if svpp.is_null() {
            None
        } else {
            Some(Temp::new(self.get_pthx(), unsafe{ *svpp }))
        };
        T::from(temp)
    }
}
