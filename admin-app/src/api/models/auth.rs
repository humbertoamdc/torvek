use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
    pub flow_id: String,
    pub csrf_token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
}
