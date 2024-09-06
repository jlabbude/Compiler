use crate::lexer::reserved::Separator;
use crate::lexer::tokens::Token;
use regex::bytes::Regex;
use std::io::Read;
use std::vec::IntoIter;
use crate::Expression;

pub type LexicalError = String;

pub fn tokenize_identifier(identifier: &str) -> Result<String, LexicalError> {
    match Regex::new(r"^[一-龠ぁ-ゔァ-ヴー＿][一-龠ぁ-ゔァ-ヴー＿０-９]*$")
        .unwrap()
        .is_match(identifier.as_bytes())
    {
        true => Ok(identifier.to_string()),
        false => { Err(format!("Lexical error with: '{}'", identifier)) },
    }
}

fn split_code(code: &str) -> IntoIter<String> {
    let re = Regex::new(&format!(
        r"(?:「[\S\s]*」|[{}{}{}{}{}{}{}{}{}\s])",
        Separator::OpenQuotation,
        Separator::CloseQuotation,
        Separator::OpenCurlyBraces,
        Separator::CloseCurlyBraces,
        Separator::OpenParentheses,
        Separator::CloseParentheses,
        Separator::Terminator,
        Separator::Comma,
        Separator::Dot
    ))
    .unwrap();
    // fixme first character cannot be a separator!!
    let mut finalvec: Vec<String> = Vec::new();
    re.split(code.as_bytes())
        .zip(re.find_iter(code.as_bytes()))
        .for_each(|(token, separator)| {
            let mut token_string = String::new();
            let mut separator_string = String::new();
            token.clone().read_to_string(&mut token_string).unwrap();
            separator
                .clone()
                .as_bytes()
                .read_to_string(&mut separator_string)
                .unwrap();
            if !token_string.is_empty() { finalvec.push(token_string); }
            if !separator_string.is_empty() { finalvec.push(separator_string); }
        });
    finalvec.into_iter()
}

pub fn tokenize(source_code_contents: &str) -> Expression {
    split_code(source_code_contents)
        .map(|token| {
        match Token::try_from(token) {
            Ok(token) => token,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }).collect::<Expression>()
}
