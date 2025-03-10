use proc_macro2::Ident;
use syn::{Meta, MetaList, Variant};
use open_protocol_codec::EnumNumber;
use crate::base::{Error, Result};
use crate::util::parse_number;


#[derive(Debug)]
pub struct EnumItem {
    pub ident: Ident,
    pub value: EnumNumber,
}

fn parse_enum_item_attributes(meta_list: &MetaList) -> Result<EnumNumber> {
    let mut value: Option<EnumNumber> = None;

    meta_list.parse_nested_meta(|meta| {
        if meta.path.is_ident("number") {
            value = Some(parse_number(meta)?);
            Ok(())
        } else {
            Err(meta.error("Invalid property for `open_protocol_value`"))
        }
    })?;

    Ok(value.ok_or(Error::FieldWithoutIdentifier)?)
}

fn parse_enum_value(variant: &Variant) -> Result<EnumNumber> {
    for attr in variant.attrs.clone() {
        match &attr.meta {
            Meta::List(meta_list) if meta_list.path.is_ident("open_protocol_value") => {
                return Ok(parse_enum_item_attributes(meta_list)?);
            }
            _ => {}
        }
    }

    Err(Error::InvalidFieldConfiguration)
}

pub fn parse_enum_items(variants: Vec<&Variant>) -> Result<Vec<EnumItem>> {
    variants
        .iter()
        .map(|variant| {
            Ok(EnumItem {
                ident: variant.ident.clone(),
                value: parse_enum_value(variant)?,
            })
        })
        .collect()
}
