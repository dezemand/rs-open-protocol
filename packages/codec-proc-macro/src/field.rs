use crate::base::{Error, Result};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote};
use syn::meta::ParseNestedMeta;
use syn::{Field, Fields, LitInt, LitStr, Meta, MetaList, Type};
use open_protocol_codec::FIELD_NUMBER_LEN;
use crate::util::{get_option_type, parse_number};

#[derive(Debug)]
pub enum Amount {
    Fixed(usize),
    FromField(String),
}

#[derive(Debug, Default)]
struct ParseField {
    pub number: Option<u8>,
    pub length: Option<Amount>,
    pub list: bool,
    pub list_amount: Option<Amount>,
}

#[derive(Debug)]
pub struct MessageField {
    pub index: usize,
    pub ident: Option<Ident>,
    pub field_type: MessageFieldType,
    pub data_type: Type,
}

impl MessageField {
    pub fn get_self_ref(&self) -> Result<TokenStream> {
        Ok(match &self.ident {
            Some(field_name) => {
                quote! { &self.#field_name }
            }
            None => return Err(Error::FieldWithoutIdentifier)
        })
    }

    pub fn get_field_identifier(&self) -> Result<Ident> {
        Ok(match &self.ident {
            Some(field_name) => Ident::new(format!("field_{}", field_name.to_string()).as_str(), Span::call_site()),
            None => Ident::new(format!("field_{}", self.index).as_str(), Span::call_site())
        })
    }
}

#[derive(Debug)]
pub enum MessageFieldType {
    FixedLength { length: Amount },
    NumberedVariableLength { number: u8, option: bool },
    NumberedFixedLength { number: u8, length: Amount, option: bool },
    List { length: Amount, amount: Amount },
    ListVariableLength { amount: Amount },
}

pub fn parse_fields(fields: &Fields) -> Result<Vec<MessageField>> {
    match fields {
        Fields::Named(ref named) => named.named
            .iter()
            .enumerate()
            .map(|(index, field)| parse_field(index, field))
            .collect::<Result<Vec<_>>>(),
        Fields::Unit => Ok(Vec::new()),
        Fields::Unnamed(_) => Err(Error::FieldWithoutIdentifier)
    }
}

pub fn get_fields_size(fields: &Vec<MessageField>) -> Option<usize> {
    let mut total = 0;

    for field in fields {
        match field.field_type {
            MessageFieldType::FixedLength { length: Amount::Fixed(size) }
            => { total += size; }
            MessageFieldType::NumberedFixedLength { length: Amount::Fixed(size), option: false, .. }
            => { total += size + FIELD_NUMBER_LEN; }
            MessageFieldType::List { length: Amount::Fixed(size), amount: Amount::Fixed(amount) }
            => { total += size * amount; }

            _ => { return None }
        }
    }

    Some(total)
}

fn parse_field(index: usize, field: &Field) -> Result<MessageField> {
    let mut field_data = ParseField::default();
    let option_type = get_option_type(field);

    for attr in &field.attrs {
        match &attr.meta {
            Meta::List(meta_list) if meta_list.path.is_ident("open_protocol_field") => {
                parse_field_attributes(meta_list, &mut field_data)?;
            }
            _ => {}
        }
    }

    let field_type = match field_data {
        ParseField { list: true, list_amount: Some(amount), length: Some(length), .. }
        => MessageFieldType::List { length, amount },

        ParseField { list: true, list_amount: Some(amount), .. }
        => MessageFieldType::ListVariableLength { amount },

        ParseField { list: false, length: Some(length), number: Some(number), .. }
        => MessageFieldType::NumberedFixedLength { number, length, option: option_type.is_some() },

        ParseField { list: false, length: None, number: Some(number), .. }
        => MessageFieldType::NumberedVariableLength { number, option: option_type.is_some() },

        ParseField { list: false, length: Some(length), number: None, .. }
        => MessageFieldType::FixedLength { length },

        ParseField { list: true, list_amount: None, .. }
        => return Err(Error::InvalidFieldConfiguration),

        _ => return Err(Error::InvalidFieldConfiguration),
    };

    Ok(MessageField {
        index,
        ident: field.ident.clone(),
        field_type,
        data_type: field.ty.clone(),
    })
}

fn parse_field_attributes(meta_list: &MetaList, field_data: &mut ParseField) -> Result<()> {
    meta_list.parse_nested_meta(|meta| {
        if meta.path.is_ident("number") {
            field_data.number = Some(parse_number(meta)?);
            Ok(())
        } else if meta.path.is_ident("length") {
            field_data.length = Some(parse_amount(&meta)?);
            Ok(())
        } else if meta.path.is_ident("list") {
            field_data.list = true;
            Ok(())
        } else if meta.path.is_ident("amount") {
            field_data.list_amount = Some(parse_amount(&meta)?);
            Ok(())
        } else {
            Err(meta.error("Invalid property for field"))
        }
    })?;

    Ok(())
}

fn parse_amount(meta: &ParseNestedMeta) -> std::result::Result<Amount, syn::Error> {
    let value = meta.value()?;

    if value.peek(LitInt) {
        Ok(Amount::Fixed(value.parse::<LitInt>()?.base10_parse()?))
    } else if value.peek(LitStr) {
        Ok(Amount::FromField(value.parse::<LitStr>()?.value()))
    } else {
        Err(meta.error("Invalid value"))
    }
}
