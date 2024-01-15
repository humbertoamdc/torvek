use crate::api::models;
use gloo_net::http::Response;
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(models::common::Error),
    #[error("unknown error")]
    UnknownError,
}

impl From<models::common::Error> for Error {
    fn from(e: models::common::Error) -> Self {
        Self::Api(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::UnknownError
    }
}

pub async fn into_json<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    // ensure we've got 2xx status
    if response.ok() {
        if response.status() != StatusCode::NO_CONTENT {
            Ok(response.json().await?)
        } else {
            let default_t: T = serde_json::from_value(Value::from(()))?;
            Ok(default_t)
        }
    } else {
        Err(response.json::<models::common::Error>().await?.into())
    }
}
