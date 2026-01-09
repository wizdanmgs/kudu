use thiserror::Error;

#[derive(Debug, Error)]
pub enum TodoError {
    #[error("Todo not found")]
    NotFound,

    #[error("Failed to read todo file")]
    ReadError(#[from] std::io::Error),

    #[error("Failed to parse todo file")]
    ParseError(#[from] serde_json::Error),
}
