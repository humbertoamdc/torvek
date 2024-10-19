use crate::app_state::AppState;
use crate::projects::usecases::create_project::CreateProjectUseCase;
use crate::projects::usecases::get_project_by_id::GetProjectByIdUseCase;
use crate::projects::usecases::query_projects_for_client::QueryProjectsForClientUseCase;
use crate::shared::usecase::UseCase;
use api_boundary::common::into_error_response::IntoError;
use api_boundary::projects::requests::{
    CreateProjectRequest, GetProjectByIdRequest, QueryProjectsForClientRequest,
};
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn create_project(
    State(app_state): State<AppState>,
    Json(request): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    let usecase = CreateProjectUseCase::new(app_state.projects.projects_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn query_projects_for_client(
    State(app_state): State<AppState>,
    Path(customer_id): Path<String>,
) -> impl IntoResponse {
    let usecase = QueryProjectsForClientUseCase::new(app_state.projects.projects_repository);
    let request = QueryProjectsForClientRequest::new(customer_id);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_project_by_id(
    State(app_state): State<AppState>,
    Path(request): Path<GetProjectByIdRequest>,
) -> impl IntoResponse {
    let usecase = GetProjectByIdUseCase::new(app_state.projects.projects_repository);
    let result = usecase.execute(request).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}
