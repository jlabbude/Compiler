use crate::lexer::reserved::{Operator, Separator};
use crate::lexer::tokens::Token;
use regex::bytes::Regex;
use std::str;

pub type LexicalError = String;

pub fn tokenize_identifier(identifier: &str) -> Result<String, LexicalError> {
    match Regex::new(r"^[一-龠ぁ-ゔァ-ヴー＿][一-龠ぁ-ゔァ-ヴー＿０-９]*$")
        .unwrap()
        .is_match(identifier.as_bytes())
    {
        true => Ok(identifier.to_string()),
        false => Err(format!(r#"Lexical error with: "{}""#, identifier)),
    }
}

fn read_split_code_to_vec(code: &str) -> Vec<Option<&str>> {
    let re = Regex::new(&format!(
        r"(?:「[\S\s]*」|[{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}\s])",
        Separator::OpenQuotation,
        Separator::CloseQuotation,
        Separator::OpenCurlyBraces,
        Separator::CloseCurlyBraces,
        Separator::OpenParentheses,
        Separator::CloseParentheses,
        Separator::Terminator,
        Separator::Comma,
        Separator::Dot,
        Operator::Assignment,
        Operator::Sum,
        Operator::Subtraction,
        Operator::Multiplication,
        Operator::Division,
        Operator::Equality,
        Operator::Inequality,
        Operator::GreaterThan,
        Operator::LessThan,
        Operator::GreaterThanOrEqual,
        Operator::LessThanOrEqual,
    ))
    .unwrap();

    re.split(code.as_bytes())
        .zip(re.find_iter(code.as_bytes()))
        .flat_map(|(token, separator)| {
            [
                match token.len() {
                    0 => None,
                    _ => Some(str::from_utf8(token).unwrap()),
                },
                match separator.len() {
                    0 => None,
                    _ => Some(str::from_utf8(separator.as_bytes()).unwrap()),
                },
            ]
        })
        .collect()
}

pub fn tokenize(source_code_contents: &str) -> Vec<Token> {
    read_split_code_to_vec(source_code_contents)
        .iter()
        .flatten()
        .map(|token| match Token::try_from(token.to_string()) {
            Ok(token) => token,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        })
        .collect::<Vec<Token>>()
}
