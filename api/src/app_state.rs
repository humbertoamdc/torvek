use std::env;
use std::sync::Arc;

use http::header::ACCEPT;
use http::HeaderMap;

use crate::auth::adapters::spi::admin_identity_manager::ory::OryAdminIdentityManager;
use crate::auth::adapters::spi::identity_manager::ory::OryIdentityManager;
use crate::auth::application::services::identity_manager::{AdminIdentityManager, IdentityManager};
use crate::config::{Config, Environment};
use crate::orders::adapters::spi::object_storage::s3::S3ObjectStorage;
use crate::orders::adapters::spi::orders_repository::dynamodb::DynamodbOrders;
use crate::orders::application::repositories::orders::OrdersRepository;
use crate::orders::application::services::object_storage::ObjectStorage;
use crate::projects::repositories::projects::ProjectsRepository;
use crate::projects::repositories::projects_dynamodb::DynamodbProjects;

#[derive(Clone)]
pub struct AppState {
    pub env: Environment,
    pub domain: String,
    pub auth: Auth,
    pub orders: Orders,
    pub projects: Projects,
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

impl AppState {
    pub async fn from(config: &Config) -> Self {
        Self {
            env: config.app.env.clone(),
            domain: config.app.domain.clone(),
            auth: Auth::from(config).await,
            orders: Orders::from(config).await,
            projects: Projects::from(config).await,
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
        headers.insert(ACCEPT, "application/json".parse().unwrap());
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
        let mut shared_config = aws_config::from_env();
        if config.app.env == Environment::Development {
            shared_config = shared_config.endpoint_url(env::var("AWS_ENDPOINT_URL").unwrap());
        }
        let shared_config = shared_config.load().await;

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
        let mut shared_config = aws_config::from_env();
        if config.app.env == Environment::Development {
            shared_config = shared_config.endpoint_url(env::var("AWS_ENDPOINT_URL").unwrap());
        }
        let shared_config = shared_config.load().await;

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
