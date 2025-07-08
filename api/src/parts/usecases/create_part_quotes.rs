use crate::parts::models::inputs::CreatePartQuotesInput;
use crate::parts::models::part::PartQuote;
use crate::parts::services::part_quotes_creation::PartQuotesCreation;
use crate::shared::{Result, UseCase};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct CreatePartQuotes {
    part_quotes_creation_service: Arc<dyn PartQuotesCreation>,
}

impl CreatePartQuotes {
    pub fn new(part_quotes_creation_service: Arc<dyn PartQuotesCreation>) -> Self {
        Self {
            part_quotes_creation_service,
        }
    }
}

#[async_trait]
impl UseCase<CreatePartQuotesInput, ()> for CreatePartQuotes {
    async fn execute(&self, request: CreatePartQuotesInput) -> Result<()> {
        let mut part_quotes_by_part: HashMap<String, Vec<PartQuote>> = HashMap::new();
        let mut selected_part_quote_by_part: HashMap<String, String> = HashMap::new();
        let mut part_ids_set = HashSet::new();

        request.data.into_iter().for_each(|quote_data| {
            // Default selected to the first part quote. We might want to revisit this decision
            // and select by price or by deadline.
            let selected = !part_ids_set.contains(&quote_data.part_id);
            part_ids_set.insert(quote_data.part_id.clone());

            let part_quote = PartQuote::new(
                quote_data.part_id.clone(),
                quote_data.unit_price,
                quote_data.sub_total,
                quote_data.workdays_to_complete,
            );

            if selected {
                selected_part_quote_by_part
                    .insert(part_quote.part_id.clone(), part_quote.id.clone());
            }

            part_quotes_by_part
                .entry(quote_data.part_id.clone())
                .or_default()
                .push(part_quote);
        });

        self.part_quotes_creation_service
            .create_part_quotes_and_update_quotation(
                request.project_id,
                request.quotation_id,
                part_quotes_by_part,
                selected_part_quote_by_part,
            )
            .await
    }
}
