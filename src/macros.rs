#[macro_export]
macro_rules! xs_return {
    ($ctx:ident, $( $val:expr ),*) => {{
        $ctx.st_prepush();
        $( $ctx.st_push($val); )*
        $ctx.st_putback();
        return;
    }}
}

/// Define Perl modules and packages.
///
/// First form of this macro is used to define a Perl package inside a module. Each invocation
/// should contain only one package and there should be only one such invocation per Rust module.
///
/// ```
/// mod acme {
///     xs! {
///         package Acme;
///         sub foo(ctx) { /* code */ }
///     }
/// }
/// ```
///
/// Second form is used to generate bootstrap function used by Perl to intialize XS module. Each
/// crate should contain exactly one invocation in this form:
///
/// ```
/// xs! {
///     bootstrap boot_Acme;
///     use acme;
/// }
/// ```
///
/// Function name given to `bootstrap` keyword must start with `boot_` followed by the module name.
#[macro_export]
macro_rules! xs {
    (
        package $pkg:ident ;
        $( sub $name:ident ($ctx:ident) $body:block )*
    ) => (
        $(
            extern "C" fn $name (pthx: $crate::raw::PerlThreadContext,
                                 _cv: *mut $crate::raw::CV) {
                let mut $ctx = $crate::Context::new(&pthx);
                $body
            }
        )*

        pub const PERL_XS: &'static [ (&'static str, $crate::raw::XSUBADDR_t) ] = &[
            $(
                (
                    concat!(stringify!($pkg), "::", stringify!($name)),
                    $name as $crate::raw::XSUBADDR_t,
                )
            ),*
        ];
    );

    (
        bootstrap $boot:ident;
        $( use $( $name:ident )::+ ; )*
    ) => (
        #[no_mangle]
        #[allow(non_snake_case)]
        pub extern "C" fn $boot (pthx: $crate::raw::PerlThreadContext,
                                 _cv: *mut $crate::raw::CV) {
            let mut ctx = $crate::Context::new(&pthx);
            $(
                for &(subname, subptr) in $( $name )::*::PERL_XS {
                    let cname = ::std::ffi::CString::new(subname).unwrap();
                    ctx.new_xs(&cname, subptr);
                }
            )*

            xs_return!(ctx, 1 as $crate::raw::IV);
        }
    );
}

#[macro_export]
macro_rules! cstr {
    ($e:expr) => (&::std::ffi::CString::new($e).unwrap())
}
