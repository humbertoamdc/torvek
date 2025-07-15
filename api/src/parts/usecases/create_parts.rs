use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use uuid::{ContextV7, Timestamp, Uuid};

use crate::parts::models::inputs::CreatePartsInput;
use crate::parts::models::part::{CNCAttributes, Part, PartAttributes, PartProcess};
use crate::parts::models::responses::CreatePartsResponse;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::services::object_storage::ObjectStorage;
use crate::services::stripe_client::StripeClient;
use crate::shared::file::File;
use crate::shared::{CustomerId, FileId, PartId, Result, UseCase};

static PRESIGNED_URLS_PUT_DURATION_SECONDS: u64 = 300;
static ORIGINAL_FILES_BASE_FILE_PATH: &str = "parts/originals";
static RENDER_FILES_BASE_FILE_PATH: &str = "parts/web_ready";
static RENDER_FILE_FORMAT: &str = ".stl";

pub struct CreateParts<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    parts_repository: Arc<P>,
    quotations_repository: Arc<Q>,
    object_storage: Arc<dyn ObjectStorage>,
    stripe_client: Arc<dyn StripeClient>,
}

impl<Q, P> CreateParts<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    pub fn new(
        parts_repository: Arc<P>,
        quotations_repository: Arc<Q>,
        object_storage: Arc<dyn ObjectStorage>,
        stripe_client: Arc<dyn StripeClient>,
    ) -> Self {
        Self {
            parts_repository,
            quotations_repository,
            object_storage,
            stripe_client,
        }
    }
}

#[async_trait]
impl<Q, P> UseCase<CreatePartsInput, CreatePartsResponse> for CreateParts<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    async fn execute(&self, input: CreatePartsInput) -> Result<CreatePartsResponse> {
        let mut parts = Vec::with_capacity(input.file_names.len());
        let mut presigned_urls = Vec::with_capacity(input.file_names.len());

        for file_name in input.file_names {
            let mut part = Part::new(
                input.identity.id.clone(),
                input.project_id.clone(),
                input.quotation_id.clone(),
                PartProcess::CNC,
                PartAttributes::CNC(CNCAttributes::default()),
                File::default(), // Original
                File::default(), // Render
            );

            let file_id = Self::generate_file_id();
            let file_extension = file_name.split(".").last().unwrap().to_string();
            let file_path = self.file_path(
                ORIGINAL_FILES_BASE_FILE_PATH,
                input.identity.id.clone(),
                part.id.clone(),
                file_id,
                file_extension,
            );
            let file_url = format!("{}/{}", self.object_storage.endpoint_url(), file_path);

            part.model_file.name = file_name;
            part.model_file.url = file_url.clone();

            let presigned_url = self
                .object_storage
                .put_object_presigned_url(
                    &file_path,
                    Duration::from_secs(PRESIGNED_URLS_PUT_DURATION_SECONDS),
                )
                .await?;

            parts.push(part);
            presigned_urls.push(presigned_url);
        }

        for part in parts.iter_mut() {
            // Create stripe product.
            self.stripe_client
                .create_product(part.model_file.name.clone(), part.id.clone())
                .await?;
        }

        self.quotations_repository
            .update(
                input.identity.id,
                input.project_id,
                input.quotation_id,
                Some(QuoteStatus::Created),
            )
            .await?;

        self.parts_repository.batch_create(parts).await?;

        Ok(CreatePartsResponse::new(presigned_urls))
    }
}

impl<Q, P> CreateParts<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
    fn file_path(
        &self,
        file_path: &str,
        customer_id: CustomerId,
        part_id: PartId,
        file_id: FileId,
        file_extension: String,
    ) -> String {
        format!(
            "{}/{}/{}/{}.{}",
            file_path, customer_id, part_id, file_id, file_extension
        )
        .to_string()
    }

    fn generate_file_id() -> String {
        let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
        let encoded_id = format!("file_{}", bs58::encode(id).into_string());
        encoded_id
    }
}
