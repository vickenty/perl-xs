#![feature(proc_macro)]

extern crate proc_macro;  
extern crate syn;
#[macro_use]
extern crate quote;

extern crate perlxs_derive_internals;
use perlxs_derive_internals as internals;

use proc_macro::TokenStream;  
use syn::{Ident, VariantData};

#[proc_macro_derive(FromKeyValueStack, attributes(perlxs))]
pub fn from_kv_stack(input: TokenStream) -> TokenStream {

    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    // Build the impl
    let gen = impl_from_kv_stack(&ast);
    println!("Meow {}", gen.to_string());
    TokenStream::empty()
    // Return the generated impl
    //gen.parse().unwrap()
}

fn impl_from_kv_stack(ast: &syn::MacroInput) -> quote::Tokens {
    let vis = &ast.vis;
    let ident = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // create a vector containing the names of all fields on the struct
    //    let dummy_const = Ident::new(format!("_IMPL_DESERIALIZE_FOR_{}", ident));

    let errors = internals::error::Errors::new();

    let fields = match ast.body {
        syn::Body::Struct(ref vdata) => {
            match vdata {
                &VariantData::Struct(ref fields) => {
                    internals::ast::fields_from_ast(&errors, fields)
                },
                &VariantData::Tuple(_) | &VariantData::Unit => {
                    panic!("You can only derive this for normal structs!");
                },
            }
        },
        syn::Body::Enum(_) => panic!("You can only derive this on structs!"),
    };

    errors.check().unwrap();

    let mut letvars = Vec::new();
    let mut paramvars = Vec::new();
    let mut matchparts = Vec::new();

    for field in fields.iter(){
        let ident    = &field.ident;
        let identval = ident.clone();
        let ty       = &field.ty;
        let var      = Ident::new(format!("value_{}", ident));

        println!( "Got key attribute: {:?}", field );

        letvars.push(
            quote! {
                let mut #identval = #ty::default();
            }
        );

        paramvars.push(
            quote! {
                #ident: #identval,
            }
        );

        for key in field.keys.iter(){
            matchparts.push(quote!{
                #key => {
                    let s_res = ctx.st_try_fetch::<#ty>(i+1).expect("no argument provided for parameter \"#ident\"");
                    let v = s_res.expect("parameter #ident unable to be interpreted as a string");
                    #var = Some( v );
                }
            })
        }

    }

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #vis fn from_kv_stack(ctx: &mut Context, offset: isize) -> Self
            {
                //define vars
                #(#letvars;)*

                while let Some(sv_res) = ctx.st_try_fetch::<String>(i) {
                    match sv_res {
                        Ok(key) => { 
                            match &*key {
                                #(#matchparts,)*
                            }
                        },
                        Err(e) => {
                            panic!("paramter key is not a string {}", e);
                        }
                    }
                };

                Self{
                    #(#paramvars,)*
                }
            }
        }
    }
}

// impl FromKeyValueStack for DBRBuilder {

//     fn from_kv_stack ( ctx: &mut Context, offset: isize ) -> Self {

//         let mut logger : Option<String> = None;
//         let mut conf   : Option<String> = None;
//         let mut admin    = false;
//         let mut fudge_tz = false;

//         let mut i = offset;

//         while let Some(sv_res) = ctx.st_try_fetch::<String>(i) {
//             match sv_res {
//                 Ok(key) => { 
//                     match &*key {
//                         "-logger" => {
//                             let s_res = ctx.st_try_fetch::<String>(i+1).expect("no argument provided for parameter \"{}\"");
//                             let v = s_res.expect("parameter {} unable to be interpreted as a string");
//                             logger = Some( v );
//                         }
//                         "-conf"   => {
//                             let s_res = ctx.st_try_fetch::<String>(i+1).expect("no argument provided for parameter \"{}\"");
//                             let v = s_res.expect("parameter {} unable to be interpreted as a string");
//                             conf = Some( v );
//                         }
//                         "-admin" => {
//                             let s_res = ctx.st_try_fetch::<bool>(i+1).expect("no argument provided for parameter \"{}\"");
//                             let v = s_res.expect("parameter {} unable to be interpreted as a bool");
//                             admin = v;
//                         }
//                         "-fudge_tz" => {
//                             let s_res = ctx.st_try_fetch::<bool>(i+1).expect("no argument provided for parameter \"{}\"");
//                             let v = s_res.expect("parameter {} unable to be interpreted as a bool");
//                             fudge_tz = v;
//                         },
//                         _ => {
//                             panic!("unsupported parameter {}",key);
//                         }
//                     }
//                 },
//                 Err(e) => {
//                     panic!("paramter key is not a string {}", e);
//                 }
//             }

//             i += 2;
//         }

//         Self{
//             use_exceptions: true,
//             app:            None,
//             conf:           conf,
//             logpath:        None,
//             loglevel:       None,
//             logger:         logger,
//             admin:          admin,
//             fudge_tz:       fudge_tz,
//         }
//     }
// }