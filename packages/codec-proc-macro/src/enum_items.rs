use std::ops::RangeInclusive;
use proc_macro2::Ident;
use syn::{LitStr, Meta, MetaList, Variant};
use open_protocol_codec::EnumNumber;
use crate::base::{Error, Result};
use crate::util::parse_number;
use itertools::Itertools;

#[derive(Debug)]
pub enum EnumValue {
    Number(EnumNumber),
    Range(RangeInclusive<EnumNumber>),
    Other,
}

#[derive(Debug)]
pub struct EnumItem {
    pub ident: Ident,
    pub value: EnumValue,
}

fn parse_enum_item_attributes(meta_list: &MetaList) -> Result<EnumValue> {
    let mut value: Option<EnumValue> = None;

    meta_list.parse_nested_meta(|meta| {
        if meta.path.is_ident("number") && value.is_none() {
            value = Some(EnumValue::Number(parse_number(meta)?));
            Ok(())
        } else if meta.path.is_ident("number_range") && value.is_none() {
            let (left, right) = meta.value()?
                .parse::<LitStr>()?
                .value()
                .split("-")
                .map(|str| str.parse::<u16>().expect("Could not parse range."))
                .collect_tuple::<(u16, u16)>()
                .ok_or(meta.error("Invalid value for range"))?;

            value = Some(EnumValue::Range(RangeInclusive::new(left, right)));
            Ok(())
        } else if meta.path.is_ident("other") && value.is_none() {
            value = Some(EnumValue::Other);
            Ok(())
        } else {
            Err(meta.error("Invalid property for `open_protocol_value`"))
        }
    })?;

    Ok(value.ok_or(Error::FieldWithoutIdentifier)?)
}

fn parse_enum_value(variant: &Variant) -> Result<EnumValue> {
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
