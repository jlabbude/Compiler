use crate::lexer::reserved::{Operator, ReservedWord, Separator};
use crate::lexer::tokenization::{tokenize_identifier, LexicalError, RawToken, Splitter};
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

const FALSE: &str = "偽";
const TRUE: &str = "真";

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Double(f64),
    Str(Str),
    Char(Char),
    Bool(Bool),
}

impl TryFrom<String> for Literal {
    type Error = String;
    fn try_from(assignment: String) -> Result<Self, Self::Error> {
        if let Ok(int) = assignment.parse::<i32>() {
            Ok(Literal::Int(int))
        } else if let Ok(jp_int) = assignment.split_and_parse_jp_numerals() {
            Ok(Literal::Int(jp_int))
        } else if let Ok(float) = assignment.parse::<f32>() {
            Ok(Literal::Float(float))
        } else if let Ok(double) = assignment.parse::<f64>() {
            Ok(Literal::Double(double))
        }
        else if assignment.eq(FALSE) || assignment.eq(TRUE) {
            Ok(Literal::Bool(match assignment.as_str() {
                FALSE => Bool::False,
                TRUE => Bool::True,
                _ => unreachable!(),
            }))
        } else if assignment.chars().next().eq(&Some(
            Separator::OpenQuotation.to_string().chars().next().unwrap(),
        )) && assignment.chars().last().eq(&Some(
            Separator::CloseQuotation
                .to_string()
                .chars()
                .next()
                .unwrap(),
        )) {
            match assignment.chars().collect::<Vec<char>>().len() {
                3 => {
                    Ok(Literal::Char(Char {
                        open_quote: Separator::OpenQuotation,
                        content: assignment
                            .chars()
                            .skip(1)
                            .take(assignment.chars().count() - 2)
                            .next()
                            .unwrap(),
                        close_quote: Separator::CloseQuotation,
                    }))
                },
                _ => {
                    Ok(Literal::Str(Str {
                        open_quote: Separator::OpenQuotation,
                        content: assignment
                            .chars()
                            .skip(1)
                            .take(assignment.chars().count() - 2)
                            .collect(),
                        close_quote: Separator::CloseQuotation,
                    }))
                }
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
            Literal::Float(float) => write!(f, "{float}"),
            Literal::Double(double) => write!(f, "{double}"),
            Literal::Str(str) =>
                write!(f, "{}{}{}",
                       str.open_quote,
                       str.content,
                       str.close_quote),
            Literal::Char(char) =>
                write!(f, "{}{}{}",
                       char.open_quote,
                       char.content,
                       char.close_quote),
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
    Separator(Separator),
    Operator(Operator),
}

impl Token {
    pub fn try_from(raw_token: RawToken) -> Result<Self, LexicalError> {
        let token = raw_token.1.to_string();
        match ReservedWord::try_from(token) {
            Ok(reserved_word) => match reserved_word {
                ReservedWord::Function => Ok(Token::ReservedWord(ReservedWord::Function)),
                ReservedWord::Return => Ok(Token::ReservedWord(ReservedWord::Return)),
                ReservedWord::If => Ok(Token::ReservedWord(ReservedWord::If)),
                ReservedWord::Else => Ok(Token::ReservedWord(ReservedWord::Else)),
                ReservedWord::Int => Ok(Token::ReservedWord(ReservedWord::Int)),
                ReservedWord::Str => Ok(Token::ReservedWord(ReservedWord::Str)),
                ReservedWord::Bool => Ok(Token::ReservedWord(ReservedWord::Bool)),
                ReservedWord::Void => Ok(Token::ReservedWord(ReservedWord::Void)),
                ReservedWord::Float => Ok(Token::ReservedWord(ReservedWord::Float)),
                ReservedWord::Double => Ok(Token::ReservedWord(ReservedWord::Double)),
                ReservedWord::Char => Ok(Token::ReservedWord(ReservedWord::Char)),
                ReservedWord::Struct => Ok(Token::ReservedWord(ReservedWord::Struct)),
                ReservedWord::Enum => Ok(Token::ReservedWord(ReservedWord::Enum)),
            },
            Err(token) => match Literal::try_from(token) {
                Ok(literal) => Ok(Token::Literal(literal)),
                Err(token) => match Separator::try_from(token.as_str()) {
                    Ok(separator) => Ok(Token::Separator(separator)),
                    Err(token) => match Operator::try_from(token.as_str()) {
                        Ok(operator) => Ok(Token::Operator(operator)),
                        Err(_) => match tokenize_identifier(raw_token) {
                            Ok(identifier) => Ok(Token::Identifier(identifier)),
                            Err(error) => Err(error),
                        },
                    },
                },
            },
        }
    }
}
