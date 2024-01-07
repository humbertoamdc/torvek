#[derive(thiserror::Error, Debug)]
pub enum ProjectError {
    #[error("error while creating project")]
    CreateProjectError,
}
