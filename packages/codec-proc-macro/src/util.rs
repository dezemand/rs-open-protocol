use std::fmt::Display;
use std::str::FromStr;
use syn::{Field, GenericArgument, LitInt, PathArguments, Type};
use syn::meta::ParseNestedMeta;

pub fn parse_number<T>(meta: ParseNestedMeta) -> Result<T, syn::Error> where T: FromStr, T::Err: Display {
    let value = meta.value()?;
    let parsed: LitInt = value.parse()?;
    Ok(parsed.base10_parse()?)
}

pub fn get_option_type(field: &Field) -> Option<Type> {
    if let Type::Path(type_path) = &field.ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                if let PathArguments::AngleBracketed(angle_bracketed_args) = &segment.arguments {
                    if !angle_bracketed_args.args.len() == 1 {
                        return None;
                    }

                    if let GenericArgument::Type(t) = angle_bracketed_args.args.first().unwrap() {
                        return Some(t.clone());
                    }

                    return None;
                }
            }
        }
    }
    None
}
