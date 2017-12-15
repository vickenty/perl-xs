extern crate perl_xs;
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

extern crate perlxs_derive_internals;
use perlxs_derive_internals as internals;

use proc_macro::TokenStream;
use syn::{Ident, Lit, StrStyle, VariantData};

#[proc_macro_derive(FromPerlKV, attributes(perlxs))]
pub fn from_kv(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input(&input.to_string()).unwrap();

    // Build the impl
    let gen = impl_from_kv(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_from_kv(ast: &syn::MacroInput) -> quote::Tokens {
    let ident = &ast.ident;
    let ident_lit = Lit::Str(ast.ident.to_string(), StrStyle::Cooked);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let errors = internals::error::Errors::new();

    let fields = match ast.body {
        syn::Body::Struct(ref vdata) => match vdata {
            &VariantData::Struct(ref fields) => internals::ast::fields_from_ast(&errors, fields),
            &VariantData::Tuple(_) | &VariantData::Unit => {
                panic!("You can only derive this for normal structs!");
            }
        },
        syn::Body::Enum(_) => panic!("You can only derive this on structs!"),
    };

    errors.check().unwrap();

    let mut letvars = Vec::new();
    let mut matchparts = Vec::new();
    let mut paramtests = Vec::new();
    let mut paramvars = Vec::new();

    for field in fields.iter() {
        let ident = &field.ident;
        let ty = &field.ty;
        let var = Ident::new(format!("value_{}", ident));
        let ty_lit = Lit::Str(quote!{#ty}.to_string(), StrStyle::Cooked);

        #[allow(unused_variables)]
        let keys_lit: Vec<_> = field
            .keys
            .iter()
            .map(|k| Lit::Str(k.to_string(), StrStyle::Cooked))
            .collect();

        letvars.push(quote! {
            let mut #var : Option<#ty> = None
        });

        for key in field.keys.iter() {
            let key_lit = Lit::Str(key.to_string(), StrStyle::Cooked);

            matchparts.push(quote!{
                #key_lit => {
                    match ctx.st_try_fetch::<#ty>(i+1) {
                        Some(Ok(v))  => {
                            #var = Some( v );
                        },
                        Some(Err(e)) => {
                            errors.push(_perlxs::error::ToStructErrPart::ValueParseFail{key: #key_lit, ty: #ty_lit, error: e.to_string(), offset: i+1});
                        },
                        None         => {
                            errors.push(_perlxs::error::ToStructErrPart::OmittedValue(#key_lit));
                        },
                    }
                }
            });
        }

        if field.optional {
            paramvars.push(quote! {
                #ident: #var
            });
        } else {
            paramtests.push(quote!{
                if #var.is_none() {
                    errors.push(_perlxs::error::ToStructErrPart::OmittedKey(&[#(#keys_lit),*]));
                };
            });

            paramvars.push(quote! {
                #ident: #var.unwrap()
            });
        }
    }

    let from_kv_stack = quote!{

        let mut i = offset;
        while let Some(sv_res) = ctx.st_try_fetch::<String>(i) {
            match sv_res {
                Ok(key) => {
                    match &*key {
                        #(#matchparts,)*
                        &_ => {
                            // TODO: Warn for unknown key.
                        }
                    }
                },
                Err(e) => {
                    errors.push(
                        _perlxs::error::ToStructErrPart::KeyParseFail{
                            offset: i,
                            ty: "String",
                            error: e.to_string()
                        });
                }
            }
            i += 2;
        };

    };

    let impl_block = quote! {
        impl #impl_generics _perlxs::FromPerlKV for #ident #ty_generics #where_clause {
            fn from_perl_kv(ctx: &mut _perlxs::Context, offset: isize) -> Result<Self,_perlxs::error::ToStructErr>
            {
                let mut errors = Vec::new();
                #(#letvars;)*
                #from_kv_stack
                #(#paramtests;)*

                if errors.len() > 0 {
                    return Err(_perlxs::error::ToStructErr{
                        name: #ident_lit,
                        errors: errors
                    });
                }

                Ok(Self{
                    #(#paramvars,)*
                })
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
