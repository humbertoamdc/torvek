use crate::app_state::AppState;
use crate::quotations::models::inputs::{
    AdminQueryQuotationsByStatusInput, CreateQuotationInput, DeleteQuotationInput,
    DownloadQuotePdfInput, GetQuotationByIdInput, GetQuotationSubtotalInput,
    QueryQuotationsForProjectInput, UpdateQuotationInput,
};
use crate::quotations::models::quotation::QuoteStatus;
use crate::quotations::usecases::admin_query_quotations_by_status::AdminQueryQuotationsByStatus;
use crate::quotations::usecases::create_quotation::CreateQuotation;
use crate::quotations::usecases::delete_quotation::DeleteQuotation;
use crate::quotations::usecases::download_quote_pdf::DowanloadQuotePdf;
use crate::quotations::usecases::get_quotation::GetQuotation;
use crate::quotations::usecases::get_quotation_subtotal::GetQuotationSubtotal;
use crate::quotations::usecases::query_quotations_by_project::QueryQuotationsByProject;
use crate::quotations::usecases::update_quotation::UpdateQuotation;
use crate::shared::extractors::session::{AdminSession, CustomerSession};
use crate::shared::into_error_response::IntoError;
use crate::shared::{ProjectId, QuoteId, UseCase};
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use http::{header, StatusCode};
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateQuotationRequest {
    pub quotation_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SendQuotationForReviewRequest {
    pub project_id: String,
    pub quotation_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminQueryQuotationsByStatusQueryParams {
    pub status: QuoteStatus,
}

pub async fn create_quotation(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Path(project_id): Path<ProjectId>,
    Json(request): Json<CreateQuotationRequest>,
) -> impl IntoResponse {
    let input = CreateQuotationInput {
        identity: session.identity,
        project_id,
        quotation_name: request.quotation_name,
    };
    let usecase = CreateQuotation::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn query_quotations_for_project(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Path(project_id): Path<String>,
) -> impl IntoResponse {
    let input = QueryQuotationsForProjectInput {
        identity: session.identity,
        project_id,
    };
    let usecase = QueryQuotationsByProject::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_quotation_by_id(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Path(quotation_id): Path<QuoteId>,
) -> impl IntoResponse {
    let input = GetQuotationByIdInput {
        identity: session.identity,
        quotation_id,
    };
    let usecase = GetQuotation::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(quotation) => Ok((StatusCode::OK, Json(quotation))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_quotation_subtotal(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Path(quotation_id): Path<QuoteId>,
) -> impl IntoResponse {
    let input = GetQuotationSubtotalInput {
        identity: session.identity,
        quotation_id,
    };
    let usecase = GetQuotationSubtotal::new(
        app_state.parts.parts_repository,
        app_state.quotations.quotations_repository,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn delete_quotation(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Path(quotation_id): Path<QuoteId>,
) -> impl IntoResponse {
    let input = DeleteQuotationInput {
        identity: session.identity,
        quotation_id,
    };
    let usecase = DeleteQuotation::new(
        app_state.quotations.quotations_repository,
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::NO_CONTENT, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn send_quotation_for_review(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<SendQuotationForReviewRequest>,
) -> impl IntoResponse {
    let input = UpdateQuotationInput {
        identity: session.identity,
        project_id: request.project_id,
        quotation_id: request.quotation_id,
    };
    let usecase = UpdateQuotation::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(quotation) => Ok((StatusCode::OK, Json(quotation))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn download_pdf_quote(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Path(quotation_id): Path<QuoteId>,
) -> impl IntoResponse {
    let input = DownloadQuotePdfInput {
        identity: session.identity,
        quotation_id,
    };
    let usecase = DowanloadQuotePdf::new(
        app_state.parts.parts_repository,
        app_state.quotations.quotations_repository,
        app_state.payments.stripe_client,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => {
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/pdf; charset=utf-8")
                .header(
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"quote.pdf\"",
                ) // Optional, for download
                .body(axum::body::Body::from(response)) // Pass the Vec<u8> directly
                .unwrap()
        }
        Err(_) => {
            // TODO: Standardize this response and error.
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(axum::body::Body::from(())) // Pass the Vec<u8> directly
                .unwrap()
        }
    }
}

pub async fn admin_query_quotations_by_status(
    State(app_state): State<AppState>,
    Query(params): Query<AdminQueryQuotationsByStatusQueryParams>,
    AdminSession(_): AdminSession,
) -> impl IntoResponse {
    let input = AdminQueryQuotationsByStatusInput {
        status: params.status,
    };
    let usecase = AdminQueryQuotationsByStatus::new(app_state.quotations.quotations_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}
