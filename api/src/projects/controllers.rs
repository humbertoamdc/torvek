use crate::app_state::AppState;
use crate::projects::models::inputs::{
    CreateProjectInput, DeleteProjectInput, GetProjectByIdInput,
};
use crate::projects::usecases::create_project::CreateProject;
use crate::projects::usecases::delete_project::DeleteProject;
use crate::projects::usecases::get_project::GetProject;
use crate::projects::usecases::query_projects_by_customer::{
    QueryProjectsByCustomer, QueryProjectsByCustomerInput,
};
use crate::shared::extractors::session::CustomerSession;
use crate::shared::into_error_response::IntoError;
use crate::shared::UseCase;
use axum::extract::{Path, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateProjectRequest {
    pub project_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct QueryProjectsForCustomerQuery {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub name: Option<String>,
    pub cursor: Option<String>,
    pub limit: Option<i32>,
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
    let usecase = CreateProject::new(app_state.projects.projects_repository);
    let result = usecase.execute(input).await;

    match result {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}

pub async fn query_projects_by_customer(
    State(app_state): State<AppState>,
    Query(query): Query<QueryProjectsForCustomerQuery>,
    CustomerSession(session): CustomerSession,
) -> impl IntoResponse {
    let input = QueryProjectsByCustomerInput::new(
        session.identity,
        query.from,
        query.to,
        query.name,
        query.cursor,
        query.limit,
    );
    let usecase = QueryProjectsByCustomer::new(app_state.projects.projects_repository);
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
    let usecase = GetProject::new(app_state.projects.projects_repository);
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
    let usecase = DeleteProject::new(
        app_state.projects.projects_repository,
        app_state.quotes.quotes_repository,
        app_state.parts.parts_repository,
        app_state.parts.object_storage,
    );
    let result = usecase.execute(input).await;

    match result {
        Ok(response) => Ok((StatusCode::NO_CONTENT, Json(response))),
        Err(err) => Err(err.into_error_response()),
    }
}
