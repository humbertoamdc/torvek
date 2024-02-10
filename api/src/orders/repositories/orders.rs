use axum::async_trait;

#[async_trait]
pub trait OrdersRepository: Send + Sync + 'static {}
