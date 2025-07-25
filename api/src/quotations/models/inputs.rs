use crate::auth::models::session::Identity;
use crate::quotations::models::quotation::QuoteStatus;
use crate::shared::{ProjectId, QuoteId};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateQuotationInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_name: ProjectId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryQuotationsForProjectInput {
    pub identity: Identity,
    pub project_id: ProjectId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetQuotationByIdInput {
    pub identity: Identity,
    pub quotation_id: QuoteId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetQuotationSubtotalInput {
    pub identity: Identity,
    pub quotation_id: QuoteId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeleteQuotationInput {
    pub identity: Identity,
    pub quotation_id: QuoteId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateQuotationInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadQuotePdfInput {
    pub identity: Identity,
    pub quotation_id: QuoteId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryQuotationsByStatusInput {
    pub status: QuoteStatus,
}
