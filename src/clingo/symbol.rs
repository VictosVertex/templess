//! This module provides a safe Rust interface for interacting with
//! the Clingo symbol API.

use crate::clingo::bindings::{
    clingo_symbol_arguments, clingo_symbol_name, clingo_symbol_number, clingo_symbol_type,
};

use super::bindings::{clingo_symbol_t, clingo_symbol_to_string, clingo_symbol_to_string_size};
use std::ffi::CStr;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolType {
    Infimum = 0,
    Number = 1,
    String = 4,
    Function = 5,
    Supremum = 7,
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

    pub fn number(&self) -> Result<i32, String> {
        if self.kind() != SymbolType::Number {
            return Err("Symbol is not a number".into());
        }
        let mut val = 0;
        unsafe {
            if !clingo_symbol_number(self.0, &mut val) {
                return Err("Failed to retrieve number".into());
            }
        }
        Ok(val)
    }

    pub fn name(&self) -> Result<String, String> {
        if self.kind() != SymbolType::Function {
            return Err("Symbol is not a function".into());
        }
        let mut name_ptr = std::ptr::null();
        unsafe {
            if !clingo_symbol_name(self.0, &mut name_ptr) {
                return Err("Failed to retrieve name".into());
            }
            if name_ptr.is_null() {
                return Ok("".into());
            }
            Ok(CStr::from_ptr(name_ptr).to_string_lossy().into_owned())
        }
    }

    pub fn arguments(&self) -> Result<Vec<Symbol>, String> {
        if self.kind() != SymbolType::Function {
            return Err("Symbol is not a function".into());
        }
        let mut args_ptr: *const clingo_symbol_t = std::ptr::null();
        let mut size: usize = 0;

        unsafe {
            if !clingo_symbol_arguments(self.0, &mut args_ptr, &mut size) {
                return Err("Failed to retrieve arguments".into());
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
