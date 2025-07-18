use crate::quotations::models::dynamodb_requests::BatchDeleteQuotationObject;
use crate::quotations::models::quotation::{Quotation, QuoteStatus};
use crate::shared::error::Error;
use crate::shared::error::Error::UnknownError;
use crate::shared::{CustomerId, ProjectId, QueryResponse, QuoteId, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

pub const ATTRIBUTES_SEPARATOR: &str = "&";

pub enum QueryBy {
    Customer,
    IsPendingReview,
}

#[async_trait]
pub trait QuotesRepository: Send + Sync + 'static {
    type TransactionItem;
    async fn create(&self, quotation: Quotation) -> Result<()>;
    /// Delete quotation ONLY if it is not in `PAYED` status.
    async fn delete(&self, customer_id: CustomerId, quotation_id: QuoteId) -> Result<()>;
    async fn get(&self, customer_id: CustomerId, quotation_id: QuoteId) -> Result<Quotation>;
    async fn query(
        &self,
        customer_id: Option<CustomerId>,
        project_id: Option<ProjectId>,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        status: Option<QuoteStatus>,
        query_by: QueryBy,
        limit: i32,
        cursor: Option<String>,
    ) -> Result<QueryResponse<Vec<Quotation>, String>>;

    async fn update(
        &self,
        customer_id: CustomerId,
        project_id: ProjectId,
        quotation_id: QuoteId,
        status: Option<QuoteStatus>,
    ) -> Result<Quotation>;
    async fn batch_delete(&self, data: Vec<BatchDeleteQuotationObject>) -> Result<()>;
    fn transaction_update(
        &self,
        customer_id: CustomerId,
        project_id: ProjectId,
        quote_id: QuoteId,
        old_status: QuoteStatus,
        new_status: QuoteStatus,
    ) -> Self::TransactionItem;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamodbQuote {
    pub pk: CustomerId,
    pub sk: QuoteId,
    /// project_id&created_at&quote_id
    pub lsi1_sk: String,
    /// status&project_id&quote_id
    pub gsi1_sk: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// is_pending_review
    pub gsi2_pk: Option<String>,
    /// created_at&quote_id
    pub gsi2_sk: String,
    pub name: String,
    pub updated_at: DateTime<Utc>,
}

impl TryInto<Quotation> for DynamodbQuote {
    type Error = Error;

    fn try_into(self) -> std::result::Result<Quotation, Self::Error> {
        let mut project_id = None::<ProjectId>;
        let mut created_at = None::<DateTime<Utc>>;
        let mut status = None::<QuoteStatus>;

        let lsi1_sk_attributes = self
            .lsi1_sk
            .split(ATTRIBUTES_SEPARATOR)
            .collect::<Vec<&str>>();
        let gsi1_sk_attributes = self.gsi1_sk.split_once(ATTRIBUTES_SEPARATOR);

        if let [sk_project_id, sk_created_at, _] = &lsi1_sk_attributes[..] {
            project_id = Some(sk_project_id.to_string());
            created_at = Some(DateTime::<Utc>::from_str(sk_created_at).unwrap());
        }
        if let Some((sk_status, _)) = gsi1_sk_attributes {
            if let Ok(parsed_status) = sk_status.parse() {
                status = Some(parsed_status);
            }
        }

        let item = Quotation {
            id: self.sk.clone(),
            customer_id: self.pk,
            project_id: project_id.ok_or_else(|| {
                tracing::error!(
                    "project id is required but not found for quote with id {}",
                    self.sk
                );
                UnknownError
            })?,
            name: self.name.clone(),
            status: status.ok_or_else(|| {
                tracing::error!(
                    "status is required but not found for quote with id {}",
                    self.sk
                );
                UnknownError
            })?,
            created_at: created_at.ok_or_else(|| {
                tracing::error!(
                    "created_at is required but not found for quote with id {}",
                    self.sk
                );
                UnknownError
            })?,
            updated_at: self.updated_at,
        };

        Ok(item)
    }
}

impl From<Quotation> for DynamodbQuote {
    fn from(value: Quotation) -> Self {
        let lsi1_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}",
            value.project_id,
            value.created_at.to_rfc3339(),
            value.id
        );

        let gsi1_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}",
            value.status, value.project_id, value.id
        );
        let gsi2_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}",
            value.created_at.to_rfc3339(),
            value.id
        );

        let gsi2_pk = if value.status == QuoteStatus::PendingReview {
            Some(String::from("true"))
        } else {
            None
        };

        Self {
            pk: value.customer_id,
            sk: value.id,
            lsi1_sk,
            gsi1_sk,
            gsi2_pk,
            gsi2_sk,
            name: value.name,
            updated_at: value.updated_at,
        }
    }
}
