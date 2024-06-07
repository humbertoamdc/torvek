use chrono::{DateTime, Utc};
use names::Generator;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub id: String,
    pub client_id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
impl Project {
    pub fn new(client_id: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let name = Generator::default().next().unwrap();
        let now = Utc::now();

        Self {
            id,
            client_id,
            name,
            created_at: now,
            updated_at: now,
        }
    }
}
