#![allow(dead_code)]
extern crate core;

use crate::lexer::reserved::Separator;
use crate::lexer::tokenization::tokenize;
use crate::lexer::tokens::Token;
use std::path::Path;

mod lexer;

fn check_file(source_file: &Path) -> Result<(), String> {
    match source_file.exists() {
        false => Err(format!("{}見つかりません", source_file.display())),
        true => match source_file.extension().unwrap().to_str().unwrap() {
            "nh" => Ok(()),
            _ => Err("ファイルフォーマットが正しくありません".to_string()),
        },
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("引数のサイズが不正です");
        std::process::exit(1);
    }
    let source_file = Path::new(&args[1]);
    match check_file(source_file) {
        Ok(_) => {
            tokenize(&std::fs::read_to_string(source_file).unwrap())
                .iter()
                .for_each(|expression| match expression {
                    Token::ReservedWord(_)
                    | Token::Literal(_)
                    | Token::Identifier(_)
                    | Token::Operator(_) => print!("{expression:?} "),
                    Token::Separator(separator) => match separator {
                        Separator::NewLine => println!(),
                        Separator::WhiteSpace | Separator::JpSpace => print!("_ "),
                        _ => print!("{expression:?} "),
                    },
                });
            println!("\n\nLexical analysis completed successfully!!");
            std::process::exit(0);
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
