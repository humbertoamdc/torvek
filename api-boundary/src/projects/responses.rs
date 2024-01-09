use crate::projects::models::Project;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryProjectsForClientResponse {
    pub projects: Vec<Project>,
}
impl QueryProjectsForClientResponse {
    pub const fn new(projects: Vec<Project>) -> Self {
        Self { projects }
    }
}
