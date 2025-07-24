use crate::parts::models::dynamodb_requests::{BatchDeletePartObject, UpdatablePart};
use crate::parts::models::part::{Part, PartAttributes, PartProcess, PartQuote};
use crate::shared::error::Error;
use crate::shared::error::Error::UnknownError;
use crate::shared::file::File;
use crate::shared::{CustomerId, PartId, PartQuoteId, ProjectId, QueryResponse, QuoteId, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;

pub const ATTRIBUTES_SEPARATOR: &str = "&";

#[async_trait]
pub trait PartsRepository: Send + Sync + 'static {
    type TransactionItem;
    async fn delete(&self, customer_id: CustomerId, part_id: PartId) -> Result<Part>;
    async fn get(&self, customer_id: CustomerId, part_id: PartId) -> Result<Part>;
    async fn query(
        &self,
        customer_id: CustomerId,
        quotation_id: QuoteId,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Part>, String>>;
    async fn update(&self, updatable_part: UpdatablePart) -> Result<Part>;
    async fn batch_create(&self, parts: Vec<Part>) -> Result<()>;
    async fn batch_delete(&self, data: Vec<BatchDeletePartObject>) -> Result<()>;
    async fn batch_get(&self, quotation_and_part_ids: Vec<(QuoteId, PartId)>) -> Result<Vec<Part>>;
    fn transaction_create_part_quotes(
        &self,
        customer_id: CustomerId,
        part_id: PartId,
        selected_part_quote_id: PartQuoteId,
        part_quotes: Vec<PartQuote>,
    ) -> Self::TransactionItem;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamodbPart {
    pub pk: CustomerId,
    pub sk: PartId,
    /// quote_id&created_at&part_id
    pub lsi1_sk: String,
    pub project_id: ProjectId,
    pub model_file: File,
    pub render_file: Option<File>,
    pub drawing_file: Option<File>,
    pub process: PartProcess,
    pub attributes: PartAttributes,
    pub quantity: u64,
    pub selected_part_quote_id: Option<PartQuoteId>,
    pub part_quotes: Option<Vec<PartQuote>>,
    pub updated_at: DateTime<Utc>,
}

impl TryInto<Part> for DynamodbPart {
    type Error = Error;

    fn try_into(self) -> std::result::Result<Part, Self::Error> {
        let mut quote_id = None::<QuoteId>;
        let mut created_at = None::<DateTime<Utc>>;

        let lsi1_sk_attributes = self
            .lsi1_sk
            .split(ATTRIBUTES_SEPARATOR)
            .collect::<Vec<&str>>();

        if let [sk_quote_id, sk_created_at, _] = &lsi1_sk_attributes[..] {
            quote_id = Some(sk_quote_id.to_string());
            created_at = Some(DateTime::<Utc>::from_str(sk_created_at).unwrap());
        }

        let item = Part {
            id: self.sk.clone(),
            customer_id: self.pk,
            project_id: self.project_id,
            quotation_id: quote_id.ok_or_else(|| {
                tracing::error!(
                    "quote id is required but not found for part with id {}",
                    self.sk
                );
                UnknownError
            })?,
            model_file: self.model_file,
            render_file: self.render_file,
            drawing_file: self.drawing_file,
            process: self.process,
            attributes: self.attributes,
            quantity: self.quantity,
            selected_part_quote_id: self.selected_part_quote_id,
            part_quotes: self.part_quotes,
            created_at: created_at.ok_or_else(|| {
                tracing::error!(
                    "created_at is required but not found for part with id {}",
                    self.sk
                );
                UnknownError
            })?,
            updated_at: self.updated_at,
        };

        Ok(item)
    }
}

impl From<Part> for DynamodbPart {
    fn from(value: Part) -> Self {
        let lsi1_sk = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}",
            value.quotation_id,
            value.created_at.to_rfc3339(),
            value.id
        );

        Self {
            pk: value.customer_id,
            sk: value.id,
            lsi1_sk,
            project_id: value.project_id,
            model_file: value.model_file,
            render_file: value.render_file,
            drawing_file: value.drawing_file,
            process: value.process,
            attributes: value.attributes,
            quantity: value.quantity,
            selected_part_quote_id: value.selected_part_quote_id,
            part_quotes: value.part_quotes,
            updated_at: value.updated_at,
        }
    }
}
