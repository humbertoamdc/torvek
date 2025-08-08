use crate::services::emailer::Emailer;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
pub struct ContactAdminsInput {
    pub name: String,
    pub email: String,
    pub company: Option<String>,
    pub phone: Option<String>,
    pub message: String,
}

pub struct ContactAdmins {
    emailer_service: Arc<dyn Emailer>,
}

impl ContactAdmins {
    pub fn new(emailer_service: Arc<dyn Emailer>) -> Self {
        Self { emailer_service }
    }
}

#[async_trait]
impl UseCase<ContactAdminsInput, ()> for ContactAdmins {
    async fn execute(&self, input: ContactAdminsInput) -> Result<()> {
        let message = format!(
            "{}\n\n{}\n{}\n{}",
            input.message,
            input.email,
            input.company.unwrap_or(String::from("None")),
            input.phone.unwrap_or(String::from("None")),
        );
        self.emailer_service
            .send_email_to_admins(&format!("Message from {}", input.name), &message)
            .await
    }
}
