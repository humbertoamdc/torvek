use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("email is already in use")]
    _EmailTakenRegistrationError,
    #[error("invalid credentials")]
    _InvalidCredentialsLoginError,
    #[error("password has been found in data breaches")]
    _BreachedPasswordRegistrationError,
    #[error("the request item doesn't exist")]
    _ItemNotFoundError,
    #[error("no quote selected for part with id `{0}`")]
    _NoSelectedQuoteAvailableForPart(String),
    #[error("cannot update parts after paying the quotation")]
    _UpdatePartAfterPayingQuotation,
    #[error("invalid url couldn't be parsed")]
    _InvalidUrl,
    #[error("cannot delete a project contains payed quotes")]
    _DeleteLockedProject,
    #[error("cannot delete a quote that has been paid for")]
    _DeletePayedQuotation,
    #[error("a pdf quote can't be generated because parts haven't been quoted yet")]
    _NoPdfQuoteAvailable,
    #[error("an unexpected error occurred")]
    _UnknownError,
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    #[default]
    UnknownError,
    ItemNotFound,
    MissingUserInput,
    NotAllowed,
    BadInput,
}
