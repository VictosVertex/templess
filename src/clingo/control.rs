//! This module provides a safe Rust interface for interacting with
//! the Clingo control API.

use std::{
    ffi::{CString, c_void},
    ptr::NonNull,
};

use super::{
    bindings::{
        clingo_control_configuration, clingo_control_free, clingo_control_ground,
        clingo_control_load, clingo_control_new, clingo_control_solve, clingo_control_t,
        clingo_literal_t, clingo_model_t, clingo_part, clingo_solve_event_type_t,
    },
    configuration::Configuration,
    error::ClingoError,
    model::Model,
    solve_handle::SolveHandle,
    solve_result::SolveResult,
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

    /// Solves the grounded logic program in the control.
    ///
    /// # Parameters
    /// - `event_handler`: A closure that handles events during the solving process.
    ///
    /// # Returns
    /// - `Ok(())` if the program was solved successfully.
    ///
    /// # Errors
    /// - `Err(ClingoError)` if there was an error during the solving process
    ///   of the program.
    pub fn solve<F>(&self, mut event_handler: F) -> Result<SolveResult, ClingoError>
    where
        F: FnMut(SolveEvent) -> bool,
    {
        let mut handle = std::ptr::null_mut();
        let assumptions = [];
        let success = unsafe {
            clingo_control_solve(
                self.inner.as_ptr(),
                1,
                assumptions.as_ptr() as *const clingo_literal_t,
                0,
                Some(unsafe_solve_callback::<F>),
                &mut event_handler as *mut _ as *mut c_void,
                &mut handle,
            )
        };

        if !success {
            return Err(ClingoError::new_internal(
                "Failed to solve program".to_owned(),
            ));
        }

        let handle_inner = NonNull::new(handle).ok_or_else(|| {
            ClingoError::new_internal("Received null pointer for solve handle".to_owned())
        })?;

        let mut real_handle = SolveHandle::new(handle_inner);

        let final_result = real_handle.get()?;

        Ok(final_result)
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

unsafe extern "C" fn unsafe_solve_callback<F>(
    event_type: clingo_solve_event_type_t,
    event_data: *mut c_void,
    user_data: *mut c_void,
    _goon: *mut bool,
) -> bool
where
    F: FnMut(SolveEvent) -> bool,
{
    let event_handler = unsafe { &mut *(user_data as *mut F) };

    match event_type {
        0 => {
            let model = NonNull::new(event_data as *mut clingo_model_t).ok_or_else(|| {
                ClingoError::new_internal("Received null pointer for model".to_owned())
            });

            match model {
                Err(e) => {
                    eprintln!("Error retrieving model: {e:?}");
                    false
                }
                Ok(m) => {
                    let mut model = Model::new(m);
                    event_handler(SolveEvent::Model(&mut model))
                }
            }
        }
        1 => event_handler(SolveEvent::Unsat),
        2 => event_handler(SolveEvent::Statistics),
        3 => event_handler(SolveEvent::Finish),
        _ => {
            eprintln!("Unknown event type: {event_type:?}");
            false
        }
    }
}
