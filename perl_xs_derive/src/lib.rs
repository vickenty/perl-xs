extern crate proc_macro;  
extern crate syn;  
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;  
use syn::{Ident, VariantData};

#[proc_macro_derive(FromKeyValueStack)]
pub fn from_kv_stack(input: TokenStream) -> TokenStream {  
    // Construct a string representation of the type definition
//    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&input).unwrap();

    // Build the impl
    let gen = impl_from_kv_stack(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_from_kv_stack(ast: &syn::MacroInput) -> quote::Tokens {
    let vis = &input.vis;
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // create a vector containing the names of all fields on the struct
    
    let fields: Vec<Field> = match ast.body {
        syn::Body::Struct(vdata) => {
            match vdata {
                VariantData::Struct(fields) => {
                    fields
                },
                VariantData::Tuple(_) | VariantData::Unit => {
                    panic!("You can only derive this for normal structs!");
                },
            }
        },
        syn::Body::Enum(_) => panic!("You can only derive this on structs!"),
    };

    // match &field.ident {
    //     &Some(ref ident) => idents.push(ident.clone()),
    //     &None => panic!("Your struct is missing a field identity!"),
    // }

    let mut let_vars = Vec::new();
    let mut param_vars = Vec::new();

    for field in fields.iter(){
        let ident = &f.ident;
        let ty = &f.ty;

        let_vars.push(
            quote! {
                let mut v__#ident = #ty::default();
            }
        );
        param_vars.push(
            quote! {
                #ident: v__#ident,
            }
        );

        match_parts.push(quote!{
            "#ident" => {
                let s_res = ctx.st_try_fetch::<#ty>(i+1).expect("no argument provided for parameter \"#ident\"");
                let v = s_res.expect("parameter #ident unable to be interpreted as a string");
                v__#ident = Some( v );
            }
        })
    }

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #vis fn from_kv_stack(ctx: &mut Context, offset: isize) -> Self
            {
                //define vars
                #(#let_vars;)*

                while let Some(sv_res) = ctx.st_try_fetch::<String>(i) {
                    match sv_res {
                        Ok(key) => { 
                            match &*key {
                                #(
                                    #matchparts,
                                )*
                            }
                        },
                        Err(e) => {
                            panic!("paramter key is not a string {}", e);
                        }
                    }
                };

                Self{
                    #(
                        #param_vars,
                    )*
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