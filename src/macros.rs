#[macro_export]
macro_rules! xs_return {
    ($ctx:ident, $( $val:expr ),*) => {{
        $ctx.st_prepush();
        $( $ctx.st_push($val); )*
        $ctx.st_putback();
        return;
    }}
}

#[macro_export]
macro_rules! XS {
    {
        package $pkg:ident { $( sub $name:ident ($ctx:ident) $body:block )* }
        loader $boot:ident;
    } => {
        $(
            #[allow(non_snake_case)]
            pub extern "C" fn $name (pthx: $crate::raw::PerlThreadContext,
                                     _cv: *mut $crate::raw::CV) {
                let mut $ctx = $crate::Context::new(&pthx);
                $body
            }
        )*

        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn $boot (pthx: $crate::raw::PerlThreadContext, _cv: *mut $crate::raw::CV) {
            let mut ctx = $crate::Context::new(&pthx);
            $({
                let fullname = cstr!(concat!(stringify!($pkg), "::", stringify!($name)));
                ctx.new_xs(&fullname, $name);
            })*

            xs_return!(ctx, 1 as $crate::raw::IV);
        }
    }
}

#[macro_export]
macro_rules! cstr {
    ($e:expr) => (&::std::ffi::CString::new($e).unwrap())
}
