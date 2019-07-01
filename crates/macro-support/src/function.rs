
use proc_macro2::TokenStream;
use crate::error::Errors;
use quote::quote;

/// Takes the parsed input from a `#[perlxs]` macro and returns the generated bindings
pub fn expand(attr: TokenStream, input: TokenStream) -> Result<TokenStream, Errors> {

    let item = syn::parse2::<syn::Item>(input.clone())?;

    match item {
        syn::Item::Fn(f) => {
           expand_function(f)
        },
        _ => panic!("cannot expand macro for non-function")
    }
}

fn expand_function (f: syn::ItemFn ) -> Result<TokenStream,Errors>{

    let rust_fn_ident = f.ident.clone();
    let rust_fn_name = format!("{}",f.ident);

    // TODO generate this from module name, overridable with attribute
    // let perl_fn_name: String = format!("XSTest::Derive::{}", rust_fn_name);
    //let boot_pkg: String = perl_fn_name.split("::").next().unwrap().to_owned();

    let xs_name = syn::Ident::new(&format!("_xs_{}",rust_fn_name),f.ident.span());

    let (impl_generics, ty_generics, where_clause) = f.decl.generics.split_for_impl();

    let errors = crate::error::Errors::new();

    let mut rust_arg_unpacks = Vec::new();
    let mut rust_args = Vec::new();

    for arg in f.decl.inputs.iter(){
        println!("{:?}", arg);
        match arg {
            syn::FnArg::SelfRef(_) => {
                //TODO: determine how to implement a proxy struct for perl objects
                //      Does it entail automatic implementation of a Context trait + automatic struct instantiation?
                unimplemented!()
            },
            syn::FnArg::SelfValue(_) => {
                //TODO: determine if this is appropriate to implement
                unimplemented!()
            }
            syn::FnArg::Captured(c) => {
                if let syn::Pat::Ident(syn::PatIdent{ ident: ref arg_ident , .. }) = c.pat {
                    let var = syn::Ident::new(&format!("value_{}", arg_ident), proc_macro2::Span::call_site());

                    let rust_arg_name = format!("{}", arg_ident);
                    let rust_arg_type = &c.ty;

                    let fetch = quote! {
                        let #var = match <#rust_arg_type as TryFromContext>::try_from_context(_xs_ctx, #rust_arg_name, &mut offset){
                              Ok(v)  => v,
//                              Err(e) => croak!(format!("{} for {}",e, #perl_fn_name)),
                              Err(e) => croak!(format!("{}",e)),
                        }
                    };

                    rust_arg_unpacks.push(fetch );
                    rust_args.push( var );
                }

            },
            syn::FnArg::Inferred(_) => {
                unimplemented!()
            },
            syn::FnArg::Ignored(_) => {

            }
        }
    }

    errors.check().unwrap();

    let dummy_const = syn::Ident::new(&format!("_IMPL_PERLXS_FOR_{}", rust_fn_name),proc_macro2::Span::call_site());


    let bind_fn = quote!{
        extern "C" fn #xs_name (pthx: *mut ::perl_sys::types::PerlInterpreter, _cv: *mut ::perl_xs::raw::CV) {

            let perl = ::perl_xs::raw::initialize(pthx);
            ::perl_xs::context::Context::wrap(perl,|mut _xs_ctx| {

                let mut offset : isize = 0;
                #(#rust_arg_unpacks;)*
                #rust_fn_ident(#(#rust_args,)*)
            });

        }
    };

    let bind = quote!{
        #[allow(non_upper_case_globals)]
        const #dummy_const: () = {
            #[macro_use]
            use perl_xs::*;

            #bind_fn

            // Run at library load time
            #[ctor]
            fn bootstrap() {
                let path = module_path!();
                println!("MODULE PATH {}", path);
                ::perl_xs::SYMBOL_REGISTRY.submit(Symbol{ name: #rust_fn_name, ptr: #xs_name});
            }
        };
    };


    let output = quote!{

        #f

        #bind

    };

    Ok(output)
}