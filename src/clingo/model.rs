//! This module provides a safe Rust interface for interacting with
//! the Clingo model API.

use super::{
    bindings::{
        clingo_model_number, clingo_model_symbols, clingo_model_symbols_size, clingo_model_t,
        clingo_symbol_t,
    },
    error::ClingoError,
    symbol::Symbol,
};
use std::ptr::NonNull;

/// A Clingo model.
#[derive(Debug, Clone, Copy)]
pub struct Model(NonNull<clingo_model_t>);

impl Model {
    /// Creates a new `Model` from a non-null pointer.
    ///
    /// # Parameters
    /// - `inner`: A non-null pointer to `clingo_model_t`.
    ///
    /// # Returns
    /// - A new `Model` instance.
    pub fn new(inner: NonNull<clingo_model_t>) -> Self {
        Model(inner)
    }

    /// Retrieves the running number of the model.
    ///
    /// # Returns
    /// - `Ok(u64)` containing the running number of the model if successful.
    ///
    /// # Errors
    /// - `Err(ClingoError)` if there was an error during the retrieval process
    ///     of the model number.
    pub fn number(&self) -> Result<u64, ClingoError> {
        let mut number = 0;

        let success = unsafe { clingo_model_number(self.0.as_ptr(), &mut number) };
        if !success {
            return Err(ClingoError::new_internal(
                "Failed to retrieve number from model".to_owned(),
            ));
        }

        Ok(number)
    }

    /// Retrieves the symbols of the model based on the specified show type.
    ///
    /// # Parameters
    /// - `show`: The show type to filter symbols.
    ///
    /// # Returns
    /// - `Ok(Vec<Symbol>)` containing the symbols of the model if successful.
    ///
    /// # Errors
    /// - `Err(ClingoError)` if there was an error during the retrieval process
    ///    of the model symbols.
    pub fn symbols(&self, show: u32) -> Result<Vec<Symbol>, ClingoError> {
        let mut size: usize = 0;

        let get_size_success =
            unsafe { clingo_model_symbols_size(self.0.as_ptr(), show, &mut size) };
        if !get_size_success {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_symbols_size() failed".to_owned(),
            ));
        }
        if size == 0 {
            return Ok(vec![]);
        }
        let mut symbols = vec![Symbol::new(0); size];

        let get_symbols_success = unsafe {
            clingo_model_symbols(
                self.0.as_ptr(),
                show,
                symbols.as_mut_ptr() as *mut clingo_symbol_t,
                size,
            )
        };
        if !get_symbols_success {
            return Err(ClingoError::new_internal(
                "Call to clingo_model_symbols() failed".to_owned(),
            ));
        }

        Ok(symbols)
    }
}
