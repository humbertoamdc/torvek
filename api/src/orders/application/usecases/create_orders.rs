use std::sync::Arc;
use std::time::Duration;

use axum::async_trait;
use uuid::Uuid;

use crate::orders::adapters::api::requests::CreateOrdersRequest;
use crate::orders::adapters::api::responses::CreateOrdersResponse;
use crate::orders::application::repositories::orders::OrdersRepository;
use crate::orders::application::services::object_storage::ObjectStorage;
use crate::orders::application::usecases::interfaces::UseCase;
use crate::orders::domain::errors::OrdersError;
use crate::orders::domain::order::Order;

pub struct CreateOrdersUseCase {
    object_storage: Arc<dyn ObjectStorage>,
    orders_repository: Arc<dyn OrdersRepository>,
}

impl CreateOrdersUseCase {
    pub const fn new(
        object_storage: Arc<dyn ObjectStorage>,
        orders_repository: Arc<dyn OrdersRepository>,
    ) -> Self {
        Self {
            object_storage,
            orders_repository,
        }
    }
}

#[async_trait]
impl UseCase<CreateOrdersRequest, Vec<CreateOrdersResponse>, OrdersError> for CreateOrdersUseCase {
    async fn execute(
        &self,
        request: CreateOrdersRequest,
    ) -> Result<Vec<CreateOrdersResponse>, OrdersError> {
        let responses = self.generate_presigned_urls(&request).await?;

        let orders = responses
            .iter()
            .enumerate()
            .map(|(i, res)| {
                Order::new(
                    request.client_id.clone(),
                    request.file_names[i].clone(),
                    res.upload_url.split("?").nth(0).unwrap().to_string(),
                )
            })
            .collect::<Vec<Order>>();
        self.orders_repository.create_orders(orders).await?;
        Ok(responses)
    }
}

impl CreateOrdersUseCase {
    async fn generate_presigned_urls(
        &self,
        request: &CreateOrdersRequest,
    ) -> Result<Vec<CreateOrdersResponse>, OrdersError> {
        let file_extensions = request
            .file_names
            .iter()
            .map(|file_name| file_name.split(".").last().unwrap().to_string())
            .collect::<Vec<String>>();

        let mut responses: Vec<CreateOrdersResponse> = Vec::with_capacity(file_extensions.len());
        for file_extension in file_extensions.into_iter() {
            let file_id = Uuid::new_v4().to_string();
            let file_path = format!("{}/{}.{}", request.client_id, file_id, file_extension);
            let presigned_url = self
                .object_storage
                .put_object_presigned_url(file_path, Duration::from_secs(300))
                .await?;
            responses.push(CreateOrdersResponse::new(file_id, presigned_url));
        }

        Ok(responses)
    }
}
