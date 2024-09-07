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
        true => {
            if source_file.extension().unwrap().to_str().unwrap().eq("nh") {
                Ok(())
            } else {
                Err("ファイルフォーマットが正しくありません".to_string())
            }
        }
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
                    Token::ReservedWord(_) => print!("{expression:?} "),
                    Token::Literal(_) => print!("{expression:?} "),
                    Token::Identifier(_) => print!("{expression:?} "),
                    Token::Separator(separator) => match separator {
                        Separator::NewLine => println!(),
                        Separator::WhiteSpace => print!("_ "),
                        _ => print!("{expression:?} "),
                    },
                    Token::Operator(_) => print!("{expression:?} "),
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
