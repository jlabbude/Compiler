use crate::reserved::{ReservedWord, Separator};
use crate::{Expression, Literal, Token};
use regex::Regex;

pub fn tokenize_int(contents: &str) -> Option<Vec<Expression>> {
    // ALL KANA CHARACTERS: [一-龠]|[ぁ-ゔ]
    // [a-zA-Z][a-zA-Z1-9\s] saving it
    let mut tokens: Vec<Expression> = Vec::new();
    Regex::new(&format!(
        r"；*\s*{}\s+([一-龠]|[ぁ-ゔ])+\s+(＝)\s*([1-9])+\s*(；)",
        ReservedWord::Int
    ))
    .unwrap()
    .captures_iter(contents)
    .for_each(|capture| {
        tokens.push(Expression {
            token: ReservedWord::Int,
            members: vec![
                Token::Identifier(String::from(capture.get(1).unwrap().as_str())),
                Token::Operator(capture.get(2).unwrap().as_str().chars().last().unwrap()),
                Token::Literal(Literal::Int(
                    capture.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                )),
                Token::Separator(Separator::Terminator),
            ],
        });
    });
    match tokens.len() {
        0 => None,
        _ => Some(tokens),
    }
}

pub fn tokenize_str(contents: &str) -> Option<Vec<Expression>> {
    let mut tokens: Vec<Expression> = Vec::new();
    Regex::new(r"([一-龠]|[ぁ-ゔ])+\s+(＝)\s*(「.*」)+\s*(；)")
        .unwrap()
        .captures_iter(contents)
        .for_each(|capture| {
            tokens.push(Expression {
                token: ReservedWord::Str,
                members: vec![
                    Token::Identifier(String::from(capture.get(1).unwrap().as_str())),
                    Token::Operator(capture.get(2).unwrap().as_str().chars().last().unwrap()),
                    Token::Literal(Literal::Str(String::from(capture.get(3).unwrap().as_str()))),
                    Token::Separator(Separator::Terminator),
                ],
            });
        });
    match tokens.len() {
        0 => None,
        _ => Some(tokens),
    }
}

pub fn get_token(contents: &str) -> impl Iterator<Item = Option<Vec<Expression>>> + '_ {
    contents
        .split("；")
        // FIXME this will require function signatures to end with "};"
        .map(|word| match ReservedWord::try_from(word) {
            Ok(reserved_word) => match reserved_word {
                ReservedWord::Int => tokenize_int(word),
                ReservedWord::Str => tokenize_str(word),
                ReservedWord::Function => todo!(),
                ReservedWord::If => todo!(),
            },
            Err(_identifier) => {
                todo!()
            }
        })
}
