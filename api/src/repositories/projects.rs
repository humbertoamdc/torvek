use crate::projects::models::project::{CustomerId, Project, ProjectId};
use crate::shared::error::Error;
use crate::shared::error::Error::UnknownError;
use crate::shared::{QueryResponse, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

pub const ATTRIBUTES_SEPARATOR: &str = "&";

#[async_trait]
pub trait ProjectsRepository: Send + Sync + 'static {
    async fn create(&self, project: Project) -> Result<()>;
    /// Delete project ONLY if it is not in `LOCKED` status.
    async fn delete(&self, customer_id: String, project_id: String) -> Result<()>;
    async fn get(&self, customer_id: String, project_id: String) -> Result<Project>;
    async fn query(
        &self,
        customer_id: String,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        name: Option<String>,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Project>, String>>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamodbProject {
    pub pk: CustomerId,
    pub sk: ProjectId,
    /// created_at&project_id
    pub lsi1_sk: String,
    /// name&project_id
    pub lsi2_sk: String,
    pub is_locked: bool,
    pub updated_at: DateTime<Utc>,
}

impl TryInto<Project> for DynamodbProject {
    type Error = Error;

    fn try_into(self) -> std::result::Result<Project, Self::Error> {
        let mut created_at = None::<DateTime<Utc>>;
        let mut name = None::<String>;

        let lsi1_sk_attributes = self.lsi1_sk.split_once(ATTRIBUTES_SEPARATOR);
        let lsi2_sk_attributes = self.lsi2_sk.split_once(ATTRIBUTES_SEPARATOR);

        if let Some((sk_created_at, _)) = lsi1_sk_attributes {
            created_at = Some(DateTime::<Utc>::from_str(sk_created_at).unwrap());
        }
        if let Some((sk_name, _)) = lsi2_sk_attributes {
            name = Some(sk_name.to_string());
        }

        let item = Project {
            id: self.sk.clone(),
            customer_id: self.pk,
            name: name.ok_or_else(|| {
                tracing::error!(
                    "name is required but not found for project with id {}",
                    self.sk
                );
                UnknownError
            })?,
            is_locked: self.is_locked,
            created_at: created_at.ok_or_else(|| {
                tracing::error!(
                    "create_at is required but not found for project with id {}",
                    self.sk
                );
                UnknownError
            })?,
            updated_at: self.updated_at,
        };

        Ok(item)
    }
}

impl From<Project> for DynamodbProject {
    fn from(value: Project) -> Self {
        let lsi1_sk = format!(
            "{}{}{}",
            value.created_at.to_rfc3339(),
            ATTRIBUTES_SEPARATOR,
            value.id
        );

        let lsi2_sk = format!("{}{}{}", value.name, ATTRIBUTES_SEPARATOR, value.id);

        Self {
            pk: value.customer_id,
            sk: value.id,
            lsi1_sk,
            lsi2_sk,
            is_locked: value.is_locked,
            updated_at: value.updated_at,
        }
    }
}
