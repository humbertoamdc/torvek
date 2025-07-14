use crate::shared::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Transaction: Send + Sync + 'static {
    type TransactionItem;
    fn add_item(&mut self, item: Self::TransactionItem);
    fn add_items(&mut self, items: Vec<Self::TransactionItem>);
    async fn execute(&mut self) -> Result<()>;
}
