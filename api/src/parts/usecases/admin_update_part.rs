use std::sync::Arc;

use api_boundary::parts::errors::PartsError;
use api_boundary::parts::models::Part;
use axum::async_trait;

use api_boundary::parts::requests::AdminUpdatePartRequest;

use crate::parts::domain::updatable_part::UpdatablePart;
use crate::parts::repositories::parts::PartsRepository;
use crate::shared::usecase::UseCase;

pub struct AdminUpdatePartUseCase {
    parts_repository: Arc<dyn PartsRepository>,
}

impl AdminUpdatePartUseCase {
    pub fn new(parts_repository: Arc<dyn PartsRepository>) -> Self {
        Self { parts_repository }
    }
}

#[async_trait]
impl UseCase<AdminUpdatePartRequest, Part, PartsError> for AdminUpdatePartUseCase {
    async fn execute(&self, request: AdminUpdatePartRequest) -> Result<Part, PartsError> {
        let updatable_part = UpdatablePart::from(&request);

        self.parts_repository.update_part(updatable_part).await
    }
}
