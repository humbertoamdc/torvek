use crate::common::{send, Result};
use gloo_net::http::Request;
use http::header::ACCEPT;
use http::Uri;
use ory_kratos_client::models::{LoginFlow, LogoutFlow, Session};
use web_sys::{window, RequestCredentials, Url};

#[derive(Clone, Copy)]
pub struct AuthClient {
    url: &'static str,
}

impl AuthClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn to_session(&self) -> Result<Session> {
        let url = format!("{}/sessions/whoami", self.url);
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()?;

        send(request).await
    }

    pub async fn create_browser_login_flow(&self) -> Result<LoginFlow> {
        let url = format!("{}/self-service/login/browser", self.url);
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .header(ACCEPT.as_str(), "application/json")
            .build()?;

        send(request).await
    }

    pub async fn redirect_to_login_url(&self, id: String, return_to_url: String) {
        let url = &format!(
            "{}/self-service/login/browser?flow_id={}&return_to={}",
            self.url, id, return_to_url
        );
        window()
            .unwrap()
            .location()
            .replace(&url)
            .expect("fail to redirect to login url");
    }

    pub async fn create_browser_logout_flow(&self) -> Result<LogoutFlow> {
        let url = format!("{}/self-service/logout/browser", self.url);
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()?;

        send(request).await
    }

    pub async fn redirect_to_logout_url(&self, url: String) {
        window()
            .unwrap()
            .location()
            .replace(&url)
            .expect("fail to redirect to logout url");
    }
}
