macro_rules! expr {
    ( $e:expr ) => {
        $e
    };
}

macro_rules! method {
    ($( #[$me:meta] )* simple fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) = $imp:ident ( $( $args:expr ),* ) $( $rest:tt )* ) => (
        $( #[$me] )*
        #[inline]
        pub fn $name(&self, $( $pname : $ptype ),* ) {
            unsafe { self.pthx().$imp(self.as_ptr(), $( $args ),*) };
        }
    );

    ($( #[$me:meta] )* simple fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) -> $rtype:ty = $imp:ident ( $( $args:expr ),* ) $( $rest:tt )* ) => (
        $( #[$me] )*
        #[inline]
        pub fn $name(&self, $( $pname : $ptype ),* ) -> $rtype {
            expr! { unsafe { self.pthx().$imp(self.as_ptr(), $( $args ),*) $( $rest )* } }
        }
    );

    ($( #[$me:meta] )* getter fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) = $imp:ident ( $( $args:expr ),* ) ) => (
        $( #[$me] )*
        #[inline]
        pub fn $name<T>(&self, $( $pname : $ptype ),* ) -> Option<T> where T: $crate::convert::FromSV {
            let svp = unsafe { self.pthx().$imp(self.as_ptr(), $( $args ),*) };
            if !svp.is_null() { Some(unsafe { T::from_sv(self.pthx(), svp) }) } else { None }
        }
    );

    ($( #[$me:meta] )* getptr fn $name:ident ( $( $pname:ident : $ptype:ty ),* ) = $imp:ident ( $( $args:expr ),* ) ) => (
        $( #[$me] )*
        #[inline]
        pub fn $name<T>(&self, $( $pname : $ptype ),* ) -> Option<T> where T: $crate::convert::FromSV {
            let svpp = unsafe { self.pthx().$imp(self.as_ptr(), $( $args ),*) };
            if !svpp.is_null() { Some(unsafe { T::from_sv(self.pthx(), *svpp) }) } else { None }
        }
    );
}
