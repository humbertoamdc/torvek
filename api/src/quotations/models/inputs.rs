use crate::auth::models::session::Identity;
use crate::quotations::models::quotation::QuoteStatus;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateQuotationInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryQuotationsForProjectInput {
    pub identity: Identity,
    pub project_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetQuotationByIdInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetQuotationSubtotalInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteQuotationInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateQuotationInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadQuotePdfInput {
    pub identity: Identity,
    pub project_id: String,
    pub quotation_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryQuotationsByStatusInput {
    pub status: QuoteStatus,
}
