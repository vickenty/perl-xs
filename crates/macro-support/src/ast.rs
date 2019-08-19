use crate::error::Errors;
use crate::field::Field;
use syn;

// fn struct_from_ast<'a>(cx: &Ctxt, data: &'a syn::`Field, attrs: Option<&attr::Variant>) -> (Style, Vec<Field<'a>>) {
//     match *data {
//         syn::Field::Struct(ref fields) => (Style::Struct, fields_from_ast(cx, fields, attrs)),
//         syn::Field::Tuple(ref fields) if fields.len() == 1 => {
//             (Style::Newtype, fields_from_ast(cx, fields, attrs))
//         }
//         syn::Field::Tuple(ref fields) => (Style::Tuple, fields_from_ast(cx, fields, attrs)),
//         syn::Field::Unit => (Style::Unit, Vec::new()),
//     }
// }

pub fn fields_from_ast<'a>(errors: &Errors, fields: Vec<syn::Field>) -> Vec<Field> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field::from_ast(errors, i, field))
        .collect()
}

pub fn de_optionalize(ty: &syn::Type) -> (bool, syn::Type) {
    if let &syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = ty
    {
        if segments.len() == 1 && segments[0].ident == "Option" {
            if let syn::PathArguments::AngleBracketed(ref abpd) = segments[0].arguments {
                if abpd.args.len() == 1 {
                    if let syn::GenericArgument::Type(path @ syn::Type::Path(_)) = &abpd.args[0] {
                        return (true, path.clone());
                    }
                }
            }
        }
    }
    (false, ty.clone())
}

//pub fn function_from_ast<'a>(errors: &Errors, item: syn:Item::Fn) {
//    match item {
//        syn::Item::Fn(mut f) => {
//            let no_mangle = f
//                .attrs
//                .iter()
//                .enumerate()
//                .filter_map(|(i, m)| m.interpret_meta().map(|m| (i, m)))
//                .find(|&(_, ref m)| m.name() == "no_mangle");
//            match no_mangle {
//                Some((i, _)) => {
//                    f.attrs.remove(i);
//                }
//                _ => {}
//            }
//            let comments = extract_doc_comments(&f.attrs);
//            f.to_tokens(tokens);
//            let opts = opts.unwrap_or_default();
//            if opts.start().is_some() {
//                if f.decl.generics.params.len() > 0 {
//                    bail_span!(&f.decl.generics, "the start function cannot have generics",);
//                }
//                if f.decl.inputs.len() > 0 {
//                    bail_span!(&f.decl.inputs, "the start function cannot have arguments",);
//                }
//            }
//            let method_kind = ast::MethodKind::Operation(ast::Operation {
//                is_static: true,
//                kind: operation_kind(&opts),
//            });
//            let rust_name = f.ident.clone();
//            let start = opts.start().is_some();
//            program.exports.push(ast::Export {
//                comments,
//                function: f.convert(opts)?,
//                js_class: None,
//                method_kind,
//                method_self: None,
//                rust_class: None,
//                rust_name,
//                start,
//            });
//        }
//    }
//}
