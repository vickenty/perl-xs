#[macro_export]
macro_rules! XS {
    {
        package $pkg:ident { $( sub $name:ident ($ctx:ident) $body:block )* }
        loader $boot:ident;
    } => {
        $(
            #[allow(non_snake_case)]
            pub extern "C" fn $name (pthx: $crate::raw::PerlContext,
                                     _cv: *mut $crate::raw::CV) {
                let mut $ctx = $crate::Context::new(pthx);
                $body
            }
        )*

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn $boot (pthx: $crate::raw::PerlContext, _cv: *mut $crate::raw::CV) {
            let mut ctx = $crate::Context::new(pthx);
            $({
                let fullname = concat!(stringify!($pkg), "::", stringify!($name));
                ctx.new_xs(&fullname, $name);
            })*
        }
    }
}
