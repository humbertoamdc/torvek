#[derive(thiserror::Error, Debug)]
pub enum PartsError {
    #[error("error while generating signed url")]
    PresignedUrlGenerationError,
    #[error("error while writing parts to the database")]
    PartsBatchCreateError,
}
