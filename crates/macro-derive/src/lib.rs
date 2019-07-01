#![recursion_limit="128"]
extern crate proc_macro;

#[macro_use]
extern crate quote;
extern crate syn;

use perl_xs_macro_support as support;

use syn::{Ident, DeriveInput, parse_macro_input};

#[proc_macro_derive(DeriveTryFromContext, attributes(perlxs))]
pub fn from_kv(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let output = impl_from_kv(&ast);

    // Return the generated impl and convert from proc_macro2::TokenStream
    output.into()
}

fn impl_from_kv(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let ident = &ast.ident;
    let ident_lit = proc_macro2::Literal::string(&ast.ident.to_string());

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let errors = support::error::Errors::new();

    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct{  fields: syn::Fields::Named(syn::FieldsNamed{ ref named, .. }), .. }) => {
            support::ast::fields_from_ast(&errors, named.iter().map(|n| n.to_owned() ).collect())
        },
        _ => {
            panic!("You can only derive this for normal structs!");
        }
    };

    errors.check().unwrap();

    let mut letvars = Vec::new();
    let mut matchparts = Vec::new();
    let mut paramtests = Vec::new();
    let mut paramvars = Vec::new();

    for field in fields.iter() {
        let ident = &field.ident;
        let ty = &field.ty;
        let var = Ident::new(&format!("value_{}", ident), proc_macro2::Span::call_site());
        let ty_lit = proc_macro2::Literal::string(&quote!{#ty}.to_string());

        #[allow(unused_variables)]
        let keys_lit: Vec<_> = field
            .keys
            .iter()
            .map(|k| proc_macro2::Literal::string(&k.to_string()))
            .collect();

        letvars.push(quote! {
            let mut #var : Option<#ty> = None
        });

        for key in field.keys.iter() {
            let key_lit = proc_macro2::Literal::string(&key.to_string());

            matchparts.push(quote!{
                #key_lit => {
                    match ctx.st_try_fetch::<#ty>(*offset + 1) {
                        Some(Ok(v))  => {
                            #var = Some( v );
                        },
                        Some(Err(e)) => {
                            errors.push(_perlxs::error::ToStructErrPart::ValueParseFail{key: #key_lit, ty: #ty_lit, error: e.to_string(), offset: *offset + 1});
                        },
                        None         => {
                            errors.push(_perlxs::error::ToStructErrPart::OmittedValue(#key_lit));
                        },
                    };

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

        while let Some(sv_res) = ctx.st_try_fetch::<String>(*offset) {
            println!("Offset {} = [{:?}]", offset, sv_res);
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
                            offset: *offset,
                            ty: "String",
                            error: e.to_string()
                        });
                }
            }
            *offset += 2;
        };

    };

    // TODO check to see if we can use $crate here or not
    let impl_block = quote! {

        impl <'a> _perlxs::TryFromContext<'a> for #ident #ty_generics #where_clause {
            type Error = _perlxs::error::ToStructErr;

            fn try_from_context<'b: 'a>(ctx: &'b mut _perlxs::Context, _name: &str, offset: &mut isize) -> Result<Self,Self::Error>
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

    let dummy_const = Ident::new(&format!("_IMPL_PERLXS_FROMPERLKV_FOR_{}", ident),proc_macro2::Span::call_site());

    quote! {
        #[allow(non_upper_case_globals)]
        const #dummy_const: () = {
            extern crate perl_xs as _perlxs;
            #impl_block
        };
    }
}
