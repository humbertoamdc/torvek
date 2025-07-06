use crate::app_state::AppState;
use crate::projects::models::inputs::{
    CreateProjectInput, DeleteProjectInput, GetProjectByIdInput, QueryProjectsForClientInput,
};
use crate::projects::usecases::create_project::CreateProjectUseCase;
use crate::projects::usecases::delete_project::DeleteProjectUseCase;
use crate::projects::usecases::get_project_by_id::GetProjectByIdUseCase;
use crate::projects::usecases::query_projects_for_client::QueryProjectsForClientUseCase;
use crate::shared::extractors::session::CustomerSession;
use crate::shared::UseCase;
use api_boundary::common::into_error_response::IntoError;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateProjectRequest {
    pub project_name: String,
}

pub async fn create_project(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
    Json(request): Json<CreateProjectRequest>,
) -> impl IntoResponse {
    let input = CreateProjectInput {
        identity: session.identity,
        project_name: request.project_name,
    };
    let usecase = CreateProjectUseCase::new(app_state.projects.projects_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn query_projects_for_client(
    State(app_state): State<AppState>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = QueryProjectsForClientInput {
        identity: session.identity,
    };
    let usecase = QueryProjectsForClientUseCase::new(app_state.projects.projects_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn get_project_by_id(
    State(app_state): State<AppState>,
    Path(project_id): Path<String>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = GetProjectByIdInput {
        identity: session.identity,
        project_id,
    };
    let usecase = GetProjectByIdUseCase::new(app_state.projects.projects_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn delete_project(
    State(app_state): State<AppState>,
    Path(project_id): Path<String>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = DeleteProjectInput {
        identity: session.identity,
        project_id,
    };
    let usecase = DeleteProjectUseCase::new(
        app_state.projects.projects_repository,
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
