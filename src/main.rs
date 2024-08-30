mod tokenization;

extern crate core;

use core::fmt;
use std::path::Path;

#[derive(Debug)]
enum Literal {
    Int(i32),
    String(String),
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

// かんすう
const FUNCTION: &str = "関数";
// せいすう
const INT: &str = "整数";
// なら
const IF: &str = "なら";

#[derive(Debug)]
enum ReservedWords {
    Function(String),
    Int(String),
    If(String),
}

impl fmt::Display for ReservedWords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReservedWords::Function(s) => write!(f, "{}", s),
            ReservedWords::Int(s) => write!(f, "{}", s),
            ReservedWords::If(s) => write!(f, "{}", s),
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

fn tokenizer(contents: String) -> Option<Vec<Expression>> {
    tokenization::tokenize_int(&contents)
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
            let tokens = tokenizer(std::fs::read_to_string(source_file).unwrap()).unwrap();
            println!("{:?}", tokens);
        },
        Err(e) => panic!("{}", e),
    };
}
