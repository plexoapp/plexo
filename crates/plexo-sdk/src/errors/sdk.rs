use thiserror::Error;

#[derive(Error, Debug)]
pub enum SDKError {
    #[error("Version not found")]
    VersionNotFound,
    #[error("Resource not found")]
    ResourceNotFound,
    #[error("SQLX Error")]
    SQLXError(#[from] sqlx::Error),
    #[error("Database Migration Error")]
    MigrateError(#[from] sqlx::migrate::MigrateError),
    #[error("Serde JSON Error")]
    SerdeJSONError(#[from] serde_json::Error),
    #[error("OpenAI Error")]
    OpenAIError(#[from] async_openai::error::OpenAIError),
}
