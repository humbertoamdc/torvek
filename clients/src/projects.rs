use crate::common::{send, Result};
use api_boundary::projects::models::Project;
use api_boundary::projects::requests::CreateProjectRequest;
use api_boundary::projects::responses::QueryProjectsForClientResponse;
use gloo_net::http::Request;
use web_sys::RequestCredentials;

#[derive(Clone, Copy)]
pub struct ProjectsClient {
    url: &'static str,
}

impl ProjectsClient {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn create_project(&self, body: CreateProjectRequest) -> Result<()> {
        let url = format!("{}/projects", self.url);
        let request = Request::post(&url)
            .credentials(RequestCredentials::Include)
            .json(&body)
            .unwrap();

        send(request).await
    }

    pub async fn query_projects_for_client(
        &self,
        customer_id: String,
    ) -> Result<QueryProjectsForClientResponse> {
        let url = format!("{}/customers/{customer_id}/projects", self.url);
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()
            .unwrap();

        send(request).await
    }

    pub async fn get_project_by_id(
        &self,
        customer_id: String,
        project_id: String,
    ) -> Result<Project> {
        let url = format!("{}/customers/{customer_id}/projects/{project_id}", self.url);
        let request = Request::get(&url)
            .credentials(RequestCredentials::Include)
            .build()
            .unwrap();

        send(request).await
    }
}
