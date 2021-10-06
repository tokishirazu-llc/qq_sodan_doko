use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("{0}")]
    LogicError(String),

    #[error("{0} is not found.")]
    NotFound(String),

    #[error("authentication error.")]
    Authentication,

    #[error("permission denied.")]
    PermissionDenied,

    #[error("{0} is no implement.")]
    NoImplement(String),

    #[error("json error. {0}")]
    Json(#[from] serde_json::Error),

    #[error("api connection error. {0}")]
    HttpError(#[from] reqwest::Error),
}
