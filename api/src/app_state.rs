use aws_config::{BehaviorVersion, SdkConfig};
use std::env;
use std::sync::Arc;

use reqwest::header::ACCEPT;
use reqwest::header::{HeaderMap, HeaderValue};

use crate::auth::adapters::spi::admin_identity_manager::ory::OryAdminIdentityManager;
use crate::auth::adapters::spi::identity_manager::ory::OryIdentityManager;
use crate::auth::application::services::identity_manager::{AdminIdentityManager, IdentityManager};
use crate::config::{Config, Environment};
use crate::orders::adapters::spi::object_storage::s3::S3ObjectStorage;
use crate::orders::adapters::spi::orders_repository::dynamodb::DynamodbOrders;
use crate::orders::application::repositories::orders::OrdersRepository;
use crate::orders::application::services::object_storage::ObjectStorage;
use crate::parts;
use crate::parts::repositories::parts::PartsRepository;
use crate::parts::repositories::parts_dynamodb::DynamodbParts;
use crate::projects::repositories::projects::ProjectsRepository;
use crate::projects::repositories::projects_dynamodb::DynamodbProjects;
use crate::quotations::repositories::quotations::QuotationsRepository;
use crate::quotations::repositories::quotations_dynamodb::DynamodbQuotations;

#[derive(Clone)]
pub struct AppState {
    pub env: Environment,
    pub domain: String,
    pub auth: Auth,
    pub orders: Orders,
    pub projects: Projects,
    pub quotations: Quotations,
    pub parts: Parts,
}

#[derive(Clone)]
pub struct Auth {
    pub identity_manager: Arc<dyn IdentityManager>,
    pub admin_identity_manager: Arc<dyn AdminIdentityManager>,
}

#[derive(Clone)]
pub struct Orders {
    pub object_storage: Arc<dyn ObjectStorage>,
    pub orders_repository: Arc<dyn OrdersRepository>,
}

#[derive(Clone)]
pub struct Projects {
    pub projects_repository: Arc<dyn ProjectsRepository>,
}

#[derive(Clone)]
pub struct Quotations {
    pub quotations_repository: Arc<dyn QuotationsRepository>,
}

#[derive(Clone)]
pub struct Parts {
    pub parts_repository: Arc<dyn PartsRepository>,
    pub object_storage: Arc<dyn parts::services::object_storage::ObjectStorage>,
}

impl AppState {
    pub async fn from(config: &Config) -> Self {
        Self {
            env: config.app.env.clone(),
            domain: config.app.domain.clone(),
            auth: Auth::from(config).await,
            orders: Orders::from(config).await,
            projects: Projects::from(config).await,
            quotations: Quotations::from(config).await,
            parts: Parts::from(config).await,
        }
    }
}

impl Auth {
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

impl Orders {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_config = get_shared_config(config).await;
        let s3_config = aws_sdk_s3::config::Builder::from(&shared_config).build();
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        let s3_client = aws_sdk_s3::Client::from_conf(s3_config);
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services & Repositories
        let object_storage = Arc::new(S3ObjectStorage::new(
            s3_client,
            config.orders.s3_bucket.clone(),
        ));
        let orders_repository = Arc::new(DynamodbOrders::new(
            dynamodb_client,
            config.orders.orders_table.clone(),
        ));

        Self {
            object_storage,
            orders_repository,
        }
    }
}

impl Projects {
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

impl Quotations {
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

impl Parts {
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

async fn get_shared_config(config: &Config) -> SdkConfig {
    let mut shared_config = aws_config::defaults(BehaviorVersion::v2023_11_09());
    if config.app.env == Environment::Development {
        shared_config = shared_config.endpoint_url(env::var("AWS_ENDPOINT_URL").unwrap());
    }
    shared_config.load().await
}
