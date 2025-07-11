use crate::auth::models::session::Identity;
use crate::parts::models::part::{PartAttributes, PartProcess};
use crate::shared::file::File;
use crate::shared::money::Money;
use crate::shared::{CustomerId, PartId, PartQuoteId, ProjectId, QuoteId};
use serde_derive::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub file_names: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetPartInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryPartsForQuotationInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub with_quotation_subtotal: bool,
    pub cursor: Option<String>,
    pub limit: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminQueryPartsForQuotationInput {
    pub customer_id: CustomerId,
    pub quotation_id: QuoteId,
    pub with_quotation_subtotal: bool,
    pub cursor: Option<String>,
    pub limit: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePartInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
    pub drawing_file: Option<File>,
    pub process: Option<PartProcess>,
    pub attributes: Option<PartAttributes>,
    pub quantity: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
    pub file_name: String,
    pub file_url: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateModelUploadUrlInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartQuotesInput {
    pub customer_id: CustomerId,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub data: Vec<CreatePartQuotesInputData>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartQuotesInputData {
    pub part_id: PartId,
    pub unit_price: Money,
    pub sub_total: Money,
    pub workdays_to_complete: u64,
    pub quantity: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSelectedPartQuoteInput {
    pub identity: Identity,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
    pub selected_part_quote_id: PartQuoteId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeletePartInput {
    pub identity: Identity,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
}
