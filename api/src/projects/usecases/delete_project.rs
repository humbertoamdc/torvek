use crate::parts::models::dynamodb_requests::BatchDeletePartObject;
use crate::parts::models::part::Part;
use crate::projects::models::inputs::DeleteProjectInput;
use crate::quotations::models::dynamodb_requests::BatchDeleteQuotationObject;
use crate::quotations::models::quotation::Quotation;
use crate::repositories::parts::PartsRepository;
use crate::repositories::projects::ProjectsRepository;
use crate::repositories::quotes::{QueryBy, QuotesRepository};
use crate::services::object_storage::ObjectStorage;
use crate::shared;
use crate::shared::{CustomerId, ProjectId, QuoteId, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct DeleteProject<Pro, Quo, Par>
where
    Pro: ProjectsRepository,
    Quo: QuotesRepository,
    Par: PartsRepository,
{
    projects_repository: Arc<Pro>,
    quotations_repository: Arc<Quo>,
    parts_repository: Arc<Par>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl<Pro, Quo, Par> DeleteProject<Pro, Quo, Par>
where
    Pro: ProjectsRepository,
    Quo: QuotesRepository,
    Par: PartsRepository,
{
    pub fn new(
        projects_repository: Arc<Pro>,
        quotations_repository: Arc<Quo>,
        parts_repository: Arc<Par>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            projects_repository,
            quotations_repository,
            parts_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl<Pro, Quo, Par> UseCase<DeleteProjectInput, ()> for DeleteProject<Pro, Quo, Par>
where
    Pro: ProjectsRepository,
    Quo: QuotesRepository,
    Par: PartsRepository,
{
    async fn execute(&self, input: DeleteProjectInput) -> crate::shared::Result<()> {
        let _ = self
            .projects_repository
            .delete(input.identity.id.clone(), input.project_id.clone())
            .await;

        self.cascade_delete_quotations_for_project(input.identity.id, input.project_id.clone())
            .await;

        Ok(())
    }
}

impl<Pro, Quo, Par> DeleteProject<Pro, Quo, Par>
where
    Pro: ProjectsRepository,
    Quo: QuotesRepository,
    Par: PartsRepository,
{
    async fn cascade_delete_quotations_for_project(
        &self,
        customer_id: CustomerId,
        project_id: ProjectId,
    ) {
        let page_limit = 25;
        let mut cursor = None;

        loop {
            let result = self
                .quotations_repository
                .query(
                    Some(customer_id.clone()),
                    Some(project_id.clone()),
                    None,
                    None,
                    None,
                    QueryBy::Customer,
                    page_limit,
                    cursor,
                )
                .await;

            match result {
                Ok(response) => {
                    if response.data.is_empty() {
                        break;
                    }

                    let _ = self.delete_quotations(&response.data).await;

                    cursor = response.cursor;
                }
                Err(_) => cursor = None,
            }

            if cursor.is_none() {
                break;
            }
        }
    }

    async fn delete_quotations(&self, quotations: &Vec<Quotation>) -> shared::Result<()> {
        for quotation in quotations {
            self.cascade_delete_parts_for_quotation(
                quotation.customer_id.clone(),
                quotation.id.clone(),
            )
            .await;
        }

        let batch_delete_objects = quotations
            .iter()
            .map(|quotation| BatchDeleteQuotationObject {
                customer_id: quotation.customer_id.clone(),
                quotation_id: quotation.id.clone(),
            })
            .collect();

        self.quotations_repository
            .batch_delete(batch_delete_objects)
            .await
    }

    async fn cascade_delete_parts_for_quotation(
        &self,
        customer_id: CustomerId,
        quotation_id: QuoteId,
    ) {
        let page_limit = 25;
        let mut cursor = None;

        loop {
            let result = self
                .parts_repository
                .query(
                    customer_id.clone(),
                    quotation_id.clone(),
                    cursor,
                    page_limit,
                )
                .await;

            match result {
                Ok(response) => {
                    if response.data.is_empty() {
                        break;
                    }

                    let _ = self.delete_parts(&response.data).await;

                    self.delete_associated_objects(&response.data).await;

                    cursor = response.cursor;
                }
                Err(_) => cursor = None,
            }

            if cursor.is_none() {
                break;
            }
        }
    }

    async fn delete_parts(&self, parts: &Vec<Part>) -> shared::Result<()> {
        let batch_delete_objects = parts
            .iter()
            .map(|part| BatchDeletePartObject {
                customer_id: part.customer_id.clone(),
                part_id: part.id.clone(),
            })
            .collect();

        self.parts_repository
            .batch_delete(batch_delete_objects)
            .await
    }

    async fn delete_associated_objects(&self, parts: &Vec<Part>) {
        // Merge model, render, and drawing file urls into one array to bulk delete all
        // the objects at the same time.
        let urls = parts
            .iter()
            .flat_map(|part| {
                vec![
                    Some(&part.model_file),
                    Option::from(&part.render_file),
                    Option::from(&part.drawing_file),
                ]
            })
            .filter_map(|file| file.map(|f| f.key.as_ref()))
            .collect();

        let _ = self.object_storage.bulk_delete_objects(urls).await;
    }
}
