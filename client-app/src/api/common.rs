use crate::api::models;
use gloo_net::http::Response;
use serde::de::DeserializeOwned;

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

pub async fn into_json<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    // ensure we've got 2xx status
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(response.json::<models::common::Error>().await?.into())
    }
}
