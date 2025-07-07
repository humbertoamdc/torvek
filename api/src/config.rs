use serde_derive::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub app: ConfigApp,
    pub auth: ConfigAuth,
    pub orders: ConfigOrders,
    pub projects: ConfigProjects,
    pub quotations: ConfigQuotations,
    pub parts: ConfigParts,
    pub payments: ConfigPayments,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigApp {
    pub env: Environment,
    pub port: u16,
    pub domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigAuth {
    pub ory_clients_url: String,
    pub ory_clients_api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigOrders {
    pub s3_bucket: String,
    pub orders_table: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigProjects {
    pub projects_table: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigQuotations {
    pub quotations_table: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigParts {
    pub s3_bucket: String,
    pub parts_table: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigPayments {
    pub secret_key: String,
    pub webhook_secret: String,
    pub success_url: String,
}

impl From<&str> for Config {
    fn from(config_string: &str) -> Self {
        toml::from_str::<Config>(config_string).expect("failed to parse config")
    }
}

#[derive(Serialize_enum_str, Deserialize_enum_str, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    Development,
    Staging,
    Production,
}

impl Environment {
    pub fn secure_session_cookie(&self) -> bool {
        *self != Environment::Development
    }
}
