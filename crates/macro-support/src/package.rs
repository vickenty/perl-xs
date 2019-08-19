use crate::error::Errors;
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub fn expand(input: TokenStream) -> Result<TokenStream, Errors> {
    let item = syn::parse2::<syn::Lit>(input.into()).unwrap();

    let package_name = match item {
        syn::Lit::Str(s) => s,
        _ => panic!("package macro requires string argument"),
    };

    let package_name_clean = package_name.value().replace("::", "__");
    let boot_fn_name = syn::Ident::new(&format!("boot_{}", package_name_clean), Span::call_site());

    let output = quote! {
        /// Register the package name with this module_path
        const _XS_PACKAGE_DEF: () = {
            #[ctor]
            fn package_def() {
            ::perl_xs::PACKAGE_REGISTRY.submit(::perl_xs::Package{ module: module_path!(), package: #package_name});
            }
        };

        /// Register a boot function for each package, which allows flexibility for how the library might be loaded
        #[no_mangle]
        #[allow(non_snake_case)]
        extern "C" fn #boot_fn_name (pthx: *mut ::perl_sys::types::PerlInterpreter, _cv: *mut ::perl_xs::raw::CV) {
            let perl = perl_xs::raw::initialize(pthx);
            perl_xs::context::Context::wrap(perl, |ctx| {

            ::perl_xs::boot::boot(ctx, #package_name);

            1 as perl_xs::raw::IV
            });
        }
    };

    Ok(output)
}
