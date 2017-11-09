use syn;
use syn::MetaItem::{List, NameValue, Word};
use syn::NestedMetaItem::{Literal, MetaItem};

use error::Errors;

#[derive(Debug)]
pub struct Field {
    pub ident:     String,
    pub keys:      Vec<syn::StrLit>, // could switch to syn::Lit later
    pub ty:    syn::Ty
}

impl Field {
    /// Extract the `#[perlxs(...)]` attributes from a struct field.
    pub fn from_ast(errors: &Errors, index: usize, field: &syn::Field) -> Self {
        let mut keys = Vec::new();

        let ident = match field.ident {
            Some(ref ident) => ident.to_string(),
            None            => index.to_string(),
        };

        for meta_items in field.attrs.iter().filter_map(get_meta_items) {
            for meta_item in meta_items {
                match meta_item {
                    // Parse `#[perlxs(key = "-foo")]`
                    MetaItem(NameValue(ref name, ref lit)) if name == "key" => {
                        if let Ok(s) = get_string_from_lit(errors, name.as_ref(), name.as_ref(), lit) {
                            keys.push( syn::StrLit{ value: s, style: syn::StrStyle::Cooked}  );
                        }
                    },
                    MetaItem(ref meta_item) => {
                        errors.error(format!("unknown perlxs container attribute `{}`",
                                         meta_item.name()));
                    }

                    Literal(_) => {
                        errors.error("unexpected literal in perlxs container attribute");
                    }
                }
            }
        }

        if keys.len() == 0 {
            match field.ident {
                Some(ref ident) => keys.push(
                    syn::StrLit{ value: ident.to_string(), style: syn::StrStyle::Cooked }
                ),
                None            => errors.error("at least one key is required"),
            };
        }

        Field {
            ident:     ident,
            keys:      keys,
            ty:        field.ty.clone(),
        }
    }

}

pub fn get_meta_items(attr: &syn::Attribute) -> Option<Vec<syn::NestedMetaItem>> {
    match attr.value {
        List(ref name, ref items) if name == "perlxs" => Some(items.iter().cloned().collect()),
        _ => None,
    }
}

fn get_string_from_lit(
    errors: &Errors,
    attr_name: &str,
    meta_item_name: &str,
    lit: &syn::Lit,
) -> Result<String, ()> {
    if let syn::Lit::Str(ref s, _) = *lit {
        Ok(s.clone())
    } else {
        errors.error(
            format!(
                "expected perlxs {} attribute to be a string: `{} = \"...\"`",
                attr_name,
                meta_item_name
            ),
        );
        Err(())
    }
}


struct Property<'c, T> {
    errors: &'c Errors,
    name: &'static str,
    value: Option<T>,
}

impl<'c, T> Property<'c, T> {
    fn none(errors: &'c Errors, name: &'static str) -> Self {
        Property {
            errors: errors,
            name: name,
            value: None,
        }
    }

    fn set(&mut self, value: T) {
        if self.value.is_some() {
            self.errors
                .error(format!("duplicate perlxs attribute `{}`", self.name));
        } else {
            self.value = Some(value);
        }
    }

    fn set_opt(&mut self, value: Option<T>) {
        if let Some(value) = value {
            self.set(value);
        }
    }

    fn set_if_none(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value);
        }
    }

    fn get(self) -> Option<T> {
        self.value
    }
}