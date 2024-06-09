use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

use api_boundary::parts::requests::{
    AdminUpdatePartRequest, CreateDrawingUploadUrlRequest, CreatePartQuotesRequest,
    CreatePartsRequest, QueryPartQuotesForPartsRequest, QueryPartsForQuotationRequest,
    UpdatePartRequest,
};

use crate::app_state::AppState;
use crate::parts::usecases::admin_update_part::AdminUpdatePartUseCase;
use crate::parts::usecases::create_part_quotes::CreatePartQuotesUseCase;
use crate::parts::usecases::create_parts::CreatePartsUseCase;
use crate::parts::usecases::drawing_upload_url::CreateDrawingUploadUrlUseCase;
use crate::parts::usecases::query_part_quotes_for_parts::QueryPartQuotesForPartsUseCase;
use crate::parts::usecases::query_parts_for_quotation::QueryPartsForQuotationUseCase;
use crate::parts::usecases::update_part::UpdatePartUseCase;
use crate::shared::mappers::api_error_to_response::api_error_to_response;
use crate::shared::usecase::UseCase;

pub async fn admin_update_part(
    State(app_state): State<AppState>,
    Json(request): Json<AdminUpdatePartRequest>,
) -> impl IntoResponse {
    let usecase = AdminUpdatePartUseCase::new(app_state.parts.parts_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(api_error_to_response(err.into())),
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
        Err(err) => Err(api_error_to_response(err.into())),
    }
}

pub async fn create_parts(
    State(app_state): State<AppState>,
    Json(request): Json<CreatePartsRequest>,
) -> impl IntoResponse {
    let usecase = CreatePartsUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(err) => Err(api_error_to_response(err.into())),
    }
}

pub async fn query_parts_for_quotation(
    State(app_state): State<AppState>,
    Path((client_id, project_id, quotation_id)): Path<(String, String, String)>,
) -> impl IntoResponse {
    let usecase = QueryPartsForQuotationUseCase::new(
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let request = QueryPartsForQuotationRequest::new(client_id, project_id, quotation_id);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(api_error_to_response(err.into())),
    }
}

pub async fn update_part(
    State(app_state): State<AppState>,
    Json(request): Json<UpdatePartRequest>,
) -> impl IntoResponse {
    let usecase = UpdatePartUseCase::new(app_state.parts.parts_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(api_error_to_response(err.into())),
    }
}

pub async fn create_drawing_upload_url(
    State(app_state): State<AppState>,
    Json(request): Json<CreateDrawingUploadUrlRequest>,
) -> impl IntoResponse {
    let usecase = CreateDrawingUploadUrlUseCase::new(app_state.parts.object_storage);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(api_error_to_response(err.into())),
    }
}

pub async fn query_part_quotes_for_parts(
    State(app_state): State<AppState>,
    Json(request): Json<QueryPartQuotesForPartsRequest>,
) -> impl IntoResponse {
    let usecase = QueryPartQuotesForPartsUseCase::new(app_state.parts.part_quotes_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(api_error_to_response(err.into())),
    }
}
