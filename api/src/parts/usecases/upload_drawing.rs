use crate::auth::models::session::Identity;
use crate::parts::models::dynamodb_requests::UpdatablePart;
use crate::parts::models::responses::UploadDrawingResponse;
use crate::repositories::parts::PartsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::file::File;
use crate::shared::{CustomerId, FileId, PartId, UseCase};
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use uuid::{ContextV7, Timestamp, Uuid};

static PRESIGNED_URLS_PUT_DURATION_SECONDS: u64 = 300;
static DRAWING_FILES_BASE_FILE_PATH: &str = "parts/drawings";

pub struct UploadDrawingInput {
    pub customer: Identity,
    pub part_id: PartId,
    pub file_name: String,
}

pub struct UploadDrawing<P>
where
    P: PartsRepository,
{
    parts_repository: Arc<P>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl<P> UploadDrawing<P>
where
    P: PartsRepository,
{
    pub fn new(parts_repository: Arc<P>, object_storage: Arc<dyn ObjectStorage>) -> Self {
        Self {
            parts_repository,
            object_storage,
        }
    }
}

#[async_trait]
impl<P> UseCase<UploadDrawingInput, UploadDrawingResponse> for UploadDrawing<P>
where
    P: PartsRepository,
{
    async fn execute(
        &self,
        input: UploadDrawingInput,
    ) -> crate::shared::Result<UploadDrawingResponse> {
        let part = self
            .parts_repository
            .get(input.customer.id.clone(), input.part_id.clone())
            .await?;

        let file_key = {
            if let Some(drawing_file) = part.drawing_file.clone() {
                drawing_file.key
            } else {
                let file_id = Self::generate_file_id();
                let file_extension = input.file_name.split(".").last().unwrap().to_string();

                self.file_key(
                    DRAWING_FILES_BASE_FILE_PATH,
                    input.customer.id.clone(),
                    input.part_id.clone(),
                    file_id,
                    file_extension,
                )
            }
        };

        let drawing_file = File::new(input.file_name, file_key.clone());

        let updatable_part = UpdatablePart {
            id: input.part_id,
            customer_id: input.customer.id,
            drawing_file: Some(drawing_file.clone()),
            ..Default::default()
        };

        self.parts_repository.update(updatable_part).await?;

        let upload_url = self
            .object_storage
            .put_object_presigned_url(
                &file_key,
                Duration::from_secs(PRESIGNED_URLS_PUT_DURATION_SECONDS),
            )
            .await?;

        Ok(UploadDrawingResponse {
            upload_url,
            file: drawing_file,
        })
    }
}

impl<P> UploadDrawing<P>
where
    P: PartsRepository,
{
    fn file_key(
        &self,
        file_path: &str,
        customer_id: CustomerId,
        part_id: PartId,
        file_id: FileId,
        file_extension: String,
    ) -> String {
        format!("{file_path}/{customer_id}/{part_id}/{file_id}.{file_extension}").to_string()
    }

    fn generate_file_id() -> String {
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("file_{}", bs58::encode(id).into_string());
        encoded_id
    }
}
