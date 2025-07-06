use crate::auth::models::session::Identity;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateProjectInput {
    pub identity: Identity,
    pub project_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetProjectByIdInput {
    pub identity: Identity,
    pub project_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryProjectsForClientInput {
    pub identity: Identity,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteProjectInput {
    pub identity: Identity,
    pub project_id: String,
}
