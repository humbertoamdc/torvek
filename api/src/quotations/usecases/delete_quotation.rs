use crate::parts::models::dynamodb_requests::BatchDeletePartObject;
use crate::parts::models::part::Part;
use crate::quotations::models::inputs::DeleteQuotationInput;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotations::QuotationsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared;
use crate::shared::UseCase;
use async_trait::async_trait;
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
impl UseCase<DeleteQuotationInput, ()> for DeleteQuotationUseCase {
    async fn execute(&self, input: DeleteQuotationInput) -> crate::shared::Result<()> {
        self.quotations_repository
            .delete(input.project_id, input.quotation_id.clone())
            .await?;

        let parts_repository = self.parts_repository.clone();
        let object_storage = self.object_storage.clone();

        Self::cascade_delete_parts_for_quotation(
            input.quotation_id.clone(),
            parts_repository,
            object_storage,
        )
        .await;

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
