#[derive(thiserror::Error, Debug)]
pub enum PartsError {
    #[error("error while generating signed url")]
    PresignedUrlGenerationError,
    #[error("error while creating parts batch")]
    CreatePartsBatchError,
    #[error("error while querying parts")]
    QueryPartsError,
    #[error("error while updating part")]
    UpdatePartError,
    #[error("an unexpected error occurred")]
    UnknownError,
}
