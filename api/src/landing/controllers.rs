use crate::app_state::AppState;
use crate::landing::usecases::contact_admins::{ContactAdmins, ContactAdminsInput};
use crate::shared::into_error_response::IntoError;
use crate::shared::UseCase;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;

pub async fn contact_admins(
    State(app_state): State<AppState>,
    Json(request): Json<ContactAdminsInput>,
) -> impl IntoResponse {
    let usecase = ContactAdmins::new(app_state.services.emailer.ses);
    let result = usecase.execute(request).await;

    match result {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(err) => Err(err.into_error_response()),
    }
}
