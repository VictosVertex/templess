//! This module provides a safe Rust interface for interacting with
//! the Clingo symbol API.

use super::bindings::{clingo_symbol_t, clingo_symbol_to_string, clingo_symbol_to_string_size};
use std::ffi::CStr;
use std::fmt::{Display, Error, Formatter, Result};

/// A Clingo symbol.
#[derive(Debug, Clone)]
pub struct Symbol(clingo_symbol_t);

impl Symbol {
    /// Creates a new `Symbol` from a `clingo_symbol_t`.
    ///
    /// # Parameters
    /// - `symbol`: The `clingo_symbol_t` to wrap.
    ///
    /// # Returns
    /// - A new `Symbol` instance.
    pub fn new(symbol: clingo_symbol_t) -> Self {
        Symbol(symbol)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut size = 0;
        let success = unsafe { clingo_symbol_to_string_size(self.0, &mut size) };
        if !success {
            writeln!(f, "Failed to get symbol string size")?;
            return Err(Error);
        }

        if size == 0 {
            return Ok(());
        }

        let mut buffer = vec![0i8; size];
        let success = unsafe { clingo_symbol_to_string(self.0, buffer.as_mut_ptr(), size) };
        if !success {
            writeln!(f, "Failed to convert symbol to string")?;
            return Err(Error);
        }

        let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
        match c_str.to_str() {
            Ok(s) => write!(f, "{s}"),
            Err(_) => {
                writeln!(f, "Failed to convert CStr to str")?;
                Err(Error)
            }
        }
    }
}
