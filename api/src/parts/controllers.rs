use crate::app_state::AppState;
use crate::parts::models::inputs::{
    AdminQueryPartsForQuotationInput, CreateDrawingUploadUrlInput, CreateModelUploadUrlInput,
    CreatePartsInput, DeletePartInput, GetPartInput, QueryPartsForQuotationInput, UpdatePartInput,
    UpdateSelectedPartQuoteInput,
};
use crate::parts::models::part::PartAttributes;
use crate::parts::usecases::admin_query_parts_for_quotation::AdminQueryPartsForQuotationUseCase;
use crate::parts::usecases::create_part_quotes::CreatePartQuotesUseCase;
use crate::parts::usecases::create_parts::CreatePartsUseCase;
use crate::parts::usecases::delete_part::DeletePartUseCase;
use crate::parts::usecases::drawing_upload_url::CreateDrawingUploadUrlUseCase;
use crate::parts::usecases::get_part::GetPartUseCase;
use crate::parts::usecases::model_upload_url::ModelUploadUrlUseCase;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::parts::usecases::update_part::UpdatePartUseCase;
use crate::parts::usecases::update_selected_part_quote::UpdateSelectedPartQuoteUseCase;
use crate::quotations::usecases::get_quotation_by_id::GetQuotationByIdUseCase;
use crate::shared::extractors::session::{AdminSession, CustomerSession};
use crate::shared::UseCase;
use api_boundary::common::file::File;
use api_boundary::common::into_error_response::IntoError;
use api_boundary::parts::requests::CreatePartQuotesRequest;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePartsRequest {
    pub project_id: String,
    pub quotation_id: String,
    pub file_names: Vec<String>,
}

#[derive(Deserialize)]
pub struct QueryPartsForQuotationQueryParameters {
    pub with_quotation_subtotal: Option<bool>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdatePartRequest {
    pub customer_id: String,
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
    pub drawing_file: Option<File>,
    pub process: Option<String>,
    pub attributes: Option<PartAttributes>,
    pub quantity: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateSelectedPartQuoteRequest {
    pub quotation_id: String,
    pub part_id: String,
    pub selected_part_quote_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateModelUploadUrlRequest {
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateDrawingUploadUrlRequest {
    pub project_id: String,
    pub quotation_id: String,
    pub part_id: String,
    pub file_name: String,
    pub file_url: Option<Url>,
}

pub async fn admin_create_part_quotes(
    State(app_state): State<AppState>,
    AdminSession(_): AdminSession,
    Json(request): Json<CreatePartQuotesRequest>,
) -> impl IntoResponse {
    let usecase = CreatePartQuotesUseCase::new(app_state.parts.part_quotes_creation);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn admin_query_parts_for_quotation(
    State(app_state): State<AppState>,
    Path(quotation_id): Path<String>,
    Query(params): Query<QueryPartsForQuotationQueryParameters>,
    AdminSession(_): AdminSession,
) -> impl IntoResponse {
    let input = AdminQueryPartsForQuotationInput {
        quotation_id,
        with_quotation_subtotal: params.with_quotation_subtotal.unwrap_or(false),
        cursor: params.cursor,
        limit: params.limit.unwrap_or(10),
    };
    let usecase = AdminQueryPartsForQuotationUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_part(
    State(app_state): State<AppState>,
    Path((project_id, quotation_id, part_id)): Path<(String, String, String)>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = GetPartInput {
        identity: session.identity,
        project_id,
        quotation_id,
        part_id,
    };
    let get_part_usecase = GetPartUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
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
    let usecase = CreatePartsUseCase::new(
        app_state.parts.parts_repository,
        app_state.quotations.quotations_repository,
        app_state.parts.object_storage,
        app_state.payments.stripe_client,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
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
    let usecase = QueryPartsForQuotationUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
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
    let usecase = UpdatePartUseCase::new(
        app_state.parts.parts_repository,
        app_state.quotations.quotations_repository,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
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
    let usecase = UpdateSelectedPartQuoteUseCase::new(app_state.parts.parts_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn create_model_file_upload_url(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<CreateModelUploadUrlRequest>,
) -> impl IntoResponse {
    let input = CreateModelUploadUrlInput {
        identity: session.identity,
        project_id: request.project_id,
        quotation_id: request.quotation_id,
        part_id: request.part_id,
    };
    let get_quotation_by_id_usecase =
        GetQuotationByIdUseCase::new(app_state.quotations.quotations_repository);
    let usecase = ModelUploadUrlUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
        get_quotation_by_id_usecase,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn create_drawing_upload_url(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<CreateDrawingUploadUrlRequest>,
) -> impl IntoResponse {
    let input = CreateDrawingUploadUrlInput {
        identity: session.identity,
        project_id: request.project_id,
        quotation_id: request.quotation_id,
        part_id: request.part_id,
        file_name: request.file_name,
        file_url: request.file_url,
    };
    let get_quotation_by_id_usecase =
        GetQuotationByIdUseCase::new(app_state.quotations.quotations_repository);
    let usecase = CreateDrawingUploadUrlUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
        get_quotation_by_id_usecase,
    );
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
    let usecase = DeletePartUseCase::new(
        app_state.parts.parts_repository,
        app_state.quotations.quotations_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}
