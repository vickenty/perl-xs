/// Define Perl modules and packages.
///
/// First form of this macro is used to define a Perl package inside a module. Each invocation
/// should contain only one package and there should be only one such invocation per Rust module.
///
/// ```
/// #[macro_use] extern crate perl_xs;
/// #[macro_use] extern crate perl_sys;
/// mod acme {
///     xs! {
///         package Acme;
///         sub foo(ctx) { /* code */ }
///     }
/// }
/// # fn main() {}
/// ```
///
/// Second form is used to generate bootstrap function used by Perl to intialize XS module. Each
/// crate should contain exactly one invocation in this form:
///
/// ```
/// #[macro_use] extern crate perl_xs;
/// #[macro_use] extern crate perl_sys;
/// # mod acme { xs! { package Acme; } }
/// xs! {
///     bootstrap boot_Acme;
///     use acme;
/// }
/// # fn main() {}
/// ```
///
/// Function name given to `bootstrap` keyword must start with `boot_` followed by the Perl module
/// name.
#[macro_export]
macro_rules! xs {
    (
        package $pkg:path ;
        $( sub $name:ident ($ctx:ident $(, $par:ident : $pty:ty )* ) $body:block )*
    ) => (
        $(
            pthx! {
                #[allow(unused_mut)]
                fn $name (pthx, _cv: *mut $crate::raw::CV) {
                    let perl = $crate::raw::initialize(pthx);
                    $crate::context::Context::wrap(perl, |mut $ctx| {
                        let mut _arg = 0;
                        $(
                            let $par = match $ctx.st_try_fetch::<$pty>(_arg) {
                                Some(Ok(v)) => v,
                                Some(Err(e)) =>
                                    croak!(
                                        concat!(
                                            "invalid argument '",
                                            stringify!($par),
                                            "' for ",
                                            stringify!($pkg),
                                            "::",
                                            stringify!($name),
                                            ": {}"),
                                        e),
                                None =>
                                    croak!(
                                        concat!(
                                            "not enough arguments for ",
                                            stringify!($pkg),
                                            "::",
                                            stringify!($name))),
                            };
                            _arg += 1;
                        )*
                        $body
                    });
                }
            }
        )*

        const _XS_PACKAGE_DEF_DEPRECATED: () = {
            #[ctor]
            fn boot() {
                $(
                    (
                     ::perl_xs::SYMBOL_REGISTRY.submit(::perl_xs::Symbol{
                        module: module_path!(),
                        name: stringify!($name),
                        package: Some(stringify!($pkg)),
                        ptr: $name as $crate::raw::XSUBADDR_t
                        })
                    )
                );*
            }

        };
    );

    (
        bootstrap $boot:ident;
        $( use $( $name:ident )::+ ; )*
    ) => (
        pthx! {
            #[no_mangle]
            #[allow(non_snake_case)]
            fn $boot (pthx, _cv: *mut $crate::raw::CV) {
                let perl = $crate::raw::initialize(pthx);
                $crate::context::Context::wrap(perl, |ctx| {
                    ::perl_xs::boot::boot(ctx, stringify!($boot).splitn(2, "_").nth(1).expect("bootstrap directive must be in the format of boot_PackageName"));

                    1 as $crate::raw::IV
                });
            }
        }
    );
}

/// Throw a perl exception.
///
/// Perl exceptions are implemented as panics in Rust, but do not call the panic hook - user
/// programs may handle the exception, in which case the panic message printed by the hook may be an
/// unwelcome interruption.
///
/// The single argument form can throw any kind of object, although only `String` and `&str` will be
/// passed to perl.
///
/// The multi-argument form throws a string formatted using the standard formatting syntax.
#[macro_export]
macro_rules! croak {
    ($msg:expr) => ({
        $crate::croak::croak($msg)
    });

    ($fmt:expr, $($arg:tt)*) => ({
        $crate::croak::croak_fmt(&format_args!($fmt, $($arg)*))
    });
}
