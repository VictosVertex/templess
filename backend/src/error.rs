use axum::response::IntoResponse;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to acquire mutex lock: {details}")]
    MutexLockFailed { details: String },

    #[error("No data found at the specified path: {path}")]
    DataMissing { path: String },

    #[error("Core error: {0}")]
    Core(#[from] crate::core::error::CoreError),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let error_message = self.to_string();
        let status = match self {
            Error::DataMissing { .. } => axum::http::StatusCode::NOT_FOUND,
            Error::MutexLockFailed { .. } | Error::Core(_) => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        (status, error_message).into_response()
    }
}
