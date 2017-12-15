use error::Errors;
use field::Field;
use syn;

// fn struct_from_ast<'a>(cx: &Ctxt, data: &'a syn::VariantData, attrs: Option<&attr::Variant>) -> (Style, Vec<Field<'a>>) {
//     match *data {
//         syn::VariantData::Struct(ref fields) => (Style::Struct, fields_from_ast(cx, fields, attrs)),
//         syn::VariantData::Tuple(ref fields) if fields.len() == 1 => {
//             (Style::Newtype, fields_from_ast(cx, fields, attrs))
//         }
//         syn::VariantData::Tuple(ref fields) => (Style::Tuple, fields_from_ast(cx, fields, attrs)),
//         syn::VariantData::Unit => (Style::Unit, Vec::new()),
//     }
// }

pub fn fields_from_ast<'a>(errors: &Errors, fields: &'a [syn::Field]) -> Vec<Field> {
    fields
        .iter()
        .enumerate()
        .map(|(i, field)| Field::from_ast(errors, i, field))
        .collect()
}
