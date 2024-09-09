use crate::lexer::reserved::{Operator, Separator};
use crate::lexer::tokens::Token;
use regex::bytes::Regex;
use std::str;

pub type LexicalError = String;

/// Tuple: (line number, raw token)
pub type RawToken<'a> = (u32, &'a str);

trait Splitter {
    fn read_split_code_to_vec(&self) -> Vec<Option<RawToken>>;
}

impl Splitter for str {
    fn read_split_code_to_vec(&self) -> Vec<Option<RawToken>> {
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
        let mut line_num: u32 = 1;

        re.split(self.as_bytes())
            .zip(re.find_iter(self.as_bytes()))
            .flat_map(|(token, separator)| {
                [
                    match token.len() {
                        0 => None,
                        _ => Some((line_num, str::from_utf8(token).unwrap())),
                    },
                    match separator.len() {
                        0 => None,
                        _ => {
                            let separator = str::from_utf8(separator.as_bytes()).unwrap();
                            line_num = match separator {
                                "\n" => line_num + 1,
                                _ => line_num,
                            };
                            Some((line_num, separator))
                        }
                    },
                ]
            })
            .collect()
    }

}

pub fn tokenize_identifier(raw_identifier: RawToken) -> Result<String, LexicalError> {
    let (line_number, identifier) = raw_identifier;
    match Regex::new(r"^[一-龠ぁ-ゔァ-ヴー＿][一-龠ぁ-ゔァ-ヴー＿０-９]*$")
        .unwrap()
        .is_match(identifier.as_bytes())
    {
        true => Ok(identifier.to_string()),
        false => Err(format!(r#"Lexical error with: "{identifier}" at line {line_number}"#)),
    }
}

pub fn tokenize(source_code_contents: &str) -> Vec<Token> {
    source_code_contents
        .read_split_code_to_vec()
        .iter()
        .flatten()
        .map(|token| match Token::try_from(*token) {
            Ok(token) => token,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        })
        .collect::<Vec<Token>>()
}
