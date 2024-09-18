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
            let mut reserved_words = Vec::new();
            let mut literals = Vec::new();
            let mut identifiers = Vec::new();
            let mut separators = Vec::new();
            let mut operators = Vec::new();
            let tokens = tokenize(&std::fs::read_to_string(source_file).unwrap());
            tokens.iter()
                .for_each(|expression| match expression {
                    Token::ReservedWord(tokens) => reserved_words.push(tokens.to_string()),
                    Token::Literal(tokens) => literals.push(tokens.to_string()),
                    Token::Identifier(tokens) => identifiers.push(tokens.to_string()),
                    Token::Separator(tokens) => match tokens {
                        Separator::WhiteSpace | Separator::NewLine => (),
                        _ => separators.push(tokens.to_string()),
                    },
                    Token::Operator(tokens) => operators.push(tokens.to_string()),
                });
            let max_length = *[
                reserved_words.len(),
                literals.len(),
                identifiers.len(),
                separators.len(),
                operators.len(),
            ].iter().max().unwrap();
            reserved_words.resize(max_length, String::new());
            literals.resize(max_length, String::new());
            identifiers.resize(max_length, String::new());
            separators.resize(max_length, String::new());
            operators.resize(max_length, String::new());
            let file = std::fs::File::create("tokens.csv").expect("Could not create file.");
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record(
                &["Reserved Words",
                        "Literals",
                        "Identifiers",
                        "Separators",
                        "Operators"])
                .unwrap();
            for i in 0..max_length {
                wtr.write_record(&[
                    &reserved_words[i],
                    &literals[i],
                    &identifiers[i],
                    &separators[i],
                    &operators[i],
                ]).unwrap();
            }
            wtr.flush().unwrap();
            std::process::exit(0);
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
