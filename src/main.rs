#![allow(dead_code)]
extern crate core;

use crate::lexer::reserved::Separator;
use crate::lexer::tokenization::tokenize;
use crate::lexer::tokens::Token;
use std::path::Path;

mod lexer;

fn check_file(source_file: &Path) -> Result<(), String> {
    match source_file.exists() {
        false => Err(format!("{} file not found", source_file.display())),
        true => match source_file.extension().unwrap().to_str().unwrap() {
            "glim" => Ok(()),
            _ => Err("Wrong extension.".to_string()),
        },
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Please use the input file as the only argument");
        std::process::exit(1);
    }
    let source_file = Path::new(&args[1]);
    match check_file(source_file) {
        Ok(_) => {
            let tokens = tokenize(
                &std::fs::read_to_string(source_file)
                    .unwrap()
                    .replace("\r\n", "\n") // windows newline
            );
            tokens.iter().for_each(|expression| {
                match expression {
                    Token::ReservedWord(_)
                    | Token::Literal(_)
                    | Token::Identifier(_)
                    | Token::Comment(_)
                    | Token::Operator(_) => print!("{expression:?} "),
                    Token::Separator(separator) => match separator {
                        Separator::NewLine => print!("{}", Separator::NewLine),
                        Separator::WhiteSpace => print!("{}", Separator::WhiteSpace),
                        _ => print!("{expression:?} "),
                    },
                }
            });
            std::process::exit(0);
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
