use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateProjectRequest {
    pub customer_id: String,
    pub project_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetProjectByIdRequest {
    pub customer_id: String,
    pub project_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryProjectsForClientRequest {
    pub customer_id: String,
}
impl QueryProjectsForClientRequest {
    pub const fn new(customer_id: String) -> Self {
        Self { customer_id }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteProjectRequest {
    pub customer_id: String,
    pub project_id: String,
}
