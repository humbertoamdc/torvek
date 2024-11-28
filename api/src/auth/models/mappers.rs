use crate::auth::models::responses::GetSessionResponse;
use crate::auth::models::session::Session;

pub struct GetSessionResponseMapper {}

impl GetSessionResponseMapper {
    pub fn to_api(entity: Session) -> GetSessionResponse {
        GetSessionResponse {
            id: entity.identity.id,
            email: entity.identity.traits.email,
        }
    }
}
