#[derive(thiserror::Error, Debug)]
pub enum PartsError {
    #[error("error while generating signed url")]
    PresignedUrlGenerationError,
    #[error("error while writing parts to the database")]
    PartsBatchCreateError,
    #[error("error while querying parts")]
    QueryPartsError,
    #[error("an unexpected error occurred")]
    UnknownError,
}
