extern crate proc_macro;  
extern crate syn;
#[macro_use]
extern crate quote;
extern crate perl_xs;

extern crate perlxs_derive_internals;
use perlxs_derive_internals as internals;

use proc_macro::TokenStream;  
use syn::{Ident, VariantData, Lit, StrStyle};

#[proc_macro_derive(FromPerlKV, attributes(perlxs))]
pub fn from_kv(input: TokenStream) -> TokenStream {

    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    // Build the impl
    let gen = impl_from_kv(&ast);

    //println!("Debug {}", gen.to_string());
    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_from_kv(ast: &syn::MacroInput) -> quote::Tokens {
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
        let ty       = &field.ty;
        let var      = Ident::new(format!("value_{}", ident));

        //println!( "Got field: {:?}", field );

        letvars.push(
            quote! {
                let mut #var : Option<#ty> = None
            }
        );

        if field.optional {
            paramvars.push(
                quote! {
                    #ident: #var
                }
            );
        }else{
            let err_omitted = field.err_omitted();

            // I'm not super in love with using expect for this.
            // There's got to be a better way
            paramvars.push(
                quote! {
                    #ident: #var.expect(#err_omitted)
                }
            );
        }

        for key in field.keys.iter(){
            let err_no_val     = field.err_no_val(key);
            let err_parse_fail = field.err_parse_fail(key);
            let keylit   = Lit::Str(key.to_string(),StrStyle::Cooked);

            matchparts.push(quote!{
                #keylit => {
                    let s_res = ctx.st_try_fetch::<#ty>(i+1).expect(#err_no_val);
                    let v = s_res.expect(#err_parse_fail);
                    #var = Some( v );
                }
            });
        }

    }

    let from_kv_stack = quote!{
        #(#letvars;)*
        let mut i = offset;
        while let Some(sv_res) = ctx.st_try_fetch::<String>(i) {
            match sv_res {
                Ok(key) => {
                    match &*key {
                        #(#matchparts,)*
                        &_ => {
                            // QUESTION: Unknown key. Should we warn?
                        }
                    }
                },
                Err(e) => {
                    panic!("paramter key is not a string {}", e);
                }
            }
            i += 2;
        };

        Self{
            #(#paramvars,)*
        }
    };
    
    let impl_block = quote! {
        impl #impl_generics _perlxs::FromPerlKV for #ident #ty_generics #where_clause {
            #vis fn from_perl_kv(ctx: &mut _perlxs::Context, offset: isize) -> Self
            {
                #from_kv_stack
            }
        }
    };

    let dummy_const = Ident::new(format!("_IMPL_PERLXS_FROMPERLKV_FOR_{}", ident));

    quote! {
        #[allow(non_upper_case_globals)]
        const #dummy_const: () = {
            extern crate perl_xs as _perlxs;
            #impl_block
        };
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