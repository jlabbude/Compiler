use crate::lexer::reserved::{Operator, ReservedWord, Separator};
use crate::lexer::tokenization::{
    tokenize_comment, tokenize_identifier, LexicalError, RawToken, Splitter,
};
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct Str {
    open_quote: Separator,
    content: String,
    close_quote: Separator,
}

#[derive(Debug)]
pub struct Char {
    open_quote: Separator,
    content: char,
    close_quote: Separator,
}

#[derive(Debug)]
pub enum Bool {
    False,
    True,
}

const FALSE: &str = "false";
const TRUE: &str = "true";

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Str(Str),
    Char(Char),
    Bool(Bool),
}

impl TryFrom<String> for Literal {
    type Error = String;
    fn try_from(assignment: String) -> Result<Self, Self::Error> {
        if let Ok(int) = assignment.split_and_parse_jp_numerals() {
            Ok(Literal::Int(int))
        } else if let Ok(long) = assignment.split_and_parse_jp_numerals() {
            Ok(Literal::Long(long))
        } else if let Ok(float) = assignment.split_and_parse_jp_numerals() {
            Ok(Literal::Float(float))
        } else if let Ok(double) = assignment.split_and_parse_jp_numerals() {
            Ok(Literal::Double(double))
        } else if assignment.eq(FALSE) || assignment.eq(TRUE) {
            Ok(Literal::Bool(match assignment.as_str() {
                FALSE => Bool::False,
                TRUE => Bool::True,
                _ => unreachable!(),
            }))
        } else if assignment.chars().next().eq(&Some(
            Separator::StringQuotation
                .to_string()
                .chars()
                .next()
                .unwrap(),
        )) && assignment.chars().last().eq(&Some(
            Separator::StringQuotation
                .to_string()
                .chars()
                .next()
                .unwrap(),
        )) {
            match assignment.chars().collect::<Vec<char>>().len() {
                3 => Ok(Literal::Char(Char {
                    open_quote: Separator::StringQuotation,
                    content: assignment
                        .chars()
                        .skip(1)
                        .take(assignment.chars().count() - 2)
                        .next()
                        .unwrap(),
                    close_quote: Separator::StringQuotation,
                })),
                _ => Ok(Literal::Str(Str {
                    open_quote: Separator::StringQuotation,
                    content: assignment
                        .chars()
                        .skip(1)
                        .take(assignment.chars().count() - 2)
                        .collect(),
                    close_quote: Separator::StringQuotation,
                })),
            }
        } else {
            Err(assignment)
        }
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Int(int) => write!(f, "{int}"),
            Literal::Long(long) => write!(f, "{long}"),
            Literal::Float(float) => write!(f, "{float}"),
            Literal::Double(double) => write!(f, "{double}"),
            Literal::Str(str) => write!(f, "{}{}{}", str.open_quote, str.content, str.close_quote),
            Literal::Char(char) => {
                write!(f, "{}{}{}", char.open_quote, char.content, char.close_quote)
            }
            Literal::Bool(bool) => match bool {
                Bool::False => write!(f, "{}", FALSE),
                Bool::True => write!(f, "{}", TRUE),
            },
        }
    }
}

#[derive(Debug)]
pub enum Token {
    ReservedWord(ReservedWord),
    Literal(Literal),
    Identifier(String),
    Comment(String),
    Separator(Separator),
    Operator(Operator),
}

impl Token {
    pub fn try_from(raw_token: RawToken) -> Result<Self, LexicalError> {
        let token = raw_token.1.to_string();
        match ReservedWord::try_from(token) {
            Ok(reserved_word) => Ok(Token::ReservedWord(reserved_word)),
            Err(token) => match Literal::try_from(token) {
                Ok(literal) => Ok(Token::Literal(literal)),
                Err(token) => match Separator::try_from(token.as_str()) {
                    Ok(separator) => Ok(Token::Separator(separator)),
                    Err(token) => match Operator::try_from(token.as_str()) {
                        Ok(operator) => Ok(Token::Operator(operator)),
                        Err(_) => match tokenize_comment(raw_token) {
                            Ok(comment) => Ok(Token::Comment(comment)),
                            Err(_) => match tokenize_identifier(raw_token) {
                                Ok(identifier) => Ok(Token::Identifier(identifier)),
                                Err(error) => Err(error),
                            },
                        },
                    },
                },
            },
        }
    }
}
