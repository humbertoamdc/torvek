use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RegisterClientRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoginClientRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct AdminLoginRequest {
    pub email: String,
    pub password: String,
    pub flow_id: String,
    pub csrf_token: String,
}
