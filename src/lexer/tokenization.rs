use crate::lexer::reserved::{Operator, Separator};
use crate::lexer::tokens::Token;
use regex::bytes::Regex;
use std::collections::HashMap;
use std::str;
use strum::IntoEnumIterator;

pub type LexicalError = String;

/// Tuple: (line number, raw token)
pub type RawToken<'a> = (u32, &'a str);

pub trait Splitter {
    fn split_code(&self) -> Vec<Option<RawToken>>;
    fn split_and_parse_jp_numerals<T>(&self) -> Result<T, &Self>
    where
        T: str::FromStr + Copy + PartialEq + PartialOrd;
}

impl Splitter for str {
    /// Splits the expected code as a &str to all Separators and Operators
    fn split_code(&self) -> Vec<Option<RawToken>> {
        let re = Regex::new(&format!(
            r#"(?:(?:[0-9])[.](?:[0-9]*)|"[\S\s]*"|{separators_and_operators})"#,
            separators_and_operators = {
                Separator::iter()
                    .map(|separator| regex::escape(&separator.to_string()))
                    .chain(Operator::iter().map(|operator| regex::escape(&operator.to_string())))
                    .collect::<Vec<_>>()
                    .join("|")
            },
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

    fn split_and_parse_jp_numerals<T>(&self) -> Result<T, &str>
    where
        T: str::FromStr + Copy + PartialEq + PartialOrd,
    {
        let num_map: HashMap<char, char> = vec![
            ('０', '0'),
            ('１', '1'),
            ('２', '2'),
            ('３', '3'),
            ('４', '4'),
            ('５', '5'),
            ('６', '6'),
            ('７', '7'),
            ('８', '8'),
            ('９', '9'),
            ('。', '.'),
        ]
        .into_iter()
        .collect();

        match self
            .chars()
            .map(|num| num_map.get(&num).cloned())
            .collect::<Option<String>>()
            .ok_or_else(|| self.to_string())
        {
            Ok(wa) => wa.parse::<T>(),
            Err(wa) => wa.parse::<T>(),
        }
        .map_err(|_| self)
    }
}

pub fn tokenize_identifier(raw_identifier: RawToken) -> Result<String, LexicalError> {
    let (line_number, identifier) = raw_identifier;
    match Regex::new(r"^[a-zA-Z_][a-zA-Z0-9\-_]*$")
        .unwrap()
        .is_match(identifier.as_bytes())
    {
        true => Ok(identifier.to_string()),
        false => Err(format!(
            r#"Lexical error with: "{identifier}" at line {line_number}"#
        )),
    }
}

pub fn tokenize(source_code_contents: &str) -> Vec<Token> {
    source_code_contents
        .split_code()
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
