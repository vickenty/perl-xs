macro_rules! expr { ( $e:expr ) => ( $e ) }

macro_rules! method {
    (simple fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) = $imp:ident ( $( $args:expr ),* ) $( $rest:tt )* ) => (
        pub fn $name(&self, $( $pname : $ptype ),* ) {
            unsafe { self.pthx().$imp(self.as_ptr(), $( $args ),*) };
        }
    );

    (simple fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) -> $rtype:ty = $imp:ident ( $( $args:expr ),* ) $( $rest:tt )* ) => (
        pub fn $name(&self, $( $pname : $ptype ),* ) -> $rtype {
            expr! { unsafe { self.pthx().$imp(self.as_ptr(), $( $args ),*) $( $rest )* } }
        }
    );

    (getter fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) = $imp:ident ( $( $args:expr ),* ) ) => (
        pub fn $name<T>(&self, $( $pname : $ptype ),* ) -> Option<T> where T: $crate::convert::FromSV {
            let svp = unsafe { self.pthx().$imp(self.as_ptr(), $( $args ),*) };
            if !svp.is_null() { Some(unsafe { T::from_sv(self.pthx(), svp) }) } else { None }
        }
    );

    (getptr fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) = $imp:ident ( $( $args:expr ),* ) ) => (
        pub fn $name<T>(&self, $( $pname : $ptype ),* ) -> Option<T> where T: $crate::convert::FromSV {
            let svpp = unsafe { self.pthx().$imp(self.as_ptr(), $( $args ),*) };
            if !svpp.is_null() { Some(unsafe { T::from_sv(self.pthx(), *svpp) }) } else { None }
        }
    );
}
