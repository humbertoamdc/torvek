use crate::parts::models::inputs::GetPartInput;
use crate::parts::models::part::Part;
use crate::repositories::parts::PartsRepository;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;

pub struct GetPart<P>
where
    P: PartsRepository,
{
    parts_repository: Arc<P>,
}

impl<P> GetPart<P>
where
    P: PartsRepository,
{
    pub fn new(parts_repository: Arc<P>) -> Self {
        Self { parts_repository }
    }
}

#[async_trait]
impl<P> UseCase<GetPartInput, Part> for GetPart<P>
where
    P: PartsRepository,
{
    async fn execute(&self, input: GetPartInput) -> Result<Part> {
        self.parts_repository
            .get(input.identity.id, input.part_id)
            .await
    }
}
