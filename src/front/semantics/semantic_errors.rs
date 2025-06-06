use crate::front::semantics::types::{IdentifierTypeCell, TypeCell};
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
    IncompatibleTypes(TypeCell, TypeCell),
    InvalidExpressionWithVariant(String, TypeCell),
    InvalidStructUsage(IdentifierTypeCell, TypeCell),
    UnexpectedSymbolInExpression(String),
    InvalidEnumVariant(String, TypeCell),
    RedeclaredIdentifier(String, String),
    VariantNotDeclared(TypeCell, String, IdentifierTypeCell),
}

impl Error for SemanticError {}
