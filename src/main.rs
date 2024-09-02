#![allow(dead_code)]

mod reserved;
mod tokenization;

extern crate core;

use crate::reserved::{ReservedWord, Separator};
use crate::tokenization::get_token;
use std::path::Path;

#[derive(Debug)]
enum Literal {
    Int(i32),
    Str(String),
}

#[derive(Debug)]
enum Token {
    Identifier(String),
    Separator(Separator),
    Operator(char),
    WhiteSpace(char),
}

#[derive(Debug)]
enum Assignment {
    Literal(Literal),
    Identifier(String),
}

impl Assignment {
    fn try_from(assignment: &str) -> Result<Self, Self> {
        if let Ok(int) = assignment.parse::<i32>() {
            Ok(Assignment::Literal(Literal::Int(int)))
        } else if assignment.chars().next() == Some('「') {
            Ok(Assignment::Literal(Literal::Str(String::from(assignment))))
        } else {
            Err(Assignment::Identifier(String::from(assignment)))
        }
    }
}

#[derive(Debug)]
struct VarDeclaration {
    token: ReservedWord,
    identifier: Token,
    operator: Token,
    assigment: Assignment,
    separator: Token,
}

#[derive(Debug)]
enum Expression {
    Var,
}

fn check_file(source_file: &Path) -> Result<String, String> {
    println!("{}をコンパイルする", source_file.display());
    match source_file.exists() {
        false => Err(format!("{}見つかりません", source_file.display())),
        true => {
            println!("{}が見つかりました", source_file.display());
            if source_file.extension().unwrap().to_str().unwrap().eq("nh") {
                Ok("ファイルフォーマットが正しいです".to_string())
            } else {
                Err("ファイルフォーマットが正しくありません".to_string())
            }
        }
    }
}

fn tokenizer(contents: String) -> Vec<Option<VarDeclaration>> {
    //Some(vec![
    //    tokenization::tokenize_int(&contents)?,
    //    tokenization::tokenize_str(&contents)?,
    //])
    get_token(&contents)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("引数のサイズが不正です");
        std::process::exit(1);
    }
    let source_file = Path::new(&args[1]);
    match check_file(source_file) {
        Ok(_) => {
            tokenizer(std::fs::read_to_string(source_file).unwrap())
                .iter()
                .flatten()
                .for_each(|expression| {
                    println!("{:?}", expression);
                });
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
