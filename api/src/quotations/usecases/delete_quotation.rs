use crate::parts::models::dynamodb_requests::BatchDeletePartObject;
use crate::parts::models::part::Part;
use crate::quotations::models::inputs::DeleteQuotationInput;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared;
use crate::shared::{CustomerId, QuoteId, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct DeleteQuotation<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    quotations_repository: Arc<Q>,
    parts_repository: Arc<P>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl<Q, P> DeleteQuotation<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    pub fn new(
        quotations_repository: Arc<Q>,
        parts_repository: Arc<P>,
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
impl<Q, P> UseCase<DeleteQuotationInput, ()> for DeleteQuotation<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    async fn execute(&self, input: DeleteQuotationInput) -> crate::shared::Result<()> {
        self.quotations_repository
            .delete(input.identity.id.clone(), input.quotation_id.clone())
            .await?;

        self.cascade_delete_parts_for_quotation(input.identity.id, input.quotation_id)
            .await;

        Ok(())
    }
}

impl<Q, P> DeleteQuotation<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
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
