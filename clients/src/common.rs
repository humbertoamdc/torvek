use api_boundary::ApiError;
use gloo_net::http::{Request, Response};
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(ApiError),
    #[error("unknown error")]
    UnknownError,
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Self::Api(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Self::UnknownError
    }
}

pub async fn send<T: DeserializeOwned>(req: Request) -> Result<T> {
    let response = req.send().await.unwrap();
    into_json(response).await
}

async fn into_json<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    // ensure we've got 2xx status
    if response.ok() {
        if response.status() != StatusCode::NO_CONTENT {
            // println!("{:#?}", response.clone().json());
            Ok(response.json().await.unwrap())
        } else {
            let default_t: T = serde_json::from_value(Value::from(()))?;
            Ok(default_t)
        }
    } else {
        Err(response.json::<ApiError>().await.unwrap().into())
    }
}
