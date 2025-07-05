use crate::parts::domain::dynamodb_requests::BatchDeletePartObject;
use crate::projects::models::requests::DeleteProjectInput;
use crate::quotations::domain::dynamodb_requests::BatchDeleteQuotationObject;
use crate::repositories::parts::PartsRepository;
use crate::repositories::projects::ProjectsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared;
use crate::shared::UseCase;
use api_boundary::parts::models::Part;
use api_boundary::quotations::models::Quotation;
use async_trait::async_trait;
use std::sync::Arc;

pub struct DeleteProjectUseCase {
    projects_repository: Arc<dyn ProjectsRepository>,
    quotations_repository: Arc<dyn QuotationsRepository>,
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl DeleteProjectUseCase {
    pub fn new(
        projects_repository: Arc<dyn ProjectsRepository>,
        quotations_repository: Arc<dyn QuotationsRepository>,
        parts_repository: Arc<dyn PartsRepository>,
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
impl UseCase<DeleteProjectInput, ()> for DeleteProjectUseCase {
    async fn execute(&self, input: DeleteProjectInput) -> crate::shared::Result<()> {
        let _ = self
            .projects_repository
            .try_delete_project(input.identity.id, input.project_id.clone())
            .await;

        let quotations_repository = self.quotations_repository.clone();
        let parts_repository = self.parts_repository.clone();
        let object_storage = self.object_storage.clone();

        Self::cascade_delete_quotations_for_project(
            input.project_id.clone(),
            quotations_repository,
            parts_repository,
            object_storage,
        )
        .await;

        Ok(())
    }
}

impl DeleteProjectUseCase {
    async fn cascade_delete_quotations_for_project(
        project_id: String,
        quotations_repository: Arc<dyn QuotationsRepository>,
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) {
        let page_limit = 25;
        let mut cursor = None;

        loop {
            let result = quotations_repository
                .query_quotations_for_project(project_id.clone(), page_limit, cursor)
                .await;

            match result {
                Ok(response) => {
                    if response.data.is_empty() {
                        break;
                    }

                    let _ = Self::delete_quotations(
                        &response.data,
                        quotations_repository.clone(),
                        parts_repository.clone(),
                        object_storage.clone(),
                    )
                    .await;

                    cursor = response.cursor;
                }
                Err(_) => cursor = None,
            }

            if cursor.is_none() {
                break;
            }
        }
    }

    async fn delete_quotations(
        quotations: &Vec<Quotation>,
        quotations_repository: Arc<dyn QuotationsRepository>,
        parts_repository: Arc<dyn PartsRepository>,
        object_repository: Arc<dyn ObjectStorage>,
    ) -> shared::Result<()> {
        for quotation in quotations {
            Self::cascade_delete_parts_for_quotation(
                quotation.id.clone(),
                parts_repository.clone(),
                object_repository.clone(),
            )
            .await;
        }

        let batch_delete_objects = quotations
            .iter()
            .map(|quotation| BatchDeleteQuotationObject {
                quotation_id: quotation.id.clone(),
                project_id: quotation.project_id.clone(),
            })
            .collect();

        quotations_repository
            .batch_delete_parts(batch_delete_objects)
            .await
    }

    async fn cascade_delete_parts_for_quotation(
        quotation_id: String,
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) {
        let page_limit = 25;
        let mut cursor = None;

        loop {
            let result = parts_repository
                .query_parts_for_quotation(quotation_id.clone(), cursor, page_limit)
                .await;

            match result {
                Ok(response) => {
                    if response.data.is_empty() {
                        break;
                    }

                    let _ = Self::delete_parts(&response.data, parts_repository.clone()).await;

                    Self::delete_associated_objects(&response.data, object_storage.clone()).await;

                    cursor = response.cursor;
                }
                Err(_) => cursor = None,
            }

            if cursor.is_none() {
                break;
            }
        }
    }

    async fn delete_parts(
        parts: &Vec<Part>,
        parts_repository: Arc<dyn PartsRepository>,
    ) -> shared::Result<()> {
        let batch_delete_objects = parts
            .iter()
            .map(|part| BatchDeletePartObject {
                part_id: part.id.clone(),
                quotation_id: part.quotation_id.clone(),
            })
            .collect();

        parts_repository
            .batch_delete_parts(batch_delete_objects)
            .await
    }

    async fn delete_associated_objects(parts: &Vec<Part>, object_storage: Arc<dyn ObjectStorage>) {
        // Merge model, render, and drawing file urls into one array to bulk delete all
        // the objects at the same time.
        let urls = parts
            .iter()
            .flat_map(|part| {
                vec![
                    Some(&part.model_file),
                    Some(&part.render_file),
                    Option::from(&part.drawing_file),
                ]
            })
            .filter_map(|file| file.map(|f| f.url.as_ref()))
            .collect();

        let _ = object_storage.bulk_delete_objects(urls).await;
    }
}
