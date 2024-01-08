use crate::api::common::{into_json, Result};
use crate::env;
use api_boundary::projects::requests::CreateProjectRequest;
use gloo_net::http::Request;
use serde::de::DeserializeOwned;
use web_sys::RequestCredentials;

#[derive(Clone, Copy)]
pub struct ProjectsClient {
    url: &'static str,
}

impl ProjectsClient {
    pub const fn new() -> Self {
        Self { url: env::API_URL }
    }

    pub async fn create_project(&self, body: CreateProjectRequest) -> Result<()> {
        let url = format!("{}/projects", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)?;

        self.send(request).await
    }

    async fn send<T: DeserializeOwned>(&self, req: Request) -> Result<T> {
        let response = req.send().await?;
        into_json(response).await
    }
}
