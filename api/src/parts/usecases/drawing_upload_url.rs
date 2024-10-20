use std::sync::Arc;
use std::time::Duration;

use api_boundary::common::error::Error;
use api_boundary::common::file::File;
use axum::async_trait;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::parts::domain::updatable_part::UpdatablePart;
use crate::parts::repositories::parts::PartsRepository;
use api_boundary::parts::requests::CreateDrawingUploadUrlRequest;
use api_boundary::parts::responses::CreateDrawingUploadUrlResponse;
use api_boundary::quotations::models::QuotationStatus;
use api_boundary::quotations::requests::GetQuotationByIdRequest;

use crate::parts::services::object_storage::ObjectStorage;
use crate::quotations::usecases::get_quotation_by_id::GetQuotationByIdUseCase;
use crate::shared::{Result, UseCase};

static DRAWING_FILES_BASE_FILE_PATH: &'static str = "parts/drawings";

pub struct CreateDrawingUploadUrlUseCase {
    parts_repository: Arc<dyn PartsRepository>,
    object_storage: Arc<dyn ObjectStorage>,
    get_quotation_by_id_use_case: GetQuotationByIdUseCase,
}

impl CreateDrawingUploadUrlUseCase {
    pub const fn new(
        parts_repository: Arc<dyn PartsRepository>,
        object_storage: Arc<dyn ObjectStorage>,
        get_quotation_by_id_use_case: GetQuotationByIdUseCase,
    ) -> Self {
        Self {
            parts_repository,
            object_storage,
            get_quotation_by_id_use_case,
        }
    }
}

#[async_trait]
impl UseCase<CreateDrawingUploadUrlRequest, CreateDrawingUploadUrlResponse>
    for CreateDrawingUploadUrlUseCase
{
    async fn execute(
        &self,
        request: CreateDrawingUploadUrlRequest,
    ) -> Result<CreateDrawingUploadUrlResponse> {
        if self.quotation_is_payed(&request).await? {
            return Err(Error::UpdatePartAfterPayingQuotation);
        }

        // We only want to preserve one drawing file per part. In the case where
        // a file has been previously uploaded, we will use the path for that file,
        // effectively overwriting the file and maintaining one drawing per part.
        let file_path = match request.file_url {
            Some(file_url) => file_url.path().strip_prefix("/").unwrap().to_string(),
            None => {
                let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));

                let file_id = format!("file_{}", bs58::encode(id).into_string());
                let file_extension = request.file_name.split(".").last().unwrap();
                let file_path = format!(
                    "{}/{}/{}.{}",
                    DRAWING_FILES_BASE_FILE_PATH, request.customer_id, file_id, file_extension
                );

                file_path
            }
        };

        let presigned_url = self
            .object_storage
            .put_object_presigned_url(file_path, Duration::from_secs(300))
            .await?;
        let url = presigned_url.split("?").nth(0).unwrap().to_string();

        let updatable_part = UpdatablePart {
            id: request.part_id,
            customer_id: request.customer_id,
            quotation_id: request.quotation_id,
            drawing_file: Some(File::new(request.file_name, url.clone())),
            process: None,
            material: None,
            tolerance: None,
            quantity: None,
            selected_part_quote_id: None,
        };
        self.parts_repository.update_part(updatable_part).await?;

        Ok(CreateDrawingUploadUrlResponse::new(url, presigned_url))
    }
}

impl CreateDrawingUploadUrlUseCase {
    async fn quotation_is_payed(&self, request: &CreateDrawingUploadUrlRequest) -> Result<bool> {
        let get_quotation_request = GetQuotationByIdRequest {
            customer_id: String::default(),
            project_id: request.project_id.clone(),
            quotation_id: request.quotation_id.clone(),
        };
        let quotation = self
            .get_quotation_by_id_use_case
            .execute(get_quotation_request)
            .await
            .map_err(|_| Error::UnknownError)?; // TODO: Handle error properly.

        Ok(quotation.status == QuotationStatus::Payed)
    }
}
