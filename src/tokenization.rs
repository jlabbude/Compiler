use regex::Regex;
use crate::{Expression, Literal, ReservedWords, TokenMembers, INT};

pub fn tokenize_int(contents: &str) -> Option<Vec<Expression>> {
    // ALL KANA CHARACTERS: [一-龠|ぁ-ゔ]
    let mut tokens: Vec<Expression> = Vec::new();
    Regex::new(&format!(
        r"；*\s*({})\s+([ぁ-ゔ]|[a-zA-Z])+\s+(＝)\s*([1-9])+\s*(；)",
        INT
    ))
        .unwrap()
        .captures_iter(contents)
        .for_each(|capture| {
            tokens.push(Expression {
                token: ReservedWords::Int(capture.get(1).unwrap().as_str().to_string()),
                members: vec![
                    TokenMembers::Identifier(capture.get(2).unwrap().as_str().to_string()),
                    TokenMembers::Operator(capture.get(3).unwrap().as_str().chars().last().unwrap()),
                    TokenMembers::Literal(Literal::Int(
                        capture.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                    )),
                    TokenMembers::Separator(capture.get(5).unwrap().as_str().chars().last().unwrap()),
                ],
            });
        });
    match tokens.len() {
        0 => None,
        _ => Some(tokens),
    }
}