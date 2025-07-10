use crate::quotations::models::dynamodb_requests::BatchDeleteQuotationObject;
use crate::quotations::models::quotation::{Quotation, QuoteStatus};
use crate::repositories::quotations::{
    DynamodbQuote, QueryBy, QuotationsRepository, ATTRIBUTES_SEPARATOR,
};
use crate::shared::error::Error;
use crate::shared::{CustomerId, ProjectId, QueryResponse, QuoteId, Result};
use crate::utils::dynamodb_key_codec::DynamodbKeyCodec;
use async_trait::async_trait;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::types::{AttributeValue, DeleteRequest, ReturnValue, WriteRequest};
use chrono::{DateTime, Utc};
use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use serde_dynamo::{from_items, to_item};
use serde_enum_str::Serialize_enum_str;
use std::collections::HashMap;

#[derive(Serialize_enum_str)]
enum TableIndex {
    #[serde(rename = "LSI1_ProjectAndCreationDateTime")]
    LSI1ProjectsAndCreationDateTime,
    #[serde(rename = "GSI1_QuoteStatus")]
    GSI1QuoteStatus,
    #[serde(rename = "GSI2_QuoteIsPendingReview")]
    GSI2IsPendingReview,
}

#[derive(Clone)]
pub struct DynamodbQuotations {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbQuotations {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl QuotationsRepository for DynamodbQuotations {
    async fn create(&self, quotation: Quotation) -> Result<()> {
        let dynamodb_quotation = DynamodbQuote::from(quotation);
        let item = to_item(dynamodb_quotation).expect("error converting to dynamodb item");
        let response = self
            .client
            .put_item()
            .set_item(Some(item))
            .table_name(&self.table)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::UnknownError),
        }
    }

    async fn delete(&self, customer_id: CustomerId, quotation_id: QuoteId) -> Result<()> {
        let response = self
            .client
            .delete_item()
            .table_name(&self.table)
            .key("pk", AttributeValue::S(customer_id))
            .key("sk", AttributeValue::S(quotation_id))
            .condition_expression("NOT begins_with(gsi1_sk, :payedStatus)")
            .set_expression_attribute_values(Some(HashMap::from([(
                String::from(":payedStatus"),
                AttributeValue::S(QuoteStatus::Payed.to_string()),
            )])))
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => match err.as_service_error() {
                Some(service_error) => match service_error {
                    DeleteItemError::ConditionalCheckFailedException(_) => {
                        Err(Error::DeletePayedQuotation)
                    }
                    delete_item_error => {
                        tracing::error!("{delete_item_error:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => {
                    tracing::error!("{err:?}");
                    Err(Error::UnknownError)
                }
            },
        }
    }

    async fn get(&self, customer_id: CustomerId, quotation_id: QuoteId) -> Result<Quotation> {
        let response = self
            .client
            .get_item()
            .table_name(&self.table)
            .set_key(Some(HashMap::from([
                (String::from("pk"), AttributeValue::S(customer_id)),
                (String::from("sk"), AttributeValue::S(quotation_id)),
            ])))
            .send()
            .await;

        match response {
            Ok(output) => match output.item {
                Some(item) => match from_item::<DynamodbQuote>(item) {
                    Ok(dynamodb_quote) => Ok(dynamodb_quote.try_into()?),
                    Err(err) => {
                        tracing::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => Err(Error::ItemNotFoundError),
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

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
    ) -> Result<QueryResponse<Vec<Quotation>, String>> {
        let mut query = {
            match query_by {
                QueryBy::Customer => {
                    let customer_id = customer_id
                        .ok_or(Error::MissingRequiredParameter(String::from("customer_id")))?;

                    if let Some(status) = status {
                        self.status_query(customer_id, status, project_id)
                    } else {
                        let project_id = project_id
                            .ok_or(Error::MissingRequiredParameter(String::from("project_id")))?;

                        self.datetime_range_query(customer_id, project_id, from, to)
                    }
                }
                QueryBy::IsPendingReview => self.is_pending_review_query(),
            }
        };

        query = query
            .table_name(&self.table)
            .limit(limit)
            .set_exclusive_start_key(DynamodbKeyCodec::decode_from_base64(cursor))
            .scan_index_forward(false);

        let response = query.send().await;

        match response {
            Ok(output) => {
                let items = output.items().to_vec();
                match from_items::<_, DynamodbQuote>(items) {
                    Ok(dynamodb_quotations) => {
                        let mut quotations = Vec::with_capacity(dynamodb_quotations.len());
                        for item in dynamodb_quotations {
                            quotations.push(item.try_into()?);
                        }
                        Ok(QueryResponse {
                            data: quotations,
                            cursor: DynamodbKeyCodec::encode_to_base64(output.last_evaluated_key()),
                        })
                    }
                    Err(err) => {
                        tracing::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                }
            }
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn update(
        &self,
        customer_id: CustomerId,
        project_id: ProjectId,
        quotation_id: QuoteId,
        status: Option<QuoteStatus>,
    ) -> Result<Quotation> {
        let mut update_expression = String::from("SET updated_at = :updated_at");
        let mut expression_attribute_values: HashMap<String, AttributeValue> = [
            (
                String::from(":updated_at"),
                AttributeValue::S(Utc::now().to_rfc3339()),
            ),
            (
                String::from(":payedStatus"),
                AttributeValue::S(QuoteStatus::Payed.to_string()),
            ),
        ]
        .into_iter()
        .collect();

        if let Some(status) = status {
            let gsi1_sk = format!(
                "{}{ATTRIBUTES_SEPARATOR}{}{ATTRIBUTES_SEPARATOR}{}",
                status, project_id, quotation_id,
            );
            update_expression.push_str(", #gsi1_sk = :gsi1_sk");
            expression_attribute_values
                .insert(String::from(":gsi1_sk"), AttributeValue::S(gsi1_sk));
        }

        let response = self
            .client
            .update_item()
            .table_name(&self.table)
            .key("pk", AttributeValue::S(customer_id))
            .key("sk", AttributeValue::S(quotation_id))
            .condition_expression("NOT begins_with(gsi1_sk, :payedStatus)")
            .update_expression(update_expression)
            .set_expression_attribute_values(Some(expression_attribute_values))
            .return_values(ReturnValue::AllNew)
            .send()
            .await;

        match response {
            Ok(output) => match output.attributes {
                Some(item) => match from_item::<Quotation>(item) {
                    Ok(quotation) => Ok(quotation),
                    Err(err) => {
                        tracing::error!("{err:?}");
                        Err(Error::UnknownError)
                    }
                },
                None => Err(Error::ItemNotFoundError),
            },
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }

    async fn batch_delete(&self, data: Vec<BatchDeleteQuotationObject>) -> Result<()> {
        let write_requests = data
            .into_iter()
            .map(|delete_object| {
                WriteRequest::builder()
                    .delete_request(
                        DeleteRequest::builder()
                            .set_key(Some(HashMap::from([
                                (
                                    String::from("pk"),
                                    AttributeValue::S(delete_object.customer_id),
                                ),
                                (
                                    String::from("sk"),
                                    AttributeValue::S(delete_object.quotation_id),
                                ),
                            ])))
                            .build()
                            .unwrap(),
                    )
                    .build()
            })
            .collect();

        let response = self
            .client
            .batch_write_item()
            .request_items(&self.table, write_requests)
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}

impl DynamodbQuotations {
    fn status_query(
        &self,
        customer_id: CustomerId,
        status: QuoteStatus,
        project_id: Option<ProjectId>,
    ) -> QueryFluentBuilder {
        let prefix = format!(
            "{}{ATTRIBUTES_SEPARATOR}{}",
            status,
            project_id.unwrap_or_default()
        );

        let expression_attribute_values: HashMap<String, AttributeValue> = [
            (String::from(":customer_id"), AttributeValue::S(customer_id)),
            (String::from(":prefix"), AttributeValue::S(prefix)),
        ]
        .into_iter()
        .collect();

        self.client
            .query()
            .index_name(TableIndex::GSI1QuoteStatus.to_string())
            .key_condition_expression("pk = :customer_id AND begins_with(gsi1_sk, :prefix)")
            .set_expression_attribute_values(Some(expression_attribute_values))
    }

    fn datetime_range_query(
        &self,
        customer_id: CustomerId,
        project_id: ProjectId,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> QueryFluentBuilder {
        let lower_bound = format!(
            "{project_id}{ATTRIBUTES_SEPARATOR}{}",
            from.unwrap_or(DateTime::<Utc>::UNIX_EPOCH).to_rfc3339()
        );
        let upper_bound = format!(
            "{project_id}{ATTRIBUTES_SEPARATOR}{}",
            to.unwrap_or(Utc::now()).to_rfc3339()
        );

        let expression_attribute_values: HashMap<String, AttributeValue> = [
            (String::from(":customer_id"), AttributeValue::S(customer_id)),
            (String::from(":lower_bound"), AttributeValue::S(lower_bound)),
            (String::from(":upper_bound"), AttributeValue::S(upper_bound)),
        ]
        .into_iter()
        .collect();

        self.client
            .query()
            .index_name(TableIndex::LSI1ProjectsAndCreationDateTime.to_string())
            .key_condition_expression(
                "pk = :customer_id AND lsi1_sk BETWEEN :lower_bound AND :upper_bound",
            )
            .set_expression_attribute_values(Some(expression_attribute_values))
    }

    fn is_pending_review_query(&self) -> QueryFluentBuilder {
        self.client
            .query()
            .index_name(TableIndex::GSI2IsPendingReview.to_string())
            .key_condition_expression("gsi2_pk = :isPendingReview")
            .expression_attribute_values(":isPendingReview", AttributeValue::Bool(true))
    }
}
