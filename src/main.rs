#![allow(dead_code)]
mod tokenization;

extern crate core;

use std::fmt;
use std::path::Path;

#[derive(Debug)]
enum Literal {
    Int(i32),
    Str(String),
}

#[derive(Debug)]
enum TokenMembers {
    Identifier(String),
    Literal(Literal),
    Operator(char),
    Separator(char),
}

#[derive(Debug)]
struct Expression {
    token: ReservedWords,
    members: Vec<TokenMembers>,
}

#[derive(Debug)]
enum ReservedWords {
    Function,
    Int,
    Str,
    If,
}

const FUNCTION: &str = "関数"; // かんすう
const INT: &str = "整数"; // なら
const IF: &str = "なら"; // せいすう
const STR: &str = "文字列"; // もじれつ

impl ReservedWords {
    fn as_str(&self) -> &str {
        match self {
            ReservedWords::Function => FUNCTION,
            ReservedWords::If => IF,
            ReservedWords::Int => INT,
            ReservedWords::Str => STR,
        }
    }
}

impl fmt::Display for ReservedWords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReservedWords::Function => write!(f, "{}", FUNCTION),
            ReservedWords::If => write!(f, "{}", IF),
            ReservedWords::Int => write!(f, "{}", INT),
            ReservedWords::Str => write!(f, "{}", STR),
        }
    }
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

fn tokenizer(contents: String) -> Option<Vec<Vec<Expression>>> {
    Some(vec![
        tokenization::tokenize_int(&contents)?,
        tokenization::tokenize_str(&contents)?,
    ])
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
            match tokenizer(std::fs::read_to_string(source_file).unwrap()) {
                Some(x) => x,
                None => panic!("Lexical error"), // todo form error
            }
            .iter()
            .for_each(|tokens| {
                tokens.iter().for_each(|token| {
                    println!("{:?}", token);
                });
            });
        }
        Err(e) => panic!("{}", e),
    };
}
