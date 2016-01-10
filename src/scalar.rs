use raw::*;
use handle;

pub trait Scalar {
    fn get_pthx(&self) -> PerlContext;
    fn get_raw_ptr(&self) -> *mut SV;

    fn to_iv(&self) -> IV {
        unsafe { ouroboros_sv_iv(self.get_pthx(), self.get_raw_ptr()) }
    }

    fn to_uv(&self) -> UV {
        unsafe { ouroboros_sv_uv(self.get_pthx(), self.get_raw_ptr()) }
    }
    
    fn copy(&self) -> handle::SV {
        unsafe { 
            let svp = Perl_newSVsv(self.get_pthx(), self.get_raw_ptr());
            handle::SV::new(self.get_pthx(), svp)
        }
    }
}
