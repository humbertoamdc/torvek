use crate::parts::models::inputs::CreatePartQuotesInput;
use crate::parts::models::part::PartQuote;
use crate::quotations::models::quotation::QuoteStatus;
use crate::repositories::parts::PartsRepository;
use crate::repositories::quotes::QuotesRepository;
use crate::repositories::transaction::Transaction;
use crate::shared::{PartId, PartQuoteId, Result, UseCase};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CreatePartQuotes<P, Q, Tx, TxItem>
where
    P: PartsRepository<TransactionItem = TxItem>,
    Q: QuotesRepository<TransactionItem = TxItem>,
    Tx: Transaction<TransactionItem = TxItem>,
{
    parts_repository: Arc<P>,
    quotes_repository: Arc<Q>,
    transaction: Arc<Mutex<Tx>>,
}

impl<P, Q, Tx, TxItem> CreatePartQuotes<P, Q, Tx, TxItem>
where
    P: PartsRepository<TransactionItem = TxItem>,
    Q: QuotesRepository<TransactionItem = TxItem>,
    Tx: Transaction<TransactionItem = TxItem>,
{
    pub fn new(
        parts_repository: Arc<P>,
        quotes_repository: Arc<Q>,
        transaction: Arc<Mutex<Tx>>,
    ) -> Self {
        Self {
            parts_repository,
            quotes_repository,
            transaction,
        }
    }
}

#[async_trait]
impl<P, Q, Tx, TxItem> UseCase<CreatePartQuotesInput, ()> for CreatePartQuotes<P, Q, Tx, TxItem>
where
    P: PartsRepository<TransactionItem = TxItem>,
    Q: QuotesRepository<TransactionItem = TxItem>,
    Tx: Transaction<TransactionItem = TxItem>,
    TxItem: Send,
{
    async fn execute(&self, input: CreatePartQuotesInput) -> Result<()> {
        let mut part_quotes_by_part: HashMap<PartId, Vec<PartQuote>> = HashMap::new();
        let mut selected_part_quote_by_part: HashMap<PartId, PartQuoteId> = HashMap::new();
        let mut part_ids_set = HashSet::new();

        input.data.into_iter().for_each(|quote_data| {
            // Default selected to the first part quote. We might want to revisit this decision
            // and select by price or by deadline.
            let selected = !part_ids_set.contains(&quote_data.part_id);
            part_ids_set.insert(quote_data.part_id.clone());

            let part_quote = PartQuote::new(
                quote_data.unit_price,
                quote_data.sub_total,
                quote_data.workdays_to_complete,
            );

            if selected {
                selected_part_quote_by_part
                    .insert(quote_data.part_id.clone(), part_quote.id.clone());
            }

            part_quotes_by_part
                .entry(quote_data.part_id.clone())
                .or_default()
                .push(part_quote);
        });

        let quote_transaction = self.quotes_repository.transaction_update(
            input.customer_id.clone(),
            input.project_id,
            input.quotation_id,
            QuoteStatus::PendingReview,
            QuoteStatus::PendingPayment,
        );

        let part_quotes_transactions = part_ids_set
            .into_iter()
            .map(|part_id| {
                self.parts_repository.transaction_create_part_quotes(
                    input.customer_id.clone(),
                    part_id.clone(),
                    selected_part_quote_by_part[&part_id].clone(),
                    part_quotes_by_part[&part_id].clone(),
                )
            })
            .collect();

        {
            let mut transaction = self.transaction.lock().await;
            transaction.add_item(quote_transaction);
            transaction.add_items(part_quotes_transactions);
            transaction.execute().await?;
        }

        Ok(())
    }
}
