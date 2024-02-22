use gloo_net::http::{Request, RequestBuilder};
use leptos::web_sys::RequestCredentials;
use serde::de::DeserializeOwned;

use crate::api::common::{into_json, Error, Result};
use crate::api::models::auth::{Credentials, UserInfo};

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str,
}

#[derive(Clone, Copy)]
pub struct AuthorizedApi {
    url: &'static str,
}

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }
    pub async fn admin_login(&self, credentials: &Credentials) -> Result<AuthorizedApi> {
        let url = format!("{}/accounts/admins/login", self.url);
        let response = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(credentials)?
            .send()
            .await?;

        match response.ok() {
            true => Ok(AuthorizedApi::new(self.url)),
            false => Err(Error::UnknownError),
        }
    }
    pub async fn try_login_with_session_cookie(&self) -> Result<(AuthorizedApi, UserInfo)> {
        let url = format!("{}/accounts/admins/session", self.url);
        let req = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()?;
        let result = self.send(req).await;
        match result {
            Ok(user_info) => Ok((AuthorizedApi::new(self.url), user_info)),
            Err(err) => Err(err),
        }
    }
    async fn send<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}

impl AuthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn logout(&self) -> Result<UnauthorizedApi> {
        let url = format!("{}/account/admins/logout", self.url);
        Request::post(&url)
            .credentials(RequestCredentials::Include)
            .send()
            .await?;
        Ok(UnauthorizedApi::new(self.url))
    }
    pub async fn user_info(&self) -> Result<UserInfo> {
        let url = format!("{}/accounts/admins/session", self.url);
        let req = Request::get(&url).credentials(RequestCredentials::Include);
        self.send(req).await
    }
    async fn send<T: DeserializeOwned>(&self, req: RequestBuilder) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
