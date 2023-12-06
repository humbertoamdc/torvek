use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetSessionResponse {
    pub id: String,
    pub email: String,
}
