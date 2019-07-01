use proc_macro2::{Ident, Span};

use crate::error::Errors;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ident: syn::Ident,
    pub keys: Vec<String>,
    pub ty: syn::Type,
    pub optional: bool,
}

impl Field {
    /// Extract the `#[perlxs(...)]` attributes from a struct field.
    pub fn from_ast(errors: &Errors, index: usize, field: &syn::Field) -> Self {
        let mut keys = Vec::new();

        let name = match field.ident {
            Some(ref ident) => ident.to_string(),
            None => index.to_string(),
        };

        for meta_items in field.attrs.iter().filter_map(get_meta_items) {
            for meta_item in meta_items {
                match meta_item {
                    // Parse `#[perlxs(key = "-foo")]`
                    syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue{ref ident, ref lit, ref eq_token})) if ident == "key" => {
                        if let Ok(s) = get_string_from_lit(errors, &ident.to_string(), &ident.to_string(), lit) {
                            keys.push(s);
                        }
                    }
                    syn::NestedMeta::Meta(ref meta_item) => {
                        errors.error(format!(
                            "unknown perlxs container attribute `{:?}`",
                            meta_item //.name()
                        ));
                    }

                    syn::NestedMeta::Literal(_) => {
                        errors.error("unexpected literal in perlxs container attribute");
                    }
                }
            }
        }

        if keys.len() == 0 {
            match field.ident {
                Some(ref ident) => keys.push(ident.to_string()),
                None => errors.error("at least one key is required"),
            };
        }

        //Path(None, Path { global: false, segments: [PathSegment { ident: Ident("Option"), parameters: AngleBracketed(AngleBracketedParameterData { lifetimes: [], types: [Path(None, Path { global: false, segments: [PathSegment { ident: Ident("String"), parameters: AngleBracketed(AngleBracketedParameterData { lifetimes: [], types: [], bindings: [] }) }] })], bindings: [] }) }] })
        let (optional, inner_ty) = crate::ast::de_optionalize(&field.ty);

        Field {
            ident: field.ident.clone().unwrap(),
            name: name,
            keys: keys,
            ty: inner_ty,
            optional: optional,
        }
    }
}

pub fn get_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMeta>> {
    match attr.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList{ref ident, ref nested, .. })) if ident == "perlxs" => Some(nested.iter().cloned().collect()),
        _ => None,
    }
}

fn get_string_from_lit(errors: &Errors, attr_name: &str, meta_item_name: &str, lit: &syn::Lit) -> Result<String, ()> {
    if let syn::Lit::Str(litstr @ syn::LitStr{..}) = lit {
        Ok(litstr.value())
    } else {
        errors.error(format!(
            "expected perlxs {} attribute to be a string: `{} = \"...\"`",
            attr_name, meta_item_name
        ));
        Err(())
    }
}
