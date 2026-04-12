//! This module provides a safe Rust interface for interacting with
//! the Clingo control API.

use std::{ffi::CString, ptr::NonNull};

use super::{
    bindings::{
        clingo_control_configuration, clingo_control_free, clingo_control_ground,
        clingo_control_load, clingo_control_new, clingo_control_solve, clingo_control_t,
        clingo_part,
    },
    configuration::Configuration,
    error::ClingoError,
    model::Model,
    solve_handle::SolveHandle,
};

/// The Clingo control structure.
#[derive(Debug)]
pub struct Control {
    inner: NonNull<clingo_control_t>,
}

impl Control {
    /// Creates a new `Control` instance.
    ///
    /// # Returns
    /// - `Ok(Control)` if the control was created successfully.
    ///
    /// # Errors
    /// - `Err(ClingoError)` if there was an error during the creation process
    ///   of the control.
    pub fn new() -> Result<Self, ClingoError> {
        let mut control_pointer: *mut clingo_control_t = std::ptr::null_mut();

        let success = unsafe {
            clingo_control_new(
                std::ptr::null(),
                0,
                None,
                std::ptr::null_mut(),
                0,
                &mut control_pointer,
            )
        };

        if !success {
            return Err(ClingoError::new_internal(
                "Failed to create clingo control".to_owned(),
            ));
        }

        let inner = NonNull::new(control_pointer).ok_or_else(|| {
            ClingoError::new_internal("Received null pointer for clingo control".to_owned())
        })?;

        Ok(Control { inner })
    }

    /// Loads a logic program from a file into the control.
    ///
    /// # Parameters
    /// - `file`: The path to the logic program file.
    ///
    /// # Returns
    /// - `Ok(())` if the program was loaded successfully.
    ///
    /// # Errors
    /// - `Err(ClingoError)` if there was an error during the loading process
    ///   of the program.
    pub fn load(&self, file: &str) -> Result<(), ClingoError> {
        let c_file = CString::new(file)?;

        let success = unsafe { clingo_control_load(self.inner.as_ptr(), c_file.as_ptr()) };

        if !success {
            return Err(ClingoError::new_internal(
                "Failed to load program into control".to_owned(),
            ));
        }

        Ok(())
    }

    /// Grounds the logic program in the control.
    ///
    /// # Returns
    /// - `Ok(())` if the program was grounded successfully.
    ///
    /// # Errors
    /// - `Err(ClingoError)` if there was an error during the grounding process
    ///   of the program.
    pub fn ground(&self) -> Result<(), ClingoError> {
        let name_cstr = CString::new("base").unwrap();
        let part = clingo_part {
            name: name_cstr.as_ptr(),
            params: std::ptr::null(),
            size: 0,
        };
        let parts = [part];

        let success = unsafe {
            clingo_control_ground(
                self.inner.as_ptr(),
                parts.as_ptr(),
                1,
                None,
                std::ptr::null_mut(),
            )
        };

        if !success {
            return Err(ClingoError::new_internal(
                "Failed to ground program".to_owned(),
            ));
        }

        Ok(())
    }

    /// Starts the solving process in asynchronous and yield mode, returning a handle to manage the solving process.
    ///
    /// # Returns
    /// - `Ok(SolveHandle)` if the solving process was started successfully.
    /// - `Err(ClingoError)` if there was an error during the starting process of the solving.
    pub fn solve(&self) -> Result<SolveHandle, ClingoError> {
        const ASYNC_YIELD_MODE: u32 = 3;

        let mut handle = std::ptr::null_mut();
        let success = unsafe {
            clingo_control_solve(
                self.inner.as_ptr(),
                ASYNC_YIELD_MODE,
                std::ptr::null(),
                0,
                None,
                std::ptr::null_mut(),
                &mut handle,
            )
        };

        if !success {
            return Err(ClingoError::new_internal(
                "Failed to start solving".to_owned(),
            ));
        }

        let inner = NonNull::new(handle).ok_or_else(|| {
            ClingoError::new_internal("Received null pointer for solve handle".to_owned())
        })?;

        Ok(SolveHandle::new(inner))
    }

    /// Retrieves the configuration associated with the control.
    ///
    /// # Returns
    /// - `Ok(Configuration)` if the configuration was retrieved successfully.
    ///
    /// # Errors
    /// - `Err(ClingoError)` if there was an error during the retrieval process
    ///   of the configuration.
    pub fn configuration_mut(&mut self) -> Result<Configuration, ClingoError> {
        let mut conf = std::ptr::null_mut();

        let success = unsafe { clingo_control_configuration(self.inner.as_ptr(), &mut conf) };
        if !success {
            return Err(ClingoError::new_internal(
                "Failed to retrieve configuration from control".to_owned(),
            ));
        }
        let conf = NonNull::new(conf).ok_or_else(|| {
            ClingoError::new_internal("Received null pointer for configuration".to_owned())
        })?;

        Configuration::new(conf)
    }
}

impl Drop for Control {
    fn drop(&mut self) {
        unsafe {
            clingo_control_free(self.inner.as_ptr());
        }
    }
}

/// Events that can occur during the solving process.
#[derive(Debug)]
pub enum SolveEvent<'a> {
    /// A model was found.
    Model(&'a mut Model),
    /// The problem is unsatisfiable.
    Unsat,
    /// Statistics are available.
    Statistics,
    /// The solving process has finished.
    Finish,
}

unsafe impl Send for Control {}
