use crate::repositories::transaction::Transaction;
use crate::shared::error::Error;
use async_trait::async_trait;
use aws_sdk_dynamodb::types::TransactWriteItem;

#[derive(Clone)]
pub struct DynamodbTransaction {
    client: aws_sdk_dynamodb::Client,
    items: Vec<TransactWriteItem>,
}

impl DynamodbTransaction {
    pub fn new(client: aws_sdk_dynamodb::Client) -> DynamodbTransaction {
        Self {
            client,
            items: Vec::new(),
        }
    }
}

#[async_trait]
impl Transaction for DynamodbTransaction {
    type TransactionItem = TransactWriteItem;

    fn add_item(&mut self, item: TransactWriteItem) {
        self.items.push(item);
    }
    fn add_items(&mut self, items: Vec<TransactWriteItem>) {
        self.items.extend(items);
    }

    async fn execute(&mut self) -> crate::shared::Result<()> {
        let mut transaction_request = self.client.transact_write_items();

        while let Some(item) = self.items.pop() {
            transaction_request = transaction_request.transact_items(item);
        }

        let response = transaction_request.send().await;

        match response {
            Ok(_) => Ok(()),
            Err(err) => {
                tracing::error!("{err:?}");
                Err(Error::UnknownError)
            }
        }
    }
}
