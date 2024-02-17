use std::sync::Arc;

use axum::async_trait;

use api_boundary::parts::requests::UpdatePartRequest;

use crate::parts::domain::errors::PartsError;
use crate::parts::domain::updatable_part::UpdatablePart;
use crate::parts::repositories::parts::PartsRepository;
use crate::shared::usecase::UseCase;

pub struct UpdatePartUseCase {
    parts_repository: Arc<dyn PartsRepository>,
}

impl UpdatePartUseCase {
    pub const fn new(parts_repository: Arc<dyn PartsRepository>) -> Self {
        Self { parts_repository }
    }
}

#[async_trait]
impl UseCase<UpdatePartRequest, (), PartsError> for UpdatePartUseCase {
    async fn execute(&self, request: UpdatePartRequest) -> Result<(), PartsError> {
        let updatable_part = UpdatablePart::from(&request);

        self.parts_repository.update_part(updatable_part).await
    }
}
