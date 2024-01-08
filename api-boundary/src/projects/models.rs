use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    id: String,
    client_id: String,
}
impl Project {
    pub fn new(client_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            client_id,
        }
    }
}
