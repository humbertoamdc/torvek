use std::sync::Arc;
use std::time::Duration;

use api_boundary::common::error::Error;
use axum::async_trait;
use uuid::{ContextV7, Timestamp, Uuid};

use api_boundary::common::file::File;
use api_boundary::parts::models::Part;
use api_boundary::parts::requests::CreatePartsRequest;
use api_boundary::parts::responses::CreatePartsResponse;
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::UpdateQuotationStatusRequest;

use crate::quotations::usecases::update_quotation_status::UpdateQuotationStatusUseCase;
use crate::repositories::parts::PartsRepository;
use crate::services::object_storage::ObjectStorage;
use crate::shared::{Result, UseCase};

static PRESIGNED_URLS_PUT_DURATION_SECONDS: u64 = 300;
static ORIGINAL_FILES_BASE_FILE_PATH: &'static str = "parts/originals";
static RENDER_FILES_BASE_FILE_PATH: &'static str = "parts/web_ready";
static RENDER_FILE_FORMAT: &'static str = ".stl";

pub struct CreatePartsUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
    update_quotation_status_usecase: UpdateQuotationStatusUseCase,
}

impl CreatePartsUseCase {
    pub fn new(
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
        update_quotation_status_usecase: UpdateQuotationStatusUseCase,
    ) -> Self {
        Self {
            parts_repository,
            object_storage,
            update_quotation_status_usecase,
        }
    }
}

#[async_trait]
impl UseCase<CreatePartsRequest, CreatePartsResponse> for CreatePartsUseCase {
    async fn execute(&self, request: CreatePartsRequest) -> Result<CreatePartsResponse> {
        let file_ids = (0..request.file_names.len())
            .into_iter()
            .map(|_| {
                let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
                let encoded_id = format!("file_{}", bs58::encode(id).into_string());
                encoded_id
            })
            .collect::<Vec<String>>();
        let original_file_names = request.file_names.clone();
        let render_file_names = request
            .file_names
            .into_iter()
            .map(|file_name| {
                let file_name = file_name.rsplit_once(".").unwrap().0.to_string();

                file_name + RENDER_FILE_FORMAT
            })
            .collect::<Vec<String>>();

        let original_presigned_urls = self
            .generate_presigned_urls(
                &original_file_names,
                &file_ids,
                ORIGINAL_FILES_BASE_FILE_PATH,
                request.customer_id.clone(),
            )
            .await?;
        let render_presigned_urls = self
            .generate_presigned_urls(
                &render_file_names,
                &file_ids,
                RENDER_FILES_BASE_FILE_PATH,
                request.customer_id.clone(),
            )
            .await?;

        let parts = original_presigned_urls
            .iter()
            .zip(render_presigned_urls)
            .enumerate()
            .map(|(i, (original_presigned_url, render_presigned_url))| {
                let original_url = original_presigned_url
                    .split("?")
                    .nth(0)
                    .unwrap()
                    .to_string();
                let render_url = render_presigned_url.split("?").nth(0).unwrap().to_string();

                let original_file = File::new(original_file_names[i].clone(), original_url);
                let render_file = File::new(render_file_names[i].clone(), render_url);

                Part::new(
                    request.customer_id.clone(),
                    request.project_id.clone(),
                    request.quotation_id.clone(),
                    original_file,
                    render_file,
                )
            })
            .collect();

        let update_quotation_status_request = UpdateQuotationStatusRequest {
            project_id: request.project_id,
            quotation_id: request.quotation_id,
            status: QuotationStatus::Created,
        };

        self.update_quotation_status_usecase
            .execute(update_quotation_status_request)
            .await
            .map_err(|_| Error::UnknownError)?;

        self.parts_repository.create_parts(parts).await?;

        Ok(CreatePartsResponse::new(original_presigned_urls))
    }
}

impl CreatePartsUseCase {
    async fn generate_presigned_urls(
        &self,
        file_names: &Vec<String>,
        file_ids: &Vec<String>,
        file_path: &str,
        customer_id: String,
    ) -> Result<Vec<String>> {
        let file_extensions = file_names
            .iter()
            .map(|file_name| file_name.split(".").last().unwrap().to_string())
            .collect::<Vec<String>>();

        let mut urls: Vec<String> = Vec::with_capacity(file_extensions.len());
        for (i, file_extension) in file_extensions.into_iter().enumerate() {
            let file_path = format!(
                "{}/{}/{}.{}",
                file_path, customer_id, file_ids[i], file_extension
            );
            let presigned_url = self
                .object_storage
                .put_object_presigned_url(
                    &file_path,
                    Duration::from_secs(PRESIGNED_URLS_PUT_DURATION_SECONDS),
                )
                .await?;
            urls.push(presigned_url);
        }

        Ok(urls)
    }
}
