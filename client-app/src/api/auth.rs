use crate::api::common::{into_json, Error, Result};
use crate::api::models::auth::{Credentials, UserInfo};
use crate::env;
use gloo_net::http::{Request, RequestBuilder};
use leptos::web_sys::RequestCredentials;
use serde::de::DeserializeOwned;

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str,
}

#[derive(Clone, Copy)]
pub struct AuthorizedApi {
    url: &'static str,
}

impl UnauthorizedApi {
    pub const fn new() -> Self {
        Self { url: env::API_URL }
    }
    pub async fn register(&self, credentials: &Credentials) -> Result<AuthorizedApi> {
        let url = format!("{}/register", self.url);
        let response = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(credentials)?
            .send()
            .await?;

        match response.ok() {
            true => Ok(AuthorizedApi::new()),
            false => Err(Error::UnknownError),
        }
    }
    pub async fn login(&self, credentials: &Credentials) -> Result<AuthorizedApi> {
        let url = format!("{}/login", self.url);
        let response = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(credentials)?
            .send()
            .await?;

        match response.ok() {
            true => Ok(AuthorizedApi::new()),
            false => Err(Error::UnknownError),
        }
    }
    pub async fn try_login_with_session_cookie(&self) -> Result<(AuthorizedApi, UserInfo)> {
        let url = format!("{}/session", self.url);
        let req = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()?;
        let result = self.send(req).await;
        match result {
            Ok(user_info) => Ok((AuthorizedApi::new(), user_info)),
            Err(err) => Err(err),
        }
    }
    async fn send<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}

impl AuthorizedApi {
    pub const fn new() -> Self {
        Self { url: env::API_URL }
    }

    pub async fn logout(&self) -> Result<UnauthorizedApi> {
        let url = format!("{}/logout", self.url);
        Request::post(&url)
            .credentials(RequestCredentials::Include)
            .send()
            .await?;
        Ok(UnauthorizedApi::new())
    }
    pub async fn user_info(&self) -> Result<UserInfo> {
        let url = format!("{}/session", self.url);
        let req = Request::get(&url).credentials(RequestCredentials::Include);
        self.send(req).await
    }
    async fn send<T: DeserializeOwned>(&self, req: RequestBuilder) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
