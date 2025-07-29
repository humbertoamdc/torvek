use std::env;
use std::sync::Arc;

use aws_config::{BehaviorVersion, SdkConfig};
use http::header::AUTHORIZATION;
use reqwest::header::ACCEPT;
use reqwest::header::{HeaderMap, HeaderValue};
use stripe::Client;

use crate::config::{Config, Environment};
use crate::repositories::orders_dynamodb::DynamodbOrders;
use crate::repositories::parts_dynamodb::DynamodbParts;
use crate::repositories::projects_dynamodb::DynamodbProjects;
use crate::repositories::quotes_dynamodb::DynamodbQuotes;
use crate::repositories::transaction_dynamodb::DynamodbTransaction;
use crate::services::identity_manager_ory::OryIdentityManager;
use crate::services::object_storage_s3::S3ObjectStorage;
use crate::services::stripe::Stripe;
use crate::services::stripe_client::StripeClient;

#[derive(Clone)]
pub struct AppState {
    pub env: Environment,
    pub domain: String,
    pub auth: AppStateAuth,
    pub orders: AppStateOrders,
    pub projects: AppStateProjects,
    pub quotes: AppStateQuotes,
    pub parts: AppStateParts,
    pub payments: AppStatePayments,
}

#[derive(Clone)]
pub struct AppStateAuth {
    pub ory_kratos: Arc<OryIdentityManager>,
}

#[derive(Clone)]
pub struct AppStateOrders {
    pub dynamodb_orders: Arc<DynamodbOrders>,
}

#[derive(Clone)]
pub struct AppStateProjects {
    pub dynamodb_projects: Arc<DynamodbProjects>,
}

#[derive(Clone)]
pub struct AppStateQuotes {
    pub dynamodb_quotes: Arc<DynamodbQuotes>,
}

#[derive(Clone)]
pub struct AppStateParts {
    pub dynamodb_parts: Arc<DynamodbParts>,
    pub s3: Arc<S3ObjectStorage>,
}

#[derive(Clone)]
pub struct AppStatePayments {
    pub webhook_secret: String,
    pub stripe_client: Arc<dyn StripeClient>,
    pub transaction: DynamodbTransaction,
}

impl AppState {
    pub async fn from(config: &Config) -> Self {
        Self {
            env: config.app.env.clone(),
            domain: config.app.domain.clone(),
            auth: AppStateAuth::from(config).await,
            orders: AppStateOrders::from(config).await,
            projects: AppStateProjects::from(config).await,
            quotes: AppStateQuotes::from(config).await,
            parts: AppStateParts::from(config).await,
            payments: AppStatePayments::from(config).await,
        }
    }
}

impl AppStateAuth {
    async fn from(config: &Config) -> Self {
        // Clients
        let reqwest_client = Self::reqwest_client(config);

        // Services & Repositories
        let identity_manager = Arc::new(OryIdentityManager::new(
            config.auth.ory_clients_url.clone(),
            reqwest_client.clone(),
            config.auth.ory_clients_api_key.clone(),
        ));

        Self {
            ory_kratos: identity_manager,
        }
    }

    fn reqwest_client(config: &Config) -> reqwest::Client {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        reqwest::Client::builder()
            .default_headers(headers)
            .https_only(config.app.env != Environment::Development)
            .build()
            .unwrap()
    }
}

impl AppStateOrders {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_config = get_shared_config(config).await;
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services & Repositories
        let orders_repository = Arc::new(DynamodbOrders::new(
            dynamodb_client.clone(),
            config.orders.orders_table.clone(),
        ));

        Self {
            dynamodb_orders: orders_repository,
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
            dynamodb_projects: projects_repository,
        }
    }
}

impl AppStateQuotes {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_config = get_shared_config(config).await;
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services & Repositories
        let quotes_repository = Arc::new(DynamodbQuotes::new(
            dynamodb_client,
            config.quotes.quotes_table.clone(),
        ));

        Self {
            dynamodb_quotes: quotes_repository,
        }
    }
}

impl AppStateParts {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_s3_config = get_s3_shared_config(config).await;
        let shared_config = get_shared_config(config).await;
        let s3_config = aws_sdk_s3::config::Builder::from(&shared_s3_config).build();
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        let s3_client = aws_sdk_s3::Client::from_conf(s3_config);
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services & Repositories
        let parts_repository = Arc::new(DynamodbParts::new(
            dynamodb_client.clone(),
            config.parts.parts_table.clone(),
        ));
        let object_storage = Arc::new(S3ObjectStorage::new(
            s3_client,
            config.parts.s3_bucket.clone(),
        ));

        Self {
            dynamodb_parts: parts_repository,
            s3: object_storage,
        }
    }
}

impl AppStatePayments {
    async fn from(config: &Config) -> Self {
        // Configs
        let shared_config = get_shared_config(config).await;
        let dynamodb_config = aws_sdk_dynamodb::config::Builder::from(&shared_config).build();

        // Clients
        let client = Client::new(&config.payments.secret_key);
        let mut headers_map = HeaderMap::new();
        headers_map.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.payments.secret_key)).unwrap(),
        );
        let files_client = reqwest::Client::builder()
            .default_headers(headers_map)
            .build()
            .expect("error while creating files client");
        let dynamodb_client = aws_sdk_dynamodb::Client::from_conf(dynamodb_config);

        // Services
        let stripe_client = Arc::new(Stripe::new(
            client,
            files_client,
            config.payments.success_url.clone(),
            config.payments.tax_ids.clone(),
        ));

        let transaction = DynamodbTransaction::new(dynamodb_client);

        Self {
            webhook_secret: config.payments.webhook_secret.clone(),
            stripe_client,
            transaction,
        }
    }
}

async fn get_shared_config(config: &Config) -> SdkConfig {
    let mut shared_config = aws_config::defaults(BehaviorVersion::latest());
    if config.app.env == Environment::Development {
        shared_config = shared_config.endpoint_url(env::var("AWS_ENDPOINT_URL").unwrap());
    }
    shared_config.load().await
}

async fn get_s3_shared_config(config: &Config) -> SdkConfig {
    let mut shared_config = aws_config::defaults(BehaviorVersion::latest());
    if config.app.env == Environment::Development {
        shared_config = shared_config.endpoint_url(env::var("AWS_S3_ENDPOINT_URL").unwrap());
    }
    shared_config.load().await
}
