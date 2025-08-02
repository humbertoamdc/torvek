use crate::app_state::AppState;
use crate::parts::models::inputs::{
    AdminQueryPartsForQuotationInput, CreatePartQuotesInput, CreatePartsInput, DeletePartInput,
    GetPartInput, QueryPartsForQuotationInput, UpdatePartInput, UpdateSelectedPartQuoteInput,
};
use crate::parts::models::part::{PartAttributes, PartProcess};
use crate::parts::usecases::admin_query_parts_for_quotation::AdminQueryPartsForQuotation;
use crate::parts::usecases::create_part_quotes::CreatePartQuotes;
use crate::parts::usecases::create_parts::CreateParts;
use crate::parts::usecases::delete_drawing_file::{DeleteDrawingFile, DeleteDrawingFileInput};
use crate::parts::usecases::delete_part::DeletePart;
use crate::parts::usecases::generate_presigned_url::{
    GeneratePresignedUrl, GeneratePresignedUrlInput,
};
use crate::parts::usecases::get_part::GetPart;
use crate::parts::usecases::query_parts_by_quotation::QueryPartsByQuotation;
use crate::parts::usecases::update_part::UpdatePart;
use crate::parts::usecases::update_selected_part_quote::UpdateSelectedPartQuote;
use crate::parts::usecases::upload_drawing::{UploadDrawing, UploadDrawingInput};
use crate::services::object_storage::ObjectStorageOperation;
use crate::shared::extractors::session::{AdminSession, CustomerSession};
use crate::shared::file::File;
use crate::shared::into_error_response::IntoError;
use crate::shared::{CustomerId, PartId, PartQuoteId, ProjectId, QuoteId, UseCase};
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsRequest {
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub file_names: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UploadDrawingRequest {
    pub file_name: String,
}

#[derive(Deserialize)]
pub struct QueryPartsForQuotationQueryParameters {
    pub with_quotation_subtotal: Option<bool>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePartRequest {
    pub customer_id: CustomerId,
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
    pub drawing_file: Option<File>,
    pub process: Option<PartProcess>,
    pub attributes: Option<PartAttributes>,
    pub quantity: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSelectedPartQuoteRequest {
    pub quotation_id: QuoteId,
    pub part_id: PartId,
    pub selected_part_quote_id: PartQuoteId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateModelUploadUrlRequest {
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlRequest {
    pub project_id: ProjectId,
    pub quotation_id: QuoteId,
    pub part_id: PartId,
    pub file_name: String,
    pub file_url: Option<Url>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GeneratePresignedUrlRequest {
    pub key: String,
    pub operation: ObjectStorageOperation,
}

pub async fn admin_create_part_quotes(
    State(app_state): State<AppState>,
    AdminSession(_): AdminSession,
    Json(request): Json<CreatePartQuotesInput>,
) -> impl IntoResponse {
    let usecase = CreatePartQuotes::new(
        app_state.parts.dynamodb_parts,
        app_state.quotes.dynamodb_quotes,
        Arc::new(Mutex::new(app_state.payments.transaction)),
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn admin_query_parts_for_quotation(
    State(app_state): State<AppState>,
    Path((customer_id, quotation_id)): Path<(CustomerId, QuoteId)>,
    Query(params): Query<QueryPartsForQuotationQueryParameters>,
    AdminSession(_): AdminSession,
) -> impl IntoResponse {
    let input = AdminQueryPartsForQuotationInput {
        customer_id,
        quotation_id,
        with_quotation_subtotal: params.with_quotation_subtotal.unwrap_or(false),
        cursor: params.cursor,
        limit: params.limit.unwrap_or(100),
    };
    let usecase = AdminQueryPartsForQuotation::new(app_state.parts.dynamodb_parts);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_part(
    State(app_state): State<AppState>,
    Path(part_id): Path<PartId>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = GetPartInput {
        identity: session.identity,
        part_id,
    };
    let get_part_usecase = GetPart::new(app_state.parts.dynamodb_parts);
    let response = get_part_usecase.execute(input).await;

    match response {
        Ok(part) => Ok((StatusCode::OK, Json(part))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn create_parts(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<CreatePartsRequest>,
) -> impl IntoResponse {
    let input = CreatePartsInput {
        identity: session.identity,
        project_id: request.project_id,
        quotation_id: request.quotation_id,
        file_names: request.file_names,
    };
    let usecase = CreateParts::new(
        app_state.parts.dynamodb_parts,
        app_state.quotes.dynamodb_quotes,
        app_state.parts.s3,
        app_state.payments.stripe_client,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn upload_part_drawing(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Path((project_id, quote_id, part_id)): Path<(ProjectId, QuoteId, PartId)>,
    Json(request): Json<UploadDrawingRequest>,
) -> impl IntoResponse {
    let input = UploadDrawingInput {
        customer: session.identity,
        project_id,
        quote_id,
        part_id,
        file_name: request.file_name,
    };
    let usecase = UploadDrawing::new(
        app_state.parts.dynamodb_parts,
        app_state.quotes.dynamodb_quotes,
        app_state.parts.s3,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn query_parts_for_quotation(
    State(app_state): State<AppState>,
    Path((project_id, quotation_id)): Path<(String, String)>,
    Query(params): Query<QueryPartsForQuotationQueryParameters>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = QueryPartsForQuotationInput {
        identity: session.identity,
        quotation_id,
        project_id,
        with_quotation_subtotal: params.with_quotation_subtotal.unwrap_or(false),
        cursor: params.cursor,
        limit: params.limit.unwrap_or(10),
    };
    let usecase = QueryPartsByQuotation::new(app_state.parts.dynamodb_parts);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn update_part(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<UpdatePartRequest>,
) -> impl IntoResponse {
    let input = UpdatePartInput {
        identity: session.identity,
        project_id: request.project_id,
        quotation_id: request.quotation_id,
        part_id: request.part_id,
        drawing_file: request.drawing_file,
        process: request.process,
        attributes: request.attributes,
        quantity: request.quantity,
    };
    let usecase = UpdatePart::new(
        app_state.parts.dynamodb_parts,
        app_state.quotes.dynamodb_quotes,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(part) => Ok((StatusCode::OK, Json(part))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn update_selected_part_quote(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<UpdateSelectedPartQuoteRequest>,
) -> impl IntoResponse {
    let input = UpdateSelectedPartQuoteInput {
        identity: session.identity,
        quotation_id: request.quotation_id,
        part_id: request.part_id,
        selected_part_quote_id: request.selected_part_quote_id,
    };
    let usecase = UpdateSelectedPartQuote::new(app_state.parts.dynamodb_parts);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn delete_part(
    State(app_state): State<AppState>,
    Path((project_id, quotation_id, part_id)): Path<(String, String, String)>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = DeletePartInput {
        identity: session.identity,
        project_id,
        quotation_id,
        part_id,
    };
    let usecase = DeletePart::new(
        app_state.parts.dynamodb_parts,
        app_state.quotes.dynamodb_quotes,
        app_state.parts.s3,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn generate_presigned_url(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<GeneratePresignedUrlRequest>,
) -> impl IntoResponse {
    let input = GeneratePresignedUrlInput {
        identity: session.identity,
        key: request.key,
        operation: request.operation,
    };
    let usecase = GeneratePresignedUrl::new(app_state.parts.s3);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn delete_drawing_file(
    State(app_state): State<AppState>,
    Path((project_id, quote_id, part_id)): Path<(ProjectId, QuoteId, PartId)>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = DeleteDrawingFileInput {
        customer: session.identity,
        project_id,
        quote_id,
        part_id,
    };
    let usecase = DeleteDrawingFile::new(
        app_state.parts.dynamodb_parts,
        app_state.quotes.dynamodb_quotes,
        app_state.parts.s3,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}
