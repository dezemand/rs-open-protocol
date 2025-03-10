use crate::base::{Error, Result};
use crate::field::{get_fields_size, parse_fields, Amount, MessageField, MessageFieldType};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DataEnum, DataStruct, DeriveInput};
use crate::enum_items::{parse_enum_items};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(data_struct) => expand_struct(&input, data_struct),
        syn::Data::Enum(data_enum) => expand_enum(&input, data_enum),
        _ => Err(Error::UnionNotSupported)
    }
}

fn expand_struct(input: &DeriveInput, data_struct: &DataStruct) -> Result<TokenStream> {
    let struct_name = &input.ident;
    let struct_name_str = struct_name.to_string();
    let fields = parse_fields(&data_struct.fields)?;

    let mut decode_field_lines = Vec::new();
    let mut create_struct_lines = Vec::new();

    for field in &fields {
        decode_field_lines.push(get_decode_field_line(&field)?);
        create_struct_lines.push(get_create_struct_line(&field)?);
    }

    let size_check = match get_fields_size(&fields) {
        Some(size) => quote! {
            if size != #size {
                return Err(::open_protocol_codec::decode::Error::SizeMismatch { requested_size: #size, type_name: #struct_name_str.into() });
            }
        },
        None => quote! {}
    };

    Ok(quote! {
        impl ::open_protocol_codec::decode::Decode for #struct_name {
            fn decode(decoder: &mut ::open_protocol_codec::decode::Decoder) -> ::open_protocol_codec::decode::Result<Self> {
                #(#decode_field_lines)*

                Ok(Self {
                    #(#create_struct_lines)*
                })
            }

            fn decode_sized(decoder: &mut ::open_protocol_codec::decode::Decoder, size: usize) -> ::open_protocol_codec::decode::Result<Self> {
                #size_check
                Self::decode(decoder)
            }
        }
    })
}

fn expand_enum(input: &DeriveInput, data_enum: &DataEnum) -> Result<TokenStream> {
    let enum_name = &input.ident;
    let items = parse_enum_items(data_enum.variants.iter().collect())?;

    let mut decode_enum_lines = Vec::with_capacity(items.len());
    for item in items {
        let name = item.ident;
        let value = item.value;
        decode_enum_lines.push(quote! { #value => Ok(Self::#name), });
    }

    Ok(quote! {
        impl ::open_protocol_codec::decode::Decode for #enum_name {
            fn decode(_: &mut ::open_protocol_codec::decode::Decoder) -> ::open_protocol_codec::decode::Result<Self> {
                Err(::open_protocol_codec::decode::Error::UnsizedDecodeNotAllowed)
            }

            fn decode_sized(decoder: &mut ::open_protocol_codec::decode::Decoder, size: usize) -> ::open_protocol_codec::decode::Result<Self> {
                let number = ::open_protocol_codec::EnumNumber::decode_sized(decoder, size)?;

                match number {
                    #(#decode_enum_lines)*
                    n => Err(::open_protocol_codec::decode::Error::InvalidEnumNumber(n, decoder.pos() - size)),
                }
            }
        }
    })
}

fn get_decode_field_line(field: &MessageField) -> Result<TokenStream> {
    let identifier = field.get_field_identifier()?;
    let data_type = &field.data_type;

    Ok(match &field.field_type {
        MessageFieldType::FixedLength { length } => {
            let length = get_amount_identifier(length);
            quote! {
                let #identifier: #data_type = decoder.read_sized_field(#length)?;
            }
        },

        MessageFieldType::NumberedVariableLength { number, option: false } => {
            quote! {
                let #identifier: #data_type = decoder.read_numbered_field(#number)?;
            }
        },

        MessageFieldType::NumberedVariableLength { number, option: true } => {
            quote! {
                let #identifier: #data_type = decoder.read_numbered_optional_field(#number)?;
            }
        },

        MessageFieldType::NumberedFixedLength { number, length, option: false } => {
            let length = get_amount_identifier(length);
            quote! {
                let #identifier: #data_type = decoder.read_numbered_sized_field(#number, #length)?;
            }
        },

        MessageFieldType::NumberedFixedLength { number, length, option: true } => {
            let length = get_amount_identifier(length);
            quote! {
                let #identifier: #data_type = decoder.read_numbered_sized_optional_field(#number, #length)?;
            }
        },

        MessageFieldType::List { length, amount } => {
            let amount = get_amount_identifier(amount);
            let length = get_amount_identifier(length);
            quote! {
                let #identifier: #data_type = decoder.read_sized_list(#amount, #length)?;
            }
        }

        MessageFieldType::ListVariableLength { amount } => {
            let amount = get_amount_identifier(amount);
            quote! {
                let #identifier: #data_type = decoder.read_list(#amount)?;
            }
        }
    })
}

fn get_create_struct_line(field: &MessageField) -> Result<TokenStream> {
    let var_identifier = field.get_field_identifier()?;
    let field_identifier = &field.ident.clone().unwrap();

    Ok(quote! {
        #field_identifier: #var_identifier,
    })
}

fn get_amount_identifier(amount: &Amount) -> TokenStream {
    match amount {
        Amount::Fixed(size) => quote! { #size },
        Amount::FromField(field) => {
            let amount_field_identifier = Ident::new(format!("field_{}", field).as_str(), Span::call_site());
            quote! { #amount_field_identifier as usize }
        }
    }
}
