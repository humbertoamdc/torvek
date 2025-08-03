use crate::services::emailer::Emailer;
use crate::shared::error::Error;
use async_trait::async_trait;
use aws_sdk_sesv2::types::{Body, Content, Destination, EmailContent, Message};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct EmailerSES {
    client: aws_sdk_sesv2::Client,
    no_reply_email: String,
    admin_emails: Vec<String>,
}

impl EmailerSES {
    pub fn new(
        client: aws_sdk_sesv2::Client,
        no_reply_email: String,
        admin_emails: Vec<String>,
    ) -> Self {
        Self {
            client,
            no_reply_email,
            admin_emails,
        }
    }
}

#[async_trait]
impl Emailer for EmailerSES {
    async fn send_email(
        &self,
        receiver: &str,
        subject: &str,
        message: &str,
    ) -> crate::shared::Result<()> {
        let destination = Destination::builder().to_addresses(receiver).build();
        let subject_content = Content::builder()
            .data(subject)
            .charset("UTF-8")
            .build()
            .map_err(|err| {
                tracing::error!("Failed to build email subject content: {}", err);
                Error::UnknownError
            })?;
        let body_content = Content::builder()
            .data(message)
            .charset("UTF-8")
            .build()
            .map_err(|err| {
                tracing::error!("Failed to build email body content: {}", err);
                Error::UnknownError
            })?;
        let body = Body::builder().text(body_content).build();
        let msg = Message::builder()
            .subject(subject_content)
            .body(body)
            .build();
        let email_content = EmailContent::builder().simple(msg).build();
        let result = self
            .client
            .send_email()
            .from_email_address(&self.no_reply_email)
            .destination(destination)
            .content(email_content)
            .send()
            .await;

        match result {
            Ok(output) => {
                tracing::info!("Email sent successfully: {:?}", output);
                Ok(())
            }
            Err(err) => {
                tracing::error!("Failed to send email: {err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn send_email_to_admins(
        &self,
        subject: &str,
        message: &str,
    ) -> crate::shared::Result<()> {
        for admin_email in &self.admin_emails {
            let _ = self.send_email(admin_email, subject, message).await;
            // The account is currently on sandbox. We are limited to 200 emails
            // per day and 1 email per second. Once we have production access we
            // can remove the sleep below.
            sleep(Duration::from_secs(1)).await;
        }

        // let send_email_futures = self
        //     .admin_emails
        //     .iter()
        //     .map(|admin_email| self.send_email(admin_email, subject, message));
        //
        // try_join_all(send_email_futures).await?;

        Ok(())
    }
}
