extern crate proc_macro;

use proc_macro::TokenStream;
use perl_xs_macro_support as support;
use proc_macro2::Span;
use quote::quote;


#[proc_macro_attribute]
pub fn perlxs(attr: TokenStream, input: TokenStream) -> TokenStream {
    println!("ATTR: {}", attr);
    println!("INPUT: {}", input);

    match support::function::expand(attr.into(), input.into()) {
        Ok(tokens) => {
          //  if cfg!(feature = "debug_print_generated_code") {
                println!("{}", tokens);
          //  }
            tokens.into()
        }
        Err(_error) => panic!("Unknown expansion error"),
    }
}


#[proc_macro]
pub fn package(item: TokenStream) -> TokenStream {

    let item = syn::parse2::<syn::Item>(input.clone())?;

//    match item {
//        syn::Item::Fn(f) => {
//            expand_function(f)
//        },
//        _ => panic!("cannot expand macro for non-function")
//    }
    let boot = syn::Ident::new(&format!("boot_{}",name),m.ident.span());

    boot_, $pkg
    println!("ITEM: {}", item);
    quote! {

        const _XS_PACKAGE_DEF: () = {
            #[ctor]
            fn package_def() {
                ::perl_xs::PACKAGE_REGISTRY.submit(::perl_xs::Package{ module: module_path!(), package: $pkg});
            }
        };

        #[no_mangle]
        #[allow(non_snake_case)]
        // TODO concat this ident
        extern "C" fn #boot (*mut ::perl_sys::types::PerlInterpreter, _cv: *mut crate::raw::CV) {
            println!("BOOT");
            let perl = perl_xs::raw::initialize(pthx);
            perl_xs::context::Context::wrap(perl, |ctx| {

//                    let package_rewrites : Vec<(&'static str, &'static str)> = Vec::new();
//                    for (package, ptr) in perl_xs::PACKAGE_REGISTRY.iter() {
//                        // TODO
//                    }

                for (symbol, ptr) in perl_xs::SYMBOL_REGISTRY.iter() {
                    println!("BOOT - FOUND {}", symbol);
                    let cname = ::std::ffi::CString::new(symbol.to_owned()).unwrap();
                    ctx.new_xs(&cname, *ptr);
                }

                1 as perl_xs::raw::IV
            });
        }
    }.into()
}
