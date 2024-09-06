use crate::lexer::reserved::{Operator, ReservedWord, Separator};
use crate::lexer::tokenization::{tokenize_identifier, LexicalError};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Str {
    open_quote: Separator,
    content: String,
    close_quote: Separator,
}

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Str(Str),
}

impl TryFrom<String> for Literal {
    type Error = String;
    fn try_from(assignment: String) -> Result<Self, Self::Error> {
        if let Ok(int) = assignment.parse::<i32>() {
            Ok(Literal::Int(int))
        } else if assignment.chars().next().eq(&Some(
            Separator::OpenQuotation.to_string().chars().next().unwrap(),
        )) && assignment.chars().last().eq(&Some(
            Separator::CloseQuotation
                .to_string()
                .chars()
                .next()
                .unwrap(),
        )) {
            Ok(Literal::Str(Str {
                open_quote: Separator::OpenQuotation,
                content: assignment
                    .chars()
                    .skip(1)
                    .take(assignment.chars().count() - 2)
                    .collect(),
                close_quote: Separator::CloseQuotation,
            }))
        } else {
            Err(assignment)
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Int(int) => write!(f, "{}", int),
            Literal::Str(str) => write!(f, "{}{}{}", str.open_quote, str.content, str.close_quote),
        }
    }
}

#[derive(Debug)]
pub enum Token {
    ReservedWord(ReservedWord),
    Literal(Literal),
    Identifier(String),
    Separator(Separator),
    Operator(Operator),
}

impl Token {
    pub fn try_from(token: String) -> Result<Self, LexicalError> {
        match ReservedWord::try_from(token) {
            Ok(reserved_word) => match reserved_word {
                ReservedWord::Function => Ok(Token::ReservedWord(ReservedWord::Function)),
                ReservedWord::If => Ok(Token::ReservedWord(ReservedWord::If)),
                ReservedWord::Int => Ok(Token::ReservedWord(ReservedWord::Int)),
                ReservedWord::Str => Ok(Token::ReservedWord(ReservedWord::Str)),
                ReservedWord::Bool => Ok(Token::ReservedWord(ReservedWord::Bool)),
            },
            Err(token) => match Literal::try_from(token) {
                Ok(literal) => Ok(Token::Literal(literal)),
                Err(token) => match Separator::try_from(token.as_str()) {
                    Ok(separator) => Ok(Token::Separator(separator)),
                    Err(token) => match Operator::try_from(token.as_str()) {
                        Ok(operator) => Ok(Token::Operator(operator)),
                        Err(token) => match tokenize_identifier(token.as_str()) {
                            Ok(identifier) => Ok(Token::Identifier(identifier)),
                            Err(error) => Err(error),
                        },
                    },
                },
            },
        }
    }
}
