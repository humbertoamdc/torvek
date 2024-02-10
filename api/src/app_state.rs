use std::env;
use std::sync::Arc;

use aws_config::{BehaviorVersion, SdkConfig};
use reqwest::header::ACCEPT;
use reqwest::header::{HeaderMap, HeaderValue};
use stripe::Client;

use crate::auth::adapters::spi::admin_identity_manager::ory::OryAdminIdentityManager;
use crate::auth::adapters::spi::identity_manager::ory::OryIdentityManager;
use crate::auth::application::services::identity_manager::{AdminIdentityManager, IdentityManager};
use crate::config::{Config, Environment};
use crate::orders::repositories::orders::OrdersRepository;
use crate::orders::repositories::orders_dynamodb::DynamodbOrders;
use crate::orders::services::orders_creation::OrdersCreationService;
use crate::orders::services::orders_creation_dynamodb::DynamodbOrdersCreationService;
use crate::parts;
use crate::parts::repositories::parts::PartsRepository;
use crate::parts::repositories::parts_dynamodb::DynamodbParts;
use crate::payments::services::stripe::StripePaymentsProcessor;
use crate::projects::repositories::projects::ProjectsRepository;
use crate::projects::repositories::projects_dynamodb::DynamodbProjects;
use crate::quotations::repositories::quotations::QuotationsRepository;
use crate::quotations::repositories::quotations_dynamodb::DynamodbQuotations;

#[derive(Clone)]
pub struct AppState {
    pub env: Environment,
    pub domain: String,
    pub auth: AppStateAuth,
    pub orders: AppStateOrders,
    pub projects: AppStateProjects,
    pub quotations: AppStateQuotations,
    pub parts: AppStateParts,
    pub payments: AppStatePayments,
}

#[derive(Clone)]
pub struct AppStateAuth {
    pub identity_manager: Arc<dyn IdentityManager>,
    pub admin_identity_manager: Arc<dyn AdminIdentityManager>,
}

#[derive(Clone)]
pub struct AppStateOrders {
    pub orders_repository: Arc<dyn OrdersRepository>,
    pub orders_creation_service: Arc<dyn OrdersCreationService>,
}

#[derive(Clone)]
pub struct AppStateProjects {
    pub projects_repository: Arc<dyn ProjectsRepository>,
}

#[derive(Clone)]
pub struct AppStateQuotations {
    pub quotations_repository: Arc<dyn QuotationsRepository>,
}

#[derive(Clone)]
pub struct AppStateParts {
    pub parts_repository: Arc<dyn PartsRepository>,
    pub object_storage: Arc<dyn parts::services::object_storage::ObjectStorage>,
}

#[derive(Clone)]
pub struct AppStatePayments {
    pub webhook_secret: String,
    pub payments_processor: StripePaymentsProcessor,
}

impl AppState {
    pub async fn from(config: &Config) -> Self {
        Self {
            env: config.app.env.clone(),
            domain: config.app.domain.clone(),
            auth: AppStateAuth::from(config).await,
            orders: AppStateOrders::from(config).await,
            projects: AppStateProjects::from(config).await,
            quotations: AppStateQuotations::from(config).await,
            parts: AppStateParts::from(config).await,
            payments: AppStatePayments::from(config),
        }
    }
}

impl AppStateAuth {
    async fn from(config: &Config) -> Self {
        // Clients
        let reqwest_client = Self::reqwest_client();

        // Services & Repositories
        let identity_manager = Arc::new(OryIdentityManager::new(
            config.auth.ory_clients_url.clone(),
            reqwest_client.clone(),
            config.auth.ory_clients_api_key.clone(),
        ));

        let admin_identity_manager = Arc::new(OryAdminIdentityManager::new(
            config.auth.ory_admins_url.clone(),
            reqwest_client,
        ));

        Self {
            identity_manager,
            admin_identity_manager,
        }
    }

    fn reqwest_client() -> reqwest::Client {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        reqwest::Client::builder()
            .default_headers(headers)
            .https_only(true)
            .build()
            .unwrap()
    }
}

impl AppStateOrders {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_config = get_shared_config(config).await;
        // let s3_config = aws_sdk_s3::config::Builder::from(&shared_config).build();
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        // let s3_client = aws_sdk_s3::Client::from_conf(s3_config);
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services & Repositories
        // let object_storage = Arc::new(S3ObjectStorage::new(
        //     s3_client,
        //     config.orders.s3_bucket.clone(),
        // ));
        let orders_repository = Arc::new(DynamodbOrders::new(
            dynamodb_client.clone(),
            config.orders.orders_table.clone(),
        ));
        let orders_creation_service = Arc::new(DynamodbOrdersCreationService::new(
            dynamodb_client,
            config.orders.orders_table.clone(),
            config.quotations.quotations_table.clone(),
        ));

        Self {
            orders_repository,
            orders_creation_service,
        }
    }
}

impl AppStateProjects {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_config = get_shared_config(config).await;
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services & Repositories
        let projects_repository = Arc::new(DynamodbProjects::new(
            dynamodb_client,
            config.projects.projects_table.clone(),
        ));
        Self {
            projects_repository,
        }
    }
}

impl AppStateQuotations {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_config = get_shared_config(config).await;
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services & Repositories
        let quotations_repository = Arc::new(DynamodbQuotations::new(
            dynamodb_client,
            config.quotations.quotations_table.clone(),
        ));

        Self {
            quotations_repository,
        }
    }
}

impl AppStateParts {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_config = get_shared_config(config).await;
        let s3_config = aws_sdk_s3::config::Builder::from(&shared_config).build();
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        let s3_client = aws_sdk_s3::Client::from_conf(s3_config);
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services & Repositories
        let parts_repository = Arc::new(DynamodbParts::new(
            dynamodb_client,
            config.parts.parts_table.clone(),
        ));
        let object_storage = Arc::new(parts::services::object_storage_s3::S3ObjectStorage::new(
            s3_client,
            config.parts.s3_bucket.clone(),
        ));

        Self {
            parts_repository,
            object_storage,
        }
    }
}

impl AppStatePayments {
    fn from(config: &Config) -> Self {
        // Clients
        let client = Client::new(&config.payments.secret_key);

        // Services
        let payments_processor =
            StripePaymentsProcessor::new(client, config.payments.success_url.clone());

        Self {
            webhook_secret: config.payments.webhook_secret.clone(),
            payments_processor,
        }
    }
}

async fn get_shared_config(config: &Config) -> SdkConfig {
    let mut shared_config = aws_config::defaults(BehaviorVersion::v2023_11_09());
    if config.app.env == Environment::Development {
        shared_config = shared_config.endpoint_url(env::var("AWS_ENDPOINT_URL").unwrap());
    }
    shared_config.load().await
}
