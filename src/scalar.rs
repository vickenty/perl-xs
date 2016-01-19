use raw::*;
use handle::{ Full };

pub trait Scalar {
    fn get_pthx(&self) -> PerlContext;
    fn get_raw_ptr(&self) -> *mut SV;

    fn to_iv(&self) -> IV {
        unsafe { ouroboros_sv_iv(self.get_pthx(), self.get_raw_ptr()) }
    }

    fn to_uv(&self) -> UV {
        unsafe { ouroboros_sv_uv(self.get_pthx(), self.get_raw_ptr()) }
    }
    
    fn copy(&self) -> Full<SV> {
        unsafe { 
            let svp = Perl_newSVsv(self.get_pthx(), self.get_raw_ptr());
            Full::new(self.get_pthx(), svp)
        }
    }
}
