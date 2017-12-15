use syn;
use syn::{PathParameters, Ty};
use syn::MetaItem::{List, NameValue};
use syn::NestedMetaItem::{Literal, MetaItem};

use error::Errors;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub ident: syn::Ident,
    pub keys: Vec<String>,
    pub ty: syn::Ty,
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
                    MetaItem(NameValue(ref name, ref lit)) if name == "key" => {
                        if let Ok(s) = get_string_from_lit(errors, name.as_ref(), name.as_ref(), lit) {
                            keys.push(s);
                        }
                    }
                    MetaItem(ref meta_item) => {
                        errors.error(format!(
                            "unknown perlxs container attribute `{}`",
                            meta_item.name()
                        ));
                    }

                    Literal(_) => {
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
        let (optional, inner_ty) = de_optionalize(&field.ty);

        Field {
            ident: field.ident.clone().unwrap(),
            name: name,
            keys: keys,
            ty: inner_ty,
            optional: optional,
        }
    }
}

pub fn de_optionalize(ty: &syn::Ty) -> (bool, syn::Ty) {
    if let &Ty::Path(_, syn::Path { ref segments, .. }) = ty {
        if segments.len() == 1 && segments[0].ident == "Option" {
            if let PathParameters::AngleBracketed(ref abpd) = segments[0].parameters {
                if abpd.types.len() == 1 {
                    if let syn::Ty::Path(_, _) = abpd.types[0] {
                        return (true, abpd.types[0].clone());
                    }
                }
            }
        }
    }
    (false, ty.clone())
}

pub fn get_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMetaItem>> {
    match attr.value {
        List(ref name, ref items) if name == "perlxs" => Some(items.iter().cloned().collect()),
        _ => None,
    }
}

fn get_string_from_lit(errors: &Errors, attr_name: &str, meta_item_name: &str, lit: &syn::Lit) -> Result<String, ()> {
    if let syn::Lit::Str(ref s, _) = *lit {
        Ok(s.clone())
    } else {
        errors.error(format!(
            "expected perlxs {} attribute to be a string: `{} = \"...\"`",
            attr_name, meta_item_name
        ));
        Err(())
    }
}
