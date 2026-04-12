//! This module provides a safe Rust interface for interacting with
//! the Clingo symbol API.

use crate::clingo::bindings::{
    clingo_symbol_arguments, clingo_symbol_name, clingo_symbol_number, clingo_symbol_type,
};
use crate::clingo::error::ClingoError;

use super::bindings::{clingo_symbol_t, clingo_symbol_to_string, clingo_symbol_to_string_size};
use std::ffi::CStr;
use std::fmt::{Display, Error, Formatter};

/// The type of a Clingo symbol.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolType {
    /// The infimum symbol, representing the smallest possible value.
    Infimum = 0,
    /// A numeric symbol, representing an integer value.
    Number = 1,
    /// A string symbol, representing a sequence of characters enclosed in double quotes.
    String = 4,
    /// A function symbol, representing a function with optional name and arguments.
    Function = 5,
    /// The supremum symbol, representing the largest possible value.
    Supremum = 7,
    /// An unrecognized or unsupported symbol type.
    Unknown,
}
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

    /// Returns the type of the symbol.
    ///
    /// # Returns
    /// - The `SymbolType` of the symbol.
    pub fn kind(&self) -> SymbolType {
        let t = unsafe { clingo_symbol_type(self.0) };
        match t {
            0 => SymbolType::Infimum,
            1 => SymbolType::Number,
            4 => SymbolType::String,
            5 => SymbolType::Function,
            7 => SymbolType::Supremum,
            _ => SymbolType::Unknown,
        }
    }

    /// Returns the numeric value of the symbol if it is a number.
    ///
    /// # Returns
    /// - `Ok(i32)` if the symbol is a number.
    ///
    /// # Errors
    /// - `Err(String)` if the symbol is not a number or if retrieval fails.
    pub fn number(&self) -> Result<i32, ClingoError> {
        if self.kind() != SymbolType::Number {
            return Err(ClingoError::new_type_error("Symbol is not a number"));
        }
        let mut val = 0;
        unsafe {
            if !clingo_symbol_number(self.0, &mut val) {
                return Err(ClingoError::new_internal(
                    "Failed to retrieve number".to_string(),
                ));
            }
        }
        Ok(val)
    }

    /// Returns the name of the symbol if it is a function.
    ///
    /// # Returns
    /// - `Ok(String)` if the symbol is a function.
    ///
    /// # Errors
    /// - `Err(String)` if the symbol is not a function or if retrieval fails.
    pub fn name(&self) -> Result<String, ClingoError> {
        if self.kind() != SymbolType::Function {
            return Err(ClingoError::new_type_error("Symbol is not a function"));
        }
        let mut name_ptr = std::ptr::null();
        unsafe {
            if !clingo_symbol_name(self.0, &mut name_ptr) {
                return Err(ClingoError::new_internal(
                    "Failed to retrieve name".to_string(),
                ));
            }
            if name_ptr.is_null() {
                return Ok("".into());
            }
            Ok(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
        }
    }

    /// Returns the arguments of the symbol if it is a function.
    ///
    /// # Returns
    /// - `Ok(Vec<Symbol>)` if the symbol is a function.
    ///
    /// # Errors
    /// - `Err(String)` if the symbol is not a function or if retrieval fails.
    pub fn arguments(&self) -> Result<Vec<Symbol>, ClingoError> {
        if self.kind() != SymbolType::Function {
            return Err(ClingoError::new_type_error("Symbol is not a function"));
        }
        let mut args_ptr: *const clingo_symbol_t = std::ptr::null();
        let mut size: usize = 0;

        unsafe {
            if !clingo_symbol_arguments(self.0, &mut args_ptr, &mut size) {
                return Err(ClingoError::new_internal(
                    "Failed to retrieve arguments".to_string(),
                ));
            }
            if size == 0 {
                return Ok(Vec::new());
            }
            let slice = std::slice::from_raw_parts(args_ptr, size);
            Ok(slice.iter().map(|&s| Symbol(s)).collect())
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
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
