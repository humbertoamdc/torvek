use crate::parts::models::inputs::UpdatePartInput;
use crate::parts::models::part::PartAttributes;
use crate::shared::file::File;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UpdatablePart {
    pub id: String,
    pub customer_id: String,
    pub quotation_id: String,
    pub drawing_file: Option<File>,
    pub process: Option<String>,
    pub attributes: Option<PartAttributes>,
    pub quantity: Option<u64>,
    pub selected_part_quote_id: Option<String>,
    pub clear_part_quotes: Option<bool>,
}

impl UpdatablePart {
    pub fn partial_new(quotation_id: String, part_id: String) -> Self {
        let mut updatable_part = Self::default();
        updatable_part.id = part_id;
        updatable_part.quotation_id = quotation_id;
        updatable_part
    }
}

impl From<&UpdatePartInput> for UpdatablePart {
    fn from(input: &UpdatePartInput) -> Self {
        Self {
            id: input.part_id.clone(),
            customer_id: input.identity.id.clone(),
            quotation_id: input.quotation_id.clone(),
            drawing_file: input.drawing_file.clone(),
            process: input.process.clone(),
            attributes: input.attributes.clone(),
            quantity: input.quantity,
            selected_part_quote_id: None,
            clear_part_quotes: Some(true),
        }
    }
}

pub struct BatchDeletePartObject {
    pub part_id: String,
    pub quotation_id: String,
}
