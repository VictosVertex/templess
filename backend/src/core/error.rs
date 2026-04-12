pub type CoreResult<T> = core::result::Result<T, CoreError>;

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON processing failed: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Database error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}
