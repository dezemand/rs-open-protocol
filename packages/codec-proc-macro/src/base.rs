use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error from syn")]
    Syn(#[from] syn::Error),
    #[error("The 'union' type is not supported")]
    UnionNotSupported,
    #[error("Invalid field configuration")]
    InvalidFieldConfiguration,
    #[error("Fields without identifier are currently not supported")]
    FieldWithoutIdentifier
}

pub type Result<T> = std::result::Result<T, Error>;
