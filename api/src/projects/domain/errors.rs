#[derive(thiserror::Error, Debug)]
pub enum ProjectsError {
    #[error("error while creating project")]
    CreateProjectError,
    #[error("error while querying projects")]
    QueryProjectsError,
    #[error("an unexpected error occurred")]
    UnknownError,
}
