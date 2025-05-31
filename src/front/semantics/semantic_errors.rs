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
}

impl Error for SemanticError {
    
}