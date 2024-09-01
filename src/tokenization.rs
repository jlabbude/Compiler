use crate::reserved::{ReservedWord, Separator};
use crate::{Expression, Literal, Token};
use regex::Regex;

pub fn tokenize_int(contents: &str) -> Option<Expression> {
    // ALL KANA CHARACTERS: [一-龠]|[ぁ-ゔ]
    // [a-zA-Z][a-zA-Z1-9\s] saving it
    Regex::new(&format!(
        r"\s*{}\s+([一-龠]+|[ぁ-ゔ]+)\s+(＝)\s*([1-9])+\s*；",
        ReservedWord::Int
    ))
    .unwrap()
    .captures(contents)
    .and_then(|capture| {
        Some(Expression {
            token: ReservedWord::Int,
            members: vec![
                Token::Identifier(String::from(capture.get(1).unwrap().as_str())),
                Token::Operator(capture.get(2).unwrap().as_str().chars().last().unwrap()),
                Token::Literal(Literal::Int(
                    capture.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                )),
                Token::Separator(Separator::Terminator),
            ],
        })
    })
}

pub fn tokenize_str(contents: &str) -> Option<Expression> {
    Regex::new(&format!(
        r"\s*{}\s+([一-龠]+|[ぁ-ゔ]+)\s+(＝)\s*(「.*」)\s*；",
        ReservedWord::Str
    ))
    .unwrap()
    .captures(contents)
    .and_then(|capture| {
        Some(Expression {
            token: ReservedWord::Str,
            members: vec![
                Token::Identifier(String::from(capture.get(1).unwrap().as_str())),
                Token::Operator(capture.get(2).unwrap().as_str().chars().last().unwrap()),
                Token::Literal(Literal::Str(String::from(capture.get(3).unwrap().as_str()))),
                Token::Separator(Separator::Terminator),
            ],
        })
    })
}

pub fn get_token(contents: &str) -> Vec<Option<Expression>> {
    contents
        .split_inclusive('；')
        // fixme function definition has to end with };
        .into_iter()
        .map(|expression| {
            match ReservedWord::try_from(expression.split_whitespace().next().unwrap()) {
                Ok(reserved_word) => match reserved_word {
                    ReservedWord::Int => tokenize_int(&expression),
                    ReservedWord::Str => tokenize_str(&expression),
                    ReservedWord::Function => todo!(),
                    ReservedWord::If => todo!(),
                },
                Err(_) => {
                    println!("{:?}", expression);
                    None
                },
            }
        })
        .collect()
}
