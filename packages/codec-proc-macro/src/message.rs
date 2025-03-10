use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, DeriveInput, LitInt, LitStr, Meta};
use open_protocol_codec::message::MessageType;
use crate::base::{Error, Result};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(data_struct) => expand_struct(&input, data_struct),
        _ => Err(Error::UnionNotSupported)
    }
}

fn get_message_type_tokens(message_type: &MessageType) -> TokenStream {
    match message_type {
        MessageType::Data => quote! { ::open_protocol_codec::message::MessageType::Data },
        MessageType::RequestExtraData => quote! { ::open_protocol_codec::message::MessageType::RequestExtraData },
        MessageType::SubscribeExtraData => quote! { ::open_protocol_codec::message::MessageType::SubscribeExtraData },
        MessageType::UnsubscribeExtraData => quote! { ::open_protocol_codec::message::MessageType::UnsubscribeExtraData },
    }
}

fn get_details(input: &DeriveInput) -> Result<(u16, u16, MessageType)> {
    let mut mid: u16 = 0;
    let mut revision: u16 = 0;
    let mut message_type = MessageType::Data;

    for attr in &input.attrs {
        match &attr.meta {
            Meta::List(meta_list) if meta_list.path.is_ident("open_protocol_message") => {
                meta_list.parse_nested_meta(|meta| {
                    if meta.path.is_ident("MID") {
                        let value = meta.value()?;
                        let parsed: LitInt = value.parse()?;
                        mid = parsed.base10_parse::<u16>()?;
                        Ok(())
                    } else if meta.path.is_ident("revision") {
                        let value = meta.value()?;
                        let parsed: LitInt = value.parse()?;
                        revision = parsed.base10_parse::<u16>()?;
                        Ok(())
                    } else if meta.path.is_ident("type") {
                        let value = meta.value()?;
                        let parsed: LitStr = value.parse()?;
                        let content = parsed.value();

                        match content.as_str() {
                            "data" => message_type = MessageType::Data,
                            "request_extra_data" => message_type = MessageType::RequestExtraData,
                            "subscribe_extra_data" => message_type = MessageType::SubscribeExtraData,
                            "unsubscribe_extra_data" => message_type = MessageType::UnsubscribeExtraData,
                            _ => return Err(meta.error(format!("Invalid value '{content}' for type field.")))
                        }
                        Ok(())
                    } else {
                        Err(meta.error("Invalid argument"))
                    }
                })?
            }
            _ => {}
        }
    }

    Ok((mid, revision, message_type))
}

fn expand_struct(input: &DeriveInput, _data_struct: &DataStruct) -> Result<TokenStream> {
    let struct_name = &input.ident;
    let (mid, revision, message_type) = get_details(input)?;

    let revision_fns = if revision != 1 {
        quote! {
            fn revision() -> u16 { #revision }
            fn to_revision(&self) -> u16 { #revision }
        }
    } else {
        TokenStream::new()
    };

    let message_type_fns = if message_type != MessageType::Data {
        let message_type_name = get_message_type_tokens(&message_type);
        quote! {
            fn message_type() -> ::open_protocol_codec::message::MessageType {
                #message_type_name
            }
            fn to_message_type(&self) -> ::open_protocol_codec::message::MessageType {
                #message_type_name
            }
        }
    } else {
        TokenStream::new()
    };

    Ok(quote! {
        impl ::open_protocol_codec::message::Message for #struct_name {
            fn mid() -> u16 { #mid }
            fn to_mid(&self) -> u16 { #mid }
            #revision_fns
            #message_type_fns
        }
    })
}
