use serde_derive::{Deserialize, Serialize};

use crate::quotations::models::QuotationStatus;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateQuotationRequest {
    pub client_id: String,
    pub project_id: String,
}
impl CreateQuotationRequest {
    pub const fn new(client_id: String, project_id: String) -> Self {
        Self {
            client_id,
            project_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryQuotationsForProjectRequest {
    pub client_id: String,
    pub project_id: String,
}
impl QueryQuotationsForProjectRequest {
    pub const fn new(client_id: String, project_id: String) -> Self {
        Self {
            client_id,
            project_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetQuotationByIdRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryQuotationsByStatusRequest {
    pub status: QuotationStatus,
}

#[derive(Debug)]
pub struct UpdateQuotationStatusRequest {
    pub client_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub status: QuotationStatus,
}
