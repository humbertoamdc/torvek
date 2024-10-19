use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use url::Url;

use crate::common::file::File;
use crate::common::money::Money;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsRequest {
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub file_names: Vec<String>,
}

impl CreatePartsRequest {
    pub const fn new(
        customer_id: String,
        project_id: String,
        quotation_id: String,
        file_names: Vec<String>,
    ) -> Self {
        Self {
            customer_id,
            project_id,
            quotation_id,
            file_names,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPartRequest {
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryPartsForQuotationRequest {
    pub quotation_id: String,
    pub with_quotation_subtotal: bool,
}

#[derive(Deserialize)]
pub struct QueryPartsForQuotationQueryParameters {
    pub with_quotation_subtotal: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePartRequest {
    pub customer_id: String, // TODO: Remove
    pub project_id: String,  // TODO: Remove
    pub quotation_id: String,
    pub part_id: String,
    pub drawing_file: Option<File>,
    pub process: Option<String>,
    pub material: Option<String>,
    pub tolerance: Option<String>,
    pub quantity: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlRequest {
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
    pub file_name: String,
    pub file_url: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateModelUploadUrlRequest {
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminUpdatePartRequest {
    pub id: String,
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
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
    pub deadline: NaiveDate,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSelectedPartQuoteRequest {
    pub quotation_id: String,
    pub part_id: String,
    pub selected_part_quote_id: String,
}
