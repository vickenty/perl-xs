use crate::error::Errors;
use proc_macro2::TokenStream;
use quote::quote;

// Takes the parsed input from a `#[perlxs]` macro and returns the generated bindings
pub fn expand(input: TokenStream) -> Result<TokenStream, Errors> {
    let ast = syn::parse2::<syn::DeriveInput>(input.clone())?;

    let ident = &ast.ident;
    let ident_lit = proc_macro2::Literal::string(&ast.ident.to_string());

    let (_impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let errors = Errors::new();

    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) => crate::ast::fields_from_ast(&errors, named.iter().map(|n| n.to_owned()).collect()),
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
        let var = syn::Ident::new(&format!("value_{}", ident), proc_macro2::Span::call_site());
        let ty_lit = proc_macro2::Literal::string(&quote! {#ty}.to_string());

        #[allow(unused_variables)]
        let keys_lit: Vec<_> = field.keys.iter().map(|k| proc_macro2::Literal::string(&k.to_string())).collect();

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
            paramtests.push(quote! {
                if #var.is_none() {
                    errors.push(_perlxs::error::ToStructErrPart::OmittedKey(&[#(#keys_lit),*]));
                };
            });

            paramvars.push(quote! {
                #ident: #var.unwrap()
            });
        }
    }

    let from_kv_stack = quote! {

            while let Some(sv_res) = ctx.st_try_fetch::<String>(*offset) {
    //            println!("Offset {} = [{:?}]", offset, sv_res);
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

    let dummy_const = syn::Ident::new(&format!("_IMPL_PERLXS_FROMPERLKV_FOR_{}", ident), proc_macro2::Span::call_site());

    let output = quote! {
        #[allow(non_upper_case_globals)]
        const #dummy_const: () = {
            extern crate perl_xs as _perlxs;
            #impl_block
        };
    };

    Ok(output)
}
