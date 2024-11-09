use crate::projects::models::Project;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryProjectsForClientResponse {
    pub projects: Vec<Project>,
    pub cursor: Option<String>,
}
