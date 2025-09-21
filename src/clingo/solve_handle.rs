//! This module provides a safe Rust interface for interacting with
//! the Clingo solve handle API.

use std::ptr::NonNull;

use super::{
    bindings::{
        clingo_model_t, clingo_solve_handle_close, clingo_solve_handle_get,
        clingo_solve_handle_model, clingo_solve_handle_t,
    },
    error::ClingoError,
    model::Model,
    solve_result::SolveResult,
};

/// A handle for managing solving processes in Clingo.
pub struct SolveHandle(NonNull<clingo_solve_handle_t>);

impl SolveHandle {
    /// Creates a new `SolveHandle` from a `NonNull<clingo_solve_handle_t>`.
    ///
    /// # Parameters
    /// - `inner`: The `NonNull<clingo_solve_handle_t>` to
    ///   wrap.
    ///
    /// # Returns
    /// - A new `SolveHandle` instance.
    pub fn new(inner: NonNull<clingo_solve_handle_t>) -> Self {
        SolveHandle(inner)
    }

    /// Retrieves the model associated with the current solve handle.
    ///
    /// # Returns
    /// - `Ok(Some(Model))` if a model is available.
    /// - `Ok(None)` if no model is available.
    ///
    /// # Errors
    /// - `Err(ClingoError)` if an error occurs during retrieval.
    pub fn model(&mut self) -> Result<Option<Model>, ClingoError> {
        let mut model_ptr: *const clingo_model_t = std::ptr::null();
        let success = unsafe { clingo_solve_handle_model(self.0.as_ptr(), &mut model_ptr) };
        if !success {
            return Err(ClingoError::new_internal(
                "Failed to retrieve model from solve handle".to_owned(),
            ));
        }

        if model_ptr.is_null() {
            return Ok(None);
        }

        let model_non_null = NonNull::new(model_ptr as *mut clingo_model_t).ok_or_else(|| {
            ClingoError::new_internal("Received null pointer for model".to_owned())
        })?;

        Ok(Some(Model::new(model_non_null)))
    }

    /// Retrieves the next result of the solving process.
    ///
    /// This function blocks until a result is available.
    ///
    /// # Returns
    /// - `Ok(SolveResult)` if the result was retrieved successfully.
    /// # Errors
    /// - `Err(ClingoError)` if there was an error during the retrieval process
    ///   of the solve result.
    pub fn get(&mut self) -> Result<SolveResult, ClingoError> {
        let mut result_bits = 0;
        if !unsafe { clingo_solve_handle_get(self.0.as_ptr(), &mut result_bits) } {
            return Err(ClingoError::new_internal(
                "Call to clingo_solve_handle_get() failed".to_owned(),
            ));
        }

        SolveResult::from_bits(result_bits).ok_or(ClingoError::Bindings {
            message: "Unknown or invalid bitflag combination in clingo_solve_result.",
        })
    }
}

impl Drop for SolveHandle {
    fn drop(&mut self) {
        unsafe {
            clingo_solve_handle_close(self.0.as_ptr());
        }
    }
}
