use crate::reserved::{Operator, ReservedWord, Separator};
use crate::{Assignment, Expression, Literal, Token};
use regex::Regex;

pub fn re_split_function_declaration<'r>() -> Regex {
    Regex::new(
        &format!(r"\s*{}\s+([一-龠ぁ-ゔ]+)\s*（\s*((?:{}|{}|{})\s+[一-龠ぁ-ゔ]+(?:\s*、\s*(?:{}|{}|{})\s+[一-龠ぁ-ゔ]+)*)*\s*）\s*｛([\s\S]*)｝",
                 ReservedWord::Function, ReservedWord::Int, ReservedWord::Str, ReservedWord::Bool, ReservedWord::Int, ReservedWord::Str, ReservedWord::Bool))
        .unwrap()
}

pub fn tokenize_int_declaration(contents: &str) -> Option<Expression> {
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
                Token::Assignment(Assignment::Literal(Literal::Int(
                    capture.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                ))),
                Token::Separator(Separator::Terminator),
            ],
        })
    })
}

pub fn tokenize_str_declaration(contents: &str) -> Option<Expression> {
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
                Token::Assignment(Assignment::Literal(Literal::Str(String::from(
                    capture.get(3).unwrap().as_str(),
                )))),
                Token::Separator(Separator::Terminator),
            ],
        })
    })
}

pub fn tokenize_identifier_operation(contents: &str, operator: Operator) -> Option<Expression> {
    Regex::new(&format!(r"\s*([一-龠]+|[ぁ-ゔ]+)\s*{operator}\s*(.*)\s*；"))
        .unwrap()
        .captures(contents)
        .and_then(|capture| {
            Some(Expression {
                token: ReservedWord::Str,
                members: vec![
                    Token::Identifier(String::from(capture.get(1).unwrap().as_str())),
                    Token::Operator(operator.to_string().chars().last().unwrap()),
                    Token::Assignment(
                        Assignment::try_from(capture.get(2).unwrap().as_str())
                            .unwrap_or_else(|identifier| identifier),
                    ),
                    Token::Separator(Separator::Terminator),
                ],
            })
        })
}

pub fn get_token(contents: &str) -> Vec<Option<Expression>> {
    re_split_function_declaration().captures_iter(contents) // FIXME regex currently ignores things outside function call
        .flat_map(|function_capture| {
            function_capture.get(3).unwrap().as_str().split_inclusive('；').map(|expression| {
                let mut expression_tokens = expression.trim().split_whitespace();
                match ReservedWord::try_from(expression_tokens.next().clone()?) // iterated to grab the first token here hence mut
                {
                    Ok(reserved_word) => match reserved_word { // if first token is a reserved word, tokenize as declaration
                        ReservedWord::Int => tokenize_int_declaration(&expression),
                        ReservedWord::Str => tokenize_str_declaration(&expression),
                        ReservedWord::Bool => todo!(),
                        ReservedWord::If => todo!(),
                        ReservedWord::Function => panic!("Function inside function"),
                    },
                    Err(_) => {
                        match Operator::try_from(expression_tokens.next().clone()?) {
                            Ok(operator) => {
                                match operator {
                                    Operator::Assignment => { tokenize_identifier_operation(&expression, operator) }
                                    Operator::Sum => { tokenize_identifier_operation(&expression, operator) }
                                    Operator::Subtraction => { tokenize_identifier_operation(&expression, operator) }
                                    Operator::Multiplication => { tokenize_identifier_operation(&expression, operator) }
                                    Operator::Division => { tokenize_identifier_operation(&expression, operator) }
                                }
                            }
                            Err(_e) => {
                                None
                            }
                        }
                    }
                }
            })
        })
        .collect()
}
