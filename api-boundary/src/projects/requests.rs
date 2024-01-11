use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateProjectRequest {
    pub client_id: String,
}
impl CreateProjectRequest {
    pub const fn new(client_id: String) -> Self {
        Self { client_id }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryProjectsForClientRequest {
    pub client_id: String,
}
impl QueryProjectsForClientRequest {
    pub const fn new(client_id: String) -> Self {
        Self { client_id }
    }
}
