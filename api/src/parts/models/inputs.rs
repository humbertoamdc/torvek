use crate::auth::models::session::Identity;
use crate::parts::models::part::PartAttributes;
use crate::shared::file::File;
use crate::shared::money::Money;
use serde_derive::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
    pub file_names: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPartInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryPartsForQuotationInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
    pub with_quotation_subtotal: bool,
    pub cursor: Option<String>,
    pub limit: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryPartsForQuotationInput {
    pub quotation_id: String,
    pub with_quotation_subtotal: bool,
    pub cursor: Option<String>,
    pub limit: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePartInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
    pub drawing_file: Option<File>,
    pub process: Option<String>,
    pub attributes: Option<PartAttributes>,
    pub quantity: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
    pub file_name: String,
    pub file_url: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateModelUploadUrlInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartQuotesRequest {
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub data: Vec<CreatePartQuotesRequestData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartQuotesRequestData {
    pub part_id: String,
    pub unit_price: Money,
    pub sub_total: Money,
    pub workdays_to_complete: u64,
    pub quantity: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSelectedPartQuoteInput {
    pub identity: Identity,
    pub quotation_id: String,
    pub part_id: String,
    pub selected_part_quote_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeletePartInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
}
