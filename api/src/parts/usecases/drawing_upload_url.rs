use std::sync::Arc;
use std::time::Duration;

use axum::async_trait;
use uuid::Uuid;

use api_boundary::parts::requests::CreateDrawingUploadUrlRequest;
use api_boundary::parts::responses::CreateDrawingUploadUrlResponse;

use crate::parts::domain::errors::PartsError;
use crate::parts::services::object_storage::ObjectStorage;
use crate::shared::usecase::UseCase;

pub struct CreateDrawingUploadUrlUseCase {
    object_storage: Arc<dyn ObjectStorage>,
}

impl CreateDrawingUploadUrlUseCase {
    pub const fn new(object_storage: Arc<dyn ObjectStorage>) -> Self {
        Self { object_storage }
    }
}

#[async_trait]
impl UseCase<CreateDrawingUploadUrlRequest, CreateDrawingUploadUrlResponse, PartsError>
    for CreateDrawingUploadUrlUseCase
{
    async fn execute(
        &self,
        request: CreateDrawingUploadUrlRequest,
    ) -> Result<CreateDrawingUploadUrlResponse, PartsError> {
        // We only want to preserve one drawing file per part. In the case where
        // a file has been previously uploaded, we will use the path for that file,
        // effectively overwriting the file and maintaining one drawing per part.
        let file_path = match request.file_url {
            Some(file_url) => {
                let file_url_without_query_parameters =
                    file_url.split("?").nth(0).unwrap().to_string();
                let file_path = file_url_without_query_parameters
                    .split("/")
                    .collect::<Vec<&str>>()
                    .iter()
                    .rev()
                    .take(2)
                    .rev()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join("/");

                file_path
            }
            None => {
                let file_id = Uuid::new_v4().to_string();
                let file_extension = request.file_name.split(".").last().unwrap();
                let file_path = format!("{}/{}.{}", request.client_id, file_id, file_extension);

                file_path
            }
        };

        let presigned_url = self
            .object_storage
            .put_object_presigned_url(file_path, Duration::from_secs(300))
            .await?;
        let url = presigned_url.split("?").nth(0).unwrap().to_string();
        Ok(CreateDrawingUploadUrlResponse::new(url, presigned_url))
    }
}
