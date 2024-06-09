use std::sync::Arc;
use std::time::Duration;

use axum::async_trait;

use api_boundary::parts::errors::PartsError;
use api_boundary::parts::requests::QueryPartsForQuotationRequest;
use api_boundary::parts::responses::QueryPartsForQuotationResponse;
use url::Url;

use crate::parts::repositories::parts::PartsRepository;
use crate::parts::services::object_storage::ObjectStorage;
use crate::shared::usecase::UseCase;

static PRESIGNED_URLS_GET_DURATION_SECONDS: u64 = 3600;

pub struct QueryPartsForQuotationUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl QueryPartsForQuotationUseCase {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            parts_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl UseCase<QueryPartsForQuotationRequest, QueryPartsForQuotationResponse, PartsError>
    for QueryPartsForQuotationUseCase
{
    async fn execute(
        &self,
        request: QueryPartsForQuotationRequest,
    ) -> Result<QueryPartsForQuotationResponse, PartsError> {
        let mut parts = self
            .parts_repository
            .query_parts_for_quotation(request.client_id, request.project_id, request.quotation_id)
            .await?;

        // Add render file presigned url.
        for part in parts.iter_mut() {
            // TODO: Handle error.
            // For both cases, we can recover from the error, we will just leave the presigned url field
            // as none. In the future we might page to have someone have a look at the url format.
            let part_render_url =
                Url::parse(&part.render_file.url).expect("invalid render file url");
            let part_render_url_path = part_render_url
                .path()
                .strip_prefix("/")
                .unwrap()
                .split_once("/")
                .unwrap()
                .1
                .to_string();
            let presigned_url = self
                .object_storage
                .get_object_presigned_url(
                    part_render_url_path,
                    Duration::from_secs(PRESIGNED_URLS_GET_DURATION_SECONDS),
                )
                .await?;

            part.render_file.presigned_url = Some(presigned_url);
        }

        Ok(QueryPartsForQuotationResponse::new(parts))
    }
}
