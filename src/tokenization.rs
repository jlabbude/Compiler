use crate::reserved::{Operator, ReservedWord, Separator};
use crate::{Assignment, Literal, Token, VarDeclaration};
use regex::Regex;

pub fn re_split_function_declaration() -> Regex {
    Regex::new(&format!(
        r"\s*{}\s+([一-龠ぁ-ゔ]+)\s*（\s*((?:{}|{}|{})\s+[一-龠ぁ-ゔ]+]
        (?:\s*、\s*(?:{}|{}|{})\s+[一-龠ぁ-ゔ]+)*)*\s*）\s*｛([\s\S]*)｝",
        ReservedWord::Function,
        ReservedWord::Int,
        ReservedWord::Str,
        ReservedWord::Bool,
        ReservedWord::Int,
        ReservedWord::Str,
        ReservedWord::Bool
    ))
    .unwrap()
}

pub fn tokenize_int_declaration(contents: &str) -> Option<VarDeclaration> {
    // ALL KANA CHARACTERS: [一-龠]|[ぁ-ゔ]
    // [a-zA-Z][a-zA-Z1-9\s] saving it
    Regex::new(&format!(
        r"\s*{}\s+([一-龠]+|[ぁ-ゔ]+)\s+(＝)\s*([1-9])+\s*；",
        ReservedWord::Int
    ))
    .unwrap()
    .captures(contents)
    .and_then(|capture| {
        Some(VarDeclaration {
            token: ReservedWord::Int,
            identifier: Token::Identifier(String::from(capture.get(1).unwrap().as_str())),
            operator: Token::Operator(capture.get(2).unwrap().as_str().chars().last().unwrap()),
            assigment: Assignment::Literal(Literal::Int(
                capture.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            )),
            separator: Token::Separator(Separator::Terminator),
        })
    })
}

pub fn tokenize_str_declaration(contents: &str) -> Option<VarDeclaration> {
    Regex::new(&format!(
        r"\s*{}\s+([一-龠]+|[ぁ-ゔ]+)\s+(＝)\s*(「.*」)\s*；",
        ReservedWord::Str
    ))
    .unwrap()
    .captures(contents)
    .and_then(|capture| {
        Some(VarDeclaration {
            token: ReservedWord::Str,
            identifier: Token::Identifier(String::from(capture.get(1).unwrap().as_str())),
            operator: Token::Operator(capture.get(2).unwrap().as_str().chars().last().unwrap()),
            assigment: Assignment::Literal(Literal::Str(String::from(
                capture.get(3).unwrap().as_str(),
            ))),
            separator: Token::Separator(Separator::Terminator),
        })
    })
}

pub fn tokenize_identifier_operation(contents: &str, operator: Operator) -> Option<VarDeclaration> {
    Regex::new(&format!(r"\s*([一-龠]+|[ぁ-ゔ]+)\s*{operator}\s*(.*)\s*；"))
        .unwrap()
        .captures(contents)
        .and_then(|capture| {
            Some(VarDeclaration {
                token: ReservedWord::Str,
                identifier: Token::Identifier(String::from(capture.get(1).unwrap().as_str())),
                operator: Token::Operator(operator.to_string().chars().last().unwrap()),
                assigment: Assignment::try_from(capture.get(2).unwrap().as_str())
                    .unwrap_or_else(|identifier| identifier),
                separator: Token::Separator(Separator::Terminator),
            })
        })
}

pub fn get_token(contents: &str) -> Vec<Option<VarDeclaration>> {
    re_split_function_declaration().captures_iter(contents) // FIXME regex currently ignores things outside function call
        .flat_map(|function_declaration_capture| {
            function_declaration_capture.get(3).unwrap().as_str().split_inclusive(Separator::Terminator.to_string().chars().next().unwrap()).map(|expression| {
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
