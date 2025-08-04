use crate::models::api_error::ApiError;
use gloo_net::http::{Request, Response};
use http::StatusCode;
use serde::de::DeserializeOwned;
use serde_json::Value;

pub type Result<T> = std::result::Result<T, ApiError>;

pub async fn send<T: DeserializeOwned + std::fmt::Debug>(req: Request) -> Result<T> {
    let response = req.send().await.unwrap();
    into_json(response).await
}

async fn into_json<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    // ensure we've got 2xx status
    if response.ok() {
        if response.status() != StatusCode::NO_CONTENT {
            Ok(response.json().await.unwrap())
        } else {
            let default_t: T = serde_json::from_value(Value::from(()))?;
            Ok(default_t)
        }
    } else {
        Err(response.json::<ApiError>().await.unwrap().into())
    }
}
