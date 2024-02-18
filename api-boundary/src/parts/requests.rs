use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

use crate::common::file::File;
use crate::common::money::Money;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub file_names: Vec<String>,
}
impl CreatePartsRequest {
    pub const fn new(
        client_id: String,
        project_id: String,
        quotation_id: String,
        file_names: Vec<String>,
    ) -> Self {
        Self {
            client_id,
            project_id,
            quotation_id,
            file_names,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryPartsForQuotationRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
}
impl QueryPartsForQuotationRequest {
    pub const fn new(client_id: String, project_id: String, quotation_id: String) -> Self {
        Self {
            client_id,
            project_id,
            quotation_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePartRequest {
    pub id: String,
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub drawing_file: Option<File>,
    pub process: Option<String>,
    pub material: Option<String>,
    pub tolerance: Option<String>,
    pub quantity: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlRequest {
    pub client_id: String,
    pub file_name: String,
    pub file_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminUpdatePartRequest {
    pub id: String,
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub unit_price: u64,
    pub sub_total: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartPriceOptionsAndUpdateQuotationStatusRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub price_data: Vec<CreatePartPriceOptionsAndUpdateQuotationStatusRequestPriceData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartPriceOptionsAndUpdateQuotationStatusRequestPriceData {
    pub part_id: String,
    pub price: Money,
    pub deadline: DateTime<Utc>,
}
