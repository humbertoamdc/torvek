use crate::projects::models::project::Project;
use crate::repositories::projects::{DynamodbProject, ProjectsRepository};
use crate::shared::error::Error;
use crate::shared::{QueryResponse, Result};
use crate::utils::dynamodb_key_codec::DynamodbKeyCodec;
use async_trait::async_trait;
use aws_sdk_dynamodb::operation::delete_item::DeleteItemError;
use aws_sdk_dynamodb::operation::query::builders::QueryFluentBuilder;
use aws_sdk_dynamodb::types::AttributeValue;
use chrono::{DateTime, Utc};
use serde_dynamo::aws_sdk_dynamodb_1::from_item;
use serde_dynamo::{from_items, to_item};
use serde_enum_str::Serialize_enum_str;
use std::collections::HashMap;

#[derive(Serialize_enum_str)]
pub enum TableIndex {
    #[serde(rename = "LSI1_CreationTimestamp")]
    LSI1CreationTimestamp,
    #[serde(rename = "LSI2_ProjectName")]
    LSI2ProjectName,
}

#[derive(Clone)]
pub struct DynamodbProjects {
    client: aws_sdk_dynamodb::Client,
    table: String,
}

impl DynamodbProjects {
    pub fn new(client: aws_sdk_dynamodb::Client, table: String) -> Self {
        Self { client, table }
    }
}

#[async_trait]
impl ProjectsRepository for DynamodbProjects {
    async fn create(&self, project: Project) -> Result<()> {
        let dynamodb_project = DynamodbProject::from(project);
        let item = to_item(dynamodb_project).expect("error converting to dynamodb item");
        let response = self
            .client
            .put_item()
            .set_item(Some(item))
            .table_name(&self.table)
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

    async fn delete(&self, customer_id: String, project_id: String) -> Result<()> {
        let response = self
            .client
            .delete_item()
            .table_name(&self.table)
            .key("pk", AttributeValue::S(customer_id))
            .key("sk", AttributeValue::S(project_id))
            .condition_expression("is_locked <> :is_locked")
            .set_expression_attribute_values(Some(HashMap::from([(
                String::from(":is_locked"),
                AttributeValue::Bool(true),
            )])))
            .send()
            .await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => match err.as_service_error() {
                Some(service_error) => match service_error {
                    DeleteItemError::ConditionalCheckFailedException(conditional_check_error) => {
                        tracing::error!("{conditional_check_error:?}");
                        Err(Error::DeleteLockedProject)
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

    async fn get(&self, customer_id: String, project_id: String) -> Result<Project> {
        let response = self
            .client
            .get_item()
            .table_name(&self.table)
            .set_key(Some(HashMap::from([
                (String::from("pk"), AttributeValue::S(customer_id)),
                (String::from("sk"), AttributeValue::S(project_id)),
            ])))
            .send()
            .await;

        match response {
            Ok(output) => match output.item {
                Some(item) => match from_item::<DynamodbProject>(item) {
                    Ok(dynamodb_project) => Ok(dynamodb_project.try_into()?),
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
        customer_id: String,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
        name: Option<String>,
        cursor: Option<String>,
        limit: i32,
    ) -> Result<QueryResponse<Vec<Project>, String>> {
        let mut query = {
            if let Some(name) = name {
                self.name_query(customer_id, name)
            } else {
                self.datetime_range_query(customer_id, from, to)
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
                match from_items::<_, DynamodbProject>(items) {
                    Ok(dynamodb_projects) => {
                        let mut projects = Vec::with_capacity(dynamodb_projects.len());
                        for item in dynamodb_projects {
                            projects.push(item.try_into()?);
                        }
                        Ok(QueryResponse {
                            data: projects,
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
}

impl DynamodbProjects {
    fn name_query(&self, customer_id: String, name: String) -> QueryFluentBuilder {
        let expression_attribute_values: HashMap<String, AttributeValue> = [
            (String::from(":customer_id"), AttributeValue::S(customer_id)),
            (String::from(":name"), AttributeValue::S(name)),
        ]
        .into_iter()
        .collect();

        self.client
            .query()
            .index_name(TableIndex::LSI2ProjectName.to_string())
            .key_condition_expression("pk = :customer_id AND begins_with(lsi2_sk, :name)")
            .set_expression_attribute_values(Some(expression_attribute_values))
    }

    fn datetime_range_query(
        &self,
        customer_id: String,
        from: Option<DateTime<Utc>>,
        to: Option<DateTime<Utc>>,
    ) -> QueryFluentBuilder {
        let lower_bound = from.unwrap_or(DateTime::<Utc>::UNIX_EPOCH).to_rfc3339();
        let upper_bound = to.unwrap_or(Utc::now()).to_rfc3339();

        let expression_attribute_values: HashMap<String, AttributeValue> = [
            (String::from(":customer_id"), AttributeValue::S(customer_id)),
            (String::from(":lower_bound"), AttributeValue::S(lower_bound)),
            (String::from(":upper_bound"), AttributeValue::S(upper_bound)),
        ]
        .into_iter()
        .collect();

        self.client
            .query()
            .index_name(TableIndex::LSI1CreationTimestamp.to_string())
            .key_condition_expression(
                "pk = :customer_id AND lsi1_sk BETWEEN :lower_bound AND :upper_bound",
            )
            .set_expression_attribute_values(Some(expression_attribute_values))
    }
}
