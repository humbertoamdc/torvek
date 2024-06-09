use std::collections::HashMap;
use std::sync::Arc;

use api_boundary::parts::errors::PartsError;
use axum::async_trait;

use api_boundary::parts::models::PartQuote;
use api_boundary::parts::requests::{
    QueryPartQuotesForPartsRequest, QueryPartQuotesForPartsResponse,
};

use crate::parts::repositories::part_quotes::PartQuotesRepository;
use crate::shared::usecase::UseCase;

pub struct QueryPartQuotesForPartsUseCase {
    part_quotes_repository: Arc<dyn PartQuotesRepository>,
}

impl QueryPartQuotesForPartsUseCase {
    pub fn new(part_quotes_repository: Arc<dyn PartQuotesRepository>) -> Self {
        Self {
            part_quotes_repository,
        }
    }
}

#[async_trait]
impl UseCase<QueryPartQuotesForPartsRequest, QueryPartQuotesForPartsResponse, PartsError>
    for QueryPartQuotesForPartsUseCase
{
    async fn execute(
        &self,
        request: QueryPartQuotesForPartsRequest,
    ) -> Result<QueryPartQuotesForPartsResponse, PartsError> {
        let mut part_quotes_by_part_id = HashMap::<String, Vec<PartQuote>>::new();

        for part_id in request.part_ids.into_iter() {
            let part_quotes = self
                .part_quotes_repository
                .query_part_quotes_for_part(part_id.clone())
                .await?;
            part_quotes_by_part_id.insert(part_id, part_quotes);
        }

        Ok(QueryPartQuotesForPartsResponse {
            part_quotes_by_part_id,
        })
    }
}
