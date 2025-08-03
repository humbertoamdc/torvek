use crate::shared::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Emailer: Send + Sync + 'static {
    async fn send_email(&self, receiver: &str, subject: &str, message: &str) -> Result<()>;
    async fn send_email_to_admins(&self, subject: &str, message: &str) -> Result<()>;
}
