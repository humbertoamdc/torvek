use crate::parts::models::inputs::UpdatePartInput;
use crate::parts::models::part::{PartAttributes, PartProcess};
use crate::shared::file::File;
use crate::shared::{CustomerId, PartId, PartQuoteId};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UpdatablePart {
    pub id: PartId,
    pub customer_id: CustomerId,
    pub drawing_file: Option<File>,
    pub process: Option<PartProcess>,
    pub attributes: Option<PartAttributes>,
    pub quantity: Option<u64>,
    pub selected_part_quote_id: Option<PartQuoteId>,
    pub clear_part_quotes: Option<bool>,
}

impl UpdatablePart {
    pub fn partial_new(customer_id: CustomerId, part_id: PartId) -> Self {
        UpdatablePart {
            customer_id,
            id: part_id,
            ..Default::default()
        }
    }
}

impl From<&UpdatePartInput> for UpdatablePart {
    fn from(input: &UpdatePartInput) -> Self {
        Self {
            id: input.part_id.clone(),
            customer_id: input.identity.id.clone(),
            drawing_file: input.drawing_file.clone(),
            process: input.process.clone(),
            attributes: input.attributes.clone(),
            quantity: input.quantity,
            selected_part_quote_id: None,
            clear_part_quotes: None,
        }
    }
}

pub struct BatchDeletePartObject {
    pub customer_id: CustomerId,
    pub part_id: PartId,
}
