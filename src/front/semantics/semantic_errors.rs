use std::error::Error;
use strum_macros::Display;

#[derive(Display, Debug)]
pub enum SemanticError {
    InvalidType,
    TypeMismatch,
    // UndefinedVariable,
    // UnreachableCode,
    // DivisionByZero,
    // InvalidFunctionCall,
    // UnsupportedOperation,
    // MissingReturnValue,
    // DuplicateVariable,
    // UnusedVariable,
    MissingIdentifier,
    MissingDataType,
    #[strum(serialize = "undeclared type {0}")]
    UndeclaredType(String),
    UndeclaredIdentifier(String),
}

impl Error for SemanticError {
    
}