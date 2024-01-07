use crate::app_state::AppState;
use crate::projects::usecases::create_project::CreateProjectUseCase;
use crate::projects::usecases::UseCase;
use api_boundary::projects::requests::CreateProjectRequest;
use axum::extract::State;
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
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}
