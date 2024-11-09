use crate::parts::domain::dynamodb_requests::BatchDeletePartObject;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared;
use crate::shared::UseCase;
use api_boundary::parts::models::Part;
use api_boundary::quotations::requests::DeleteQuotationRequest;
use axum::async_trait;
use std::sync::Arc;

pub struct DeleteQuotationUseCase {
    quotations_repository: Arc<dyn QuotationsRepository>,
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl DeleteQuotationUseCase {
    pub fn new(
        quotations_repository: Arc<dyn QuotationsRepository>,
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            quotations_repository,
            parts_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl UseCase<DeleteQuotationRequest, ()> for DeleteQuotationUseCase {
    async fn execute(&self, request: DeleteQuotationRequest) -> crate::shared::Result<()> {
        self.quotations_repository
            .try_delete_quotation(request.project_id, request.quotation_id.clone())
            .await?;

        let parts_repository = self.parts_repository.clone();
        let object_storage = self.object_storage.clone();

        // Spawn a background task to cascade delete all the parts belonging to the project.
        // This allows us to return early because we don't have to wait for the deletes to
        // happen.
        tokio::task::spawn(async move {
            Self::cascade_delete_parts_for_quotation(
                request.quotation_id.clone(),
                parts_repository,
                object_storage,
            )
            .await;
        });

        Ok(())
    }
}

impl DeleteQuotationUseCase {
    async fn cascade_delete_parts_for_quotation(
        quotation_id: String,
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) {
        let page_limit = 25;
        let mut cursor = None;

        loop {
            let result = parts_repository
                .query_parts_for_quotation(quotation_id.clone(), page_limit, cursor)
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
        let model_urls = parts
            .iter()
            .map(|part| part.model_file.url.as_ref())
            .collect::<Vec<&str>>();
        let render_urls = parts
            .iter()
            .map(|part| part.render_file.url.as_ref())
            .collect::<Vec<&str>>();
        let drawing_urls = parts
            .iter()
            .filter(|part| part.drawing_file.is_some())
            .map(|part| part.drawing_file.as_ref().unwrap().url.as_ref())
            .collect::<Vec<&str>>();

        let _ = object_storage.bulk_delete_objects(model_urls).await;
        let _ = object_storage.bulk_delete_objects(render_urls).await;
        if !drawing_urls.is_empty() {
            let _ = object_storage.bulk_delete_objects(drawing_urls).await;
        }
    }
}
