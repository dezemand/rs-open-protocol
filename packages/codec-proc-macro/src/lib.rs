use syn::{parse_macro_input, DeriveInput};
use base::{Error};

mod decode;
mod encode;
mod base;
mod field;
mod message;
mod enum_items;
mod util;

extern crate proc_macro;

#[proc_macro_derive(OpenProtocolEncode, attributes(open_protocol_field, open_protocol_value))]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let token_stream = try_expand(encode::expand(input));
    proc_macro::TokenStream::from(token_stream)
}

#[proc_macro_derive(OpenProtocolDecode, attributes(open_protocol_field, open_protocol_value))]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let token_stream = try_expand(decode::expand(input));
    proc_macro::TokenStream::from(token_stream)
}

#[proc_macro_derive(OpenProtocolMessage, attributes(open_protocol_message))]
pub fn derive_message(msg: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(msg as DeriveInput);
    let token_stream = try_expand(message::expand(input));
    proc_macro::TokenStream::from(token_stream)
}


fn try_expand(result: Result<proc_macro2::TokenStream, Error>) -> proc_macro2::TokenStream {
    match result {
        Ok(token_stream) => token_stream,
        Err(Error::Syn(syn_error)) => syn_error.to_compile_error(),
        Err(err) => panic!("Error encountered: {}", err)
    }
}
