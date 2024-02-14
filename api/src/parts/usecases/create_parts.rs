use std::sync::Arc;
use std::time::Duration;

use axum::async_trait;
use uuid::Uuid;

use api_boundary::common::file::File;
use api_boundary::parts::models::Part;
use api_boundary::parts::requests::CreatePartsRequest;
use api_boundary::parts::responses::CreatePartsResponse;

use crate::parts::domain::errors::PartsError;
use crate::parts::repositories::parts::PartsRepository;
use crate::parts::services::object_storage::ObjectStorage;
use crate::shared::usecase::UseCase;

pub struct CreatePartsUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
}

impl CreatePartsUseCase {
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
impl UseCase<CreatePartsRequest, CreatePartsResponse, PartsError> for CreatePartsUseCase {
    async fn execute(
        &self,
        request: CreatePartsRequest,
    ) -> Result<CreatePartsResponse, PartsError> {
        let presigned_urls = self.generate_presigned_urls(&request).await?;

        let parts = presigned_urls
            .iter()
            .enumerate()
            .map(|(i, presigned_url)| {
                let url = presigned_url.split("?").nth(0).unwrap().to_string();
                let file = File::new(request.file_names[i].clone(), url);
                Part::new(
                    request.client_id.clone(),
                    request.project_id.clone(),
                    request.quotation_id.clone(),
                    file,
                )
            })
            .collect();
        self.parts_repository.create_parts(parts).await?;

        Ok(CreatePartsResponse::new(presigned_urls))
    }
}

impl CreatePartsUseCase {
    async fn generate_presigned_urls(
        &self,
        request: &CreatePartsRequest,
    ) -> Result<Vec<String>, PartsError> {
        let file_extensions = request
            .file_names
            .iter()
            .map(|file_name| file_name.split(".").last().unwrap().to_string())
            .collect::<Vec<String>>();

        let mut urls: Vec<String> = Vec::with_capacity(file_extensions.len());
        for file_extension in file_extensions.into_iter() {
            let file_id = Uuid::new_v4().to_string();
            let file_path = format!("{}/{}.{}", request.client_id, file_id, file_extension);
            let presigned_url = self
                .object_storage
                .put_object_presigned_url(file_path, Duration::from_secs(300))
                .await?;
            urls.push(presigned_url);
        }

        Ok(urls)
    }
}
