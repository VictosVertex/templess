//! This module provides error types for Clingo API interactions.

use std::{
    error::Error,
    ffi::{CStr, NulError},
};

use super::bindings::{
    clingo_error_code, clingo_error_e_clingo_error_bad_alloc, clingo_error_e_clingo_error_logic,
    clingo_error_e_clingo_error_runtime, clingo_error_e_clingo_error_success,
    clingo_error_e_clingo_error_unknown, clingo_error_message,
};

/// Represents error codes returned by the Clingo API.
#[derive(Debug)]
pub enum ErrorCode {
    /// No error, everything went fine
    Success,
    /// Error during execution
    Runtime,
    /// Misuse of the API
    Logic,
    /// Error during memory allocation
    BadAlloc,
    /// Errors unknown to the clingo API
    Unknown,
    /// Invalid error code, should not be used
    Invalid,
}

impl From<i32> for ErrorCode {
    fn from(error: i32) -> Self {
        match error {
            clingo_error_e_clingo_error_success => ErrorCode::Success,
            clingo_error_e_clingo_error_runtime => ErrorCode::Runtime,
            clingo_error_e_clingo_error_logic => ErrorCode::Logic,
            clingo_error_e_clingo_error_bad_alloc => ErrorCode::BadAlloc,
            clingo_error_e_clingo_error_unknown => ErrorCode::Unknown,
            invalid_code => {
                eprintln!(
                    "Invalid error code {} found in {} {}, {}.",
                    invalid_code,
                    file!(),
                    line!(),
                    column!(),
                );
                ErrorCode::Invalid
            }
        }
    }
}

/// Represents errors that can occur when interacting with the Clingo API.
#[derive(Debug)]
pub enum ClingoError {
    /// Error related to null bytes in strings
    NulError(NulError),
    /// An internal error from the Clingo API
    Internal {
        /// A descriptive message about the context of the error
        message: String,
        /// The error code returned by the Clingo API
        internal_code: ErrorCode,
        /// The detailed error message from the Clingo API
        internal_message: String,
    },
    /// An error in the Rust interface itself
    Bindings {
        /// A descriptive message about the context of the error
        message: &'static str,
    },
    /// The operation is not valid for this symbol type
    TypeError {
        /// A descriptive message about the context of the type error
        message: &'static str,
    },
}

impl std::fmt::Display for ClingoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClingoError::NulError(e) => write!(f, "NulError: {e}"),
            ClingoError::Internal {
                message,
                internal_code,
                internal_message,
            } => {
                write!(
                    f,
                    "Internal error: {message} (code: {internal_code:?}): {internal_message}"
                )
            }
            ClingoError::Bindings { message } => write!(f, "Bindings error: {message}"),
            ClingoError::TypeError { message } => write!(f, "Type error: {message}"),
        }
    }
}

impl Error for ClingoError {}

impl ClingoError {
    /// Creates a new `ClingoError::Internal` with the provided message and retrieves
    /// the internal error code and message from the Clingo API.
    ///
    /// # Parameters
    /// - `message`: A descriptive message about the context of the error.
    ///
    /// # Returns
    /// - A `ClingoError::Internal` instance containing the provided message,
    ///   the internal error code, and the internal error message from the Clingo API.
    pub fn new_internal(message: String) -> Self {
        let internal_message = unsafe {
            let ptr = clingo_error_message();
            if ptr.is_null() {
                "clingo_error_message() returned null.".to_string()
            } else {
                CStr::from_ptr(ptr)
                    .to_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|_| {
                        "clingo_error_message() returned invalid UTF-8.".to_string()
                    })
            }
        };

        let internal_code = ErrorCode::from(unsafe { clingo_error_code() });

        ClingoError::Internal {
            message,
            internal_code,
            internal_message,
        }
    }

    /// Creates a new type error with the provided message.
    ///
    /// # Parameters
    /// - `message`: A descriptive message about the type error.
    ///
    /// # Returns
    /// - A `ClingoError::TypeError` instance containing the provided message.
    pub fn new_type_error(message: &'static str) -> Self {
        ClingoError::TypeError { message }
    }
}

impl From<NulError> for ClingoError {
    fn from(error: NulError) -> Self {
        ClingoError::NulError(error)
    }
}

impl From<ClingoError> for String {
    fn from(error: ClingoError) -> String {
        error.to_string()
    }
}
