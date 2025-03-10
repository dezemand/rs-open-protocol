use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitInt, Meta};
use crate::base::{Error, Result};

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        syn::Data::Struct(data_struct) => expand_struct(&input, data_struct),
        _ => Err(Error::UnionNotSupported)
    }
}

fn expand_struct(input: &DeriveInput, _data_struct: &syn::DataStruct) -> Result<TokenStream> {
    let struct_name = &input.ident;
    let mut mid: u16 = 0;
    let mut revision: u16 = 0;

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
                    } else {
                        Err(meta.error("Invalid argument"))
                    }
                })?
            }
            _ => {}
        }
    }

    Ok(quote! {
        impl ::open_protocol_codec::message::Message for #struct_name {
            fn mid() -> u16 {
                #mid
            }

            fn revision() -> u16 {
                #revision
            }

            fn to_mid(&self) -> u16 {
                #mid
            }

            fn to_revision(&self) -> u16 {
                #revision
            }
        }
    })
}
