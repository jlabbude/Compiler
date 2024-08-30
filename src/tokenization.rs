use crate::{Expression, Literal, ReservedWords, TokenMembers, INT, STR};
use regex::Regex;

pub fn tokenize_int(contents: &str) -> Option<Vec<Expression>> {
    // ALL KANA CHARACTERS: [一-龠|ぁ-ゔ]
    let mut tokens: Vec<Expression> = Vec::new();
    Regex::new(&format!(
        r"；*\s*({})\s+([ぁ-ゔ])+\s+(＝)\s*([1-9])+\s*(；)",
        INT
    ))
    .unwrap()
    .captures_iter(contents)
    .for_each(|capture| {
        tokens.push(Expression {
            token: ReservedWords::Int(String::from(capture.get(1).unwrap().as_str())),
            members: vec![
                TokenMembers::Identifier(String::from(capture.get(2).unwrap().as_str())),
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

pub fn tokenize_str(contents: &str) -> Option<Vec<Expression>> {
    let mut tokens: Vec<Expression> = Vec::new();
    Regex::new(&format!(
        r"；*\s*({})\s+([ぁ-ゔ])+\s+(＝)\s*([a-zA-Z][a-zA-Z1-9\s]*)+\s*(；)",
        STR
    ))
    .unwrap()
    .captures_iter(contents)
    .for_each(|capture| {
        tokens.push(Expression {
            token: ReservedWords::Str(String::from(capture.get(1).unwrap().as_str())),
            members: vec![
                TokenMembers::Identifier(String::from(capture.get(2).unwrap().as_str())),
                TokenMembers::Operator(capture.get(3).unwrap().as_str().chars().last().unwrap()),
                TokenMembers::Literal(Literal::Str(String::from(capture.get(4).unwrap().as_str()))),
                TokenMembers::Separator(capture.get(5).unwrap().as_str().chars().last().unwrap()),
            ],
        });
    });
    match tokens.len() {
        0 => None,
        _ => Some(tokens),
    }
}
