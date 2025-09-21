//! This module provides a safe Rust interface for interacting with
//! the Clingo configuration API.

use std::{ffi::CString, ptr::NonNull};

use super::{
    bindings::{
        clingo_configuration_map_at, clingo_configuration_root, clingo_configuration_t,
        clingo_configuration_value_set,
    },
    error::ClingoError,
};

/// Clingo configuration keys
#[derive(Debug, Clone, Copy)]
pub enum ConfigKey {
    /// Key for setting the number of solve models.
    SolveModels,
}

impl ConfigKey {
    /// Returns the string representation of the configuration key.
    ///
    /// # Returns
    /// - A static string slice representing the configuration key.
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigKey::SolveModels => "solve.models",
        }
    }
}

/// Clingo configuration wrapper.
///
/// This struct provides a safe interface to interact with
/// the underlying `clingo_configuration_t` pointer.
pub struct Configuration {
    inner: NonNull<clingo_configuration_t>,
    root: u32,
}

impl Configuration {
    /// Creates a new `Configuration` instance from a non-null pointer.
    ///
    /// # Parameters
    /// - `inner`: A non-null pointer to `clingo_configuration_t`.
    ///
    /// # Returns
    /// - `Ok(Configuration)` if successful.
    /// - `Err(ClingoError)` if there was an error retrieving the root.
    pub fn new(inner: NonNull<clingo_configuration_t>) -> Result<Self, ClingoError> {
        let mut root = 0;

        let success = unsafe { clingo_configuration_root(inner.as_ptr(), &mut root) };

        if !success {
            return Err(ClingoError::new_internal(
                "Failed to get root of configuration".to_owned(),
            ));
        }

        Ok(Configuration { inner, root })
    }

    /// Sets a configuration key to a specified value.
    ///
    /// # Parameters
    /// - `key`: The configuration key to set.
    /// - `value`: The value to set for the key.
    ///
    /// # Returns
    /// - `Ok(())` if successful.
    /// - `Err(ClingoError)` if there was an error setting the value.
    pub fn set_key_to_value(&mut self, key: ConfigKey, value: &str) -> Result<(), ClingoError> {
        let mut models_key = 0;
        let key_name = CString::new(ConfigKey::SolveModels.as_str())?;

        let cet_key_success = unsafe {
            clingo_configuration_map_at(
                self.inner.as_ptr(),
                self.root,
                key_name.as_ptr(),
                &mut models_key,
            )
        };

        if !cet_key_success {
            return Err(ClingoError::new_internal(format!(
                "Failed to get key for '{}'",
                key.as_str()
            )));
        }

        let value = CString::new(value.to_string())?;

        let set_value_success = unsafe {
            clingo_configuration_value_set(self.inner.as_ptr(), models_key, value.as_ptr())
        };

        if !set_value_success {
            return Err(ClingoError::new_internal(format!(
                "Failed to set value for '{}'",
                key.as_str()
            )));
        }

        Ok(())
    }
}
