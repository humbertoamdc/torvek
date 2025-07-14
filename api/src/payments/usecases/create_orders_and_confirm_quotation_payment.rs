use crate::orders::models::order::{Order, OrderStatus};
use crate::parts::models::part::PartQuote;
use crate::payments::models::inputs::CompleteCheckoutSessionWebhookRequest;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::orders::OrdersRepository;
use crate::repositories::parts::PartsRepository;
use crate::repositories::projects::ProjectsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::repositories::transaction::Transaction;
use crate::shared::{Result, UseCase};
use crate::utils::workdays::Workdays;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CreateOrdersAndConfirmQuotationPayment<Pro, Quo, Ord, Par, Tx, TxItem>
where
    Pro: ProjectsRepository<TransactionItem = TxItem>,
    Quo: QuotesRepository<TransactionItem = TxItem>,
    Ord: OrdersRepository<TransactionItem = TxItem>,
    Par: PartsRepository<TransactionItem = TxItem>,
    Tx: Transaction<TransactionItem = TxItem>,
{
    projects_repository: Arc<Pro>,
    quotes_repository: Arc<Quo>,
    orders_repository: Arc<Ord>,
    parts_repository: Arc<Par>,
    transaction: Arc<Mutex<Tx>>,
}

impl<Pro, Quo, Ord, Par, Tx, TxItem>
    CreateOrdersAndConfirmQuotationPayment<Pro, Quo, Ord, Par, Tx, TxItem>
where
    Pro: ProjectsRepository<TransactionItem = TxItem>,
    Quo: QuotesRepository<TransactionItem = TxItem>,
    Ord: OrdersRepository<TransactionItem = TxItem>,
    Par: PartsRepository<TransactionItem = TxItem>,
    Tx: Transaction<TransactionItem = TxItem>,
{
    pub fn new(
        projects_repository: Arc<Pro>,
        quotes_repository: Arc<Quo>,
        orders_repository: Arc<Ord>,
        parts_repository: Arc<Par>,
        transaction: Arc<Mutex<Tx>>,
    ) -> Self {
        Self {
            projects_repository,
            quotes_repository,
            orders_repository,
            parts_repository,
            transaction,
        }
    }
}

#[async_trait]
impl<Pro, Quo, Ord, Par, Tx, TxItem> UseCase<CompleteCheckoutSessionWebhookRequest, ()>
    for CreateOrdersAndConfirmQuotationPayment<Pro, Quo, Ord, Par, Tx, TxItem>
where
    Pro: ProjectsRepository<TransactionItem = TxItem>,
    Quo: QuotesRepository<TransactionItem = TxItem>,
    Ord: OrdersRepository<TransactionItem = TxItem>,
    Par: PartsRepository<TransactionItem = TxItem>,
    Tx: Transaction<TransactionItem = TxItem>,
    TxItem: Send,
{
    async fn execute(&self, request: CompleteCheckoutSessionWebhookRequest) -> Result<()> {
        let query_parts_for_quotation_response = self
            .parts_repository
            .query(
                request.customer_id.clone(),
                request.quotation_id.clone(),
                None,
                100,
            )
            .await?;

        let selected_part_quote_for_part = query_parts_for_quotation_response
            .data
            .iter()
            .map(|part| {
                (
                    part.id.clone(),
                    // TODO: Safely unwrap.
                    part.part_quotes
                        .clone()
                        .expect("expecting part quotes")
                        .into_iter()
                        .find(|part_quote| {
                            part_quote.id == part.selected_part_quote_id.clone().unwrap()
                        })
                        .unwrap(),
                )
            })
            .collect::<HashMap<String, PartQuote>>();

        let orders: Vec<Order> = query_parts_for_quotation_response
            .data
            .into_iter()
            .map(|part| {
                let part_quote = selected_part_quote_for_part[&part.id].clone();
                let now = Utc::now().naive_utc().date();
                let deadline = Workdays::add_workdays(now, part_quote.workdays_to_complete);
                Order::new(
                    part.customer_id,
                    part.project_id,
                    part.quotation_id,
                    part.id.clone(),
                    part_quote.id,
                    deadline,
                    OrderStatus::Open,
                    request.shipping_recipient_name.clone(),
                    request.shipping_address.clone(),
                )
            })
            .collect();

        let project_transaction = self
            .projects_repository
            .transaction_update(request.customer_id.clone(), request.project_id.clone());
        let quote_transaction = self.quotes_repository.transaction_update(
            request.customer_id.clone(),
            request.project_id.clone(),
            request.quotation_id.clone(),
            QuoteStatus::PendingPayment,
            QuoteStatus::Payed,
        );
        let orders_transactions: Vec<_> = orders
            .into_iter()
            .map(|order| self.orders_repository.transaction_create(order))
            .collect();
        {
            let mut transaction = self.transaction.lock().await;
            transaction.add_item(project_transaction);
            transaction.add_item(quote_transaction);
            transaction.add_items(orders_transactions);
            transaction.execute().await?;
        }

        Ok(())
    }
}
