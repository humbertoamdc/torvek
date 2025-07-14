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
use crate::shared::{Result, UseCase};

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
        let file_ids = (0..input.file_names.len())
            .map(|_| {
                let id = Uuid::new_v7(Timestamp::now(ContextV7::new()));
                let encoded_id = format!("file_{}", bs58::encode(id).into_string());
                encoded_id
            })
            .collect::<Vec<String>>();
        let original_file_names = input.file_names.clone();
        let render_file_names = input
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
                input.identity.id.clone(),
            )
            .await?;
        let render_presigned_urls = self
            .generate_presigned_urls(
                &render_file_names,
                &file_ids,
                RENDER_FILES_BASE_FILE_PATH,
                input.identity.id.clone(),
            )
            .await?;

        let mut parts: Vec<Part> = original_presigned_urls
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
                    input.identity.id.clone(),
                    input.project_id.clone(),
                    input.quotation_id.clone(),
                    PartProcess::CNC,
                    PartAttributes::CNC(CNCAttributes::default()),
                    original_file,
                    render_file,
                )
            })
            .collect();

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

        Ok(CreatePartsResponse::new(original_presigned_urls))
    }
}

impl<Q, P> CreateParts<Q, P>
where
    Q: QuotesRepository,
    P: PartsRepository,
{
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
