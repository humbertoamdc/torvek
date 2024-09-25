use api_boundary::common::into_error_response::IntoErrorResponse;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

use api_boundary::parts::requests::{
    AdminUpdatePartRequest, CreateDrawingUploadUrlRequest, CreateModelUploadUrlRequest,
    CreatePartQuotesRequest, CreatePartsRequest, GetPartRequest,
    QueryPartsForQuotationQueryParameters, QueryPartsForQuotationRequest, UpdatePartRequest,
};

use crate::app_state::AppState;
use crate::parts::usecases::admin_update_part::AdminUpdatePartUseCase;
use crate::parts::usecases::create_part_quotes::CreatePartQuotesUseCase;
use crate::parts::usecases::create_parts::CreatePartsUseCase;
use crate::parts::usecases::drawing_upload_url::CreateDrawingUploadUrlUseCase;
use crate::parts::usecases::get_part::GetPartUseCase;
use crate::parts::usecases::model_upload_url::ModelUploadUrlUseCase;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::parts::usecases::update_part::UpdatePartUseCase;
use crate::quotations::usecases::get_quotation_by_id::GetQuotationByIdUseCase;
use crate::quotations::usecases::update_quotation_status::UpdateQuotationStatusUseCase;
use crate::shared::usecase::UseCase;

pub async fn admin_update_part(
    State(app_state): State<AppState>,
    Json(request): Json<AdminUpdatePartRequest>,
) -> impl IntoResponse {
    let usecase = AdminUpdatePartUseCase::new(app_state.parts.parts_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn admin_create_part_quotes(
    State(app_state): State<AppState>,
    Json(request): Json<CreatePartQuotesRequest>,
) -> impl IntoResponse {
    let usecase = CreatePartQuotesUseCase::new(app_state.parts.part_quotes_creation);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_part(
    State(app_state): State<AppState>,
    Path(request): Path<GetPartRequest>,
) -> impl IntoResponse {
    let get_part_usecase = GetPartUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let response = get_part_usecase.execute(request).await;

    match response {
        Ok(part) => Ok((StatusCode::OK, Json(part))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn create_parts(
    State(app_state): State<AppState>,
    Json(request): Json<CreatePartsRequest>,
) -> impl IntoResponse {
    let update_quotation_status_usecase =
        UpdateQuotationStatusUseCase::new(app_state.quotations.quotations_repository);
    let usecase = CreatePartsUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
        update_quotation_status_usecase,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn query_parts_for_quotation(
    State(app_state): State<AppState>,
    Path((_, _, quotation_id)): Path<(String, String, String)>,
    Query(query_params): Query<QueryPartsForQuotationQueryParameters>,
) -> impl IntoResponse {
    let request = QueryPartsForQuotationRequest {
        quotation_id,
        with_quotation_subtotal: query_params.with_quotation_subtotal.unwrap_or(false),
    };

    let usecase = QueryPartsForQuotationUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn update_part(
    State(app_state): State<AppState>,
    Json(request): Json<UpdatePartRequest>,
) -> impl IntoResponse {
    let get_quotation_by_id_usecase =
        GetQuotationByIdUseCase::new(app_state.quotations.quotations_repository);
    let usecase = UpdatePartUseCase::new(
        app_state.parts.parts_repository,
        get_quotation_by_id_usecase,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn create_model_file_upload_url(
    State(app_state): State<AppState>,
    Json(request): Json<CreateModelUploadUrlRequest>,
) -> impl IntoResponse {
    let get_quotation_by_id_usecase =
        GetQuotationByIdUseCase::new(app_state.quotations.quotations_repository);
    let usecase = ModelUploadUrlUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
        get_quotation_by_id_usecase,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn create_drawing_upload_url(
    State(app_state): State<AppState>,
    Json(request): Json<CreateDrawingUploadUrlRequest>,
) -> impl IntoResponse {
    let get_quotation_by_id_usecase =
        GetQuotationByIdUseCase::new(app_state.quotations.quotations_repository);
    let usecase = CreateDrawingUploadUrlUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
        get_quotation_by_id_usecase,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}
