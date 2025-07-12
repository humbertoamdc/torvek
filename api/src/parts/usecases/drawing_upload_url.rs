use crate::parts::models::dynamodb_requests::UpdatablePart;
use crate::parts::models::inputs::CreateDrawingUploadUrlInput;
use crate::parts::models::responses::CreateDrawingUploadUrlResponse;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::error::Error;
use crate::shared::file::File;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use uuid::{ContextV7, Timestamp, Uuid};

static DRAWING_FILES_BASE_FILE_PATH: &'static str = "parts/drawings";

pub struct CreateDrawingUploadUrl {
    parts_repository: Arc<dyn PartsRepository>,
    quotation_repository: Arc<dyn QuotesRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl CreateDrawingUploadUrl {
    pub const fn new(
        parts_repository: Arc<dyn PartsRepository>,
        quotation_repository: Arc<dyn QuotesRepository>,
        object_storage: Arc<dyn ObjectStorage>,
    ) -> Self {
        Self {
            parts_repository,
            quotation_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl UseCase<CreateDrawingUploadUrlInput, CreateDrawingUploadUrlResponse>
    for CreateDrawingUploadUrl
{
    async fn execute(
        &self,
        input: CreateDrawingUploadUrlInput,
    ) -> Result<CreateDrawingUploadUrlResponse> {
        if self.quotation_is_payed(&input).await? {
            return Err(Error::UpdatePartAfterPayingQuotation);
        }

        // We only want to preserve one drawing file per part. In the case where
        // a file has been previously uploaded, we will use the path for that file,
        // effectively overwriting the file and maintaining one drawing per part.
        let file_path = match input.file_url {
            Some(file_url) => file_url.path().strip_prefix("/").unwrap().to_string(),
            None => {
                let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));

                let file_id = format!("file_{}", bs58::encode(id).into_string());
                let file_extension = input.file_name.split(".").last().unwrap();
                let file_path = format!(
                    "{}/{}/{}.{}",
                    DRAWING_FILES_BASE_FILE_PATH, input.identity.id, file_id, file_extension
                );

                file_path
            }
        };

        let presigned_url = self
            .object_storage
            .put_object_presigned_url(&file_path, Duration::from_secs(300))
            .await?;
        let url = presigned_url.split("?").nth(0).unwrap().to_string();

        let updatable_part = UpdatablePart {
            id: input.part_id,
            customer_id: input.identity.id,
            quotation_id: input.quotation_id,
            drawing_file: Some(File::new(input.file_name, url.clone())),
            process: None,
            attributes: None,
            quantity: None,
            selected_part_quote_id: None,
            clear_part_quotes: Some(true),
        };
        self.parts_repository.update(updatable_part).await?;

        Ok(CreateDrawingUploadUrlResponse::new(url, presigned_url))
    }
}

impl CreateDrawingUploadUrl {
    async fn quotation_is_payed(&self, input: &CreateDrawingUploadUrlInput) -> Result<bool> {
        let quotation = self
            .quotation_repository
            .get(input.project_id.clone(), input.quotation_id.clone())
            .await?;

        Ok(quotation.status == QuoteStatus::Payed)
    }
}
