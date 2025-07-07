use crate::projects::models::project::Project;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryProjectsForClientResponse {
    pub projects: Vec<Project>,
    pub cursor: Option<String>,
}
