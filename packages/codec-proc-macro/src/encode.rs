use crate::base::{Error, Result};
use crate::field::{parse_fields, get_fields_size, Amount, MessageField, MessageFieldType};
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
    let fields = parse_fields(&data_struct.fields)?;

    let mut write_field_lines = Vec::new();
    for field in &fields {
        write_field_lines.push(encode_field_line(&field)?)
    }

    let size_check = match get_fields_size(&fields) {
        Some(size) => quote! {
            if size != #size {
                return Err(::open_protocol_codec::encode::Error::InvalidSize(size));
            }
        },
        None => quote! {}
    };

    Ok(quote! {
        impl ::open_protocol_codec::encode::Encode for #struct_name {
            fn encode(&self, encoder: &mut ::open_protocol_codec::encode::Encoder) -> ::open_protocol_codec::encode::Result<()> {
                #(#write_field_lines)*
                Ok(())
            }

            fn encode_sized(&self, encoder: &mut ::open_protocol_codec::encode::Encoder, size: usize) -> ::open_protocol_codec::encode::Result<()> {
                #size_check
                self.encode(encoder)
            }
        }
    })
}

fn encode_field_line(field: &MessageField) -> Result<TokenStream> {
    let self_ref = field.get_self_ref()?;

    let token_stream = match &field.field_type {
        MessageFieldType::FixedLength { length } => {
            let length = get_amount_identifier(length);
            quote! {
                encoder.write_sized_field(#self_ref, #length)?;
            }
        }

        MessageFieldType::NumberedVariableLength { number, option: false } => {
            quote! {
                encoder.write_numbered_field(#self_ref, #number)?;
            }
        }

        MessageFieldType::NumberedVariableLength { number, option: true } => {
            quote! {
                encoder.write_numbered_optional_field(#self_ref, #number)?;
            }
        }

        MessageFieldType::NumberedFixedLength { length, number, option: false } => {
            let length = get_amount_identifier(length);
            quote! {
                encoder.write_numbered_sized_field(#self_ref, #number, #length)?;
            }
        }

        MessageFieldType::NumberedFixedLength { length, number, option: true } => {
            let length = get_amount_identifier(length);
            quote! {
                encoder.write_numbered_sized_optional_field(#self_ref, #number, #length)?;
            }
        }

        MessageFieldType::List { length, amount } => {
            let length = get_amount_identifier(length);
            let amount = get_amount_identifier(amount);
            quote! {
                encoder.write_sized_list(#self_ref, #length, #amount)?;
            }
        }

        MessageFieldType::ListVariableLength { amount } => {
            let amount = get_amount_identifier(amount);
            quote! {
                encoder.write_list(#self_ref, #amount)?;
            }
        }
    };

    Ok(token_stream)
}

fn expand_enum(input: &DeriveInput, data_enum: &DataEnum) -> Result<TokenStream> {
    let enum_name = &input.ident;
    let fields = parse_enum_items(data_enum.variants.iter().collect())?;

    let mut write_enum_lines = Vec::with_capacity(fields.len());
    for field in fields {
        let item = field.ident;
        let value = field.value;

        write_enum_lines.push(quote! { Self::#item => #value, });
    }

    Ok(quote! {
        impl ::open_protocol_codec::encode::Encode for #enum_name {
            fn encode(&self, _: &mut ::open_protocol_codec::encode::Encoder) -> ::open_protocol_codec::encode::Result<()> {
                Err(::open_protocol_codec::encode::Error::UnsizedEncodeNotAllowed)
            }

            fn encode_sized(&self, encoder: &mut ::open_protocol_codec::encode::Encoder, size: usize) -> ::open_protocol_codec::encode::Result<()> {
                let number = match self {
                    #(#write_enum_lines)*
                };
                number.encode_sized(encoder, size)
            }
        }
    })
}

fn get_amount_identifier(amount: &Amount) -> TokenStream {
    match amount {
        Amount::Fixed(size) => quote! { #size },
        Amount::FromField(field) => {
            let field_ident = Ident::new(field, Span::call_site());
            quote! { self.#field_ident as usize }
        }
    }
}
