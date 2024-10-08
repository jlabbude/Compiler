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
            let mut reserved_words = Vec::new();
            let mut literals = Vec::new();
            let mut identifiers = Vec::new();
            let mut separators = Vec::new();
            let mut comments = Vec::new();
            let mut operators = Vec::new();
            let tokens = tokenize(
                &std::fs::read_to_string(source_file)
                    .unwrap()
                    .replace("\r\n", "\n") // windows newline
            );
            tokens.iter().for_each(|expression| {
                match expression {
                    Token::ReservedWord(tokens) => reserved_words.push(tokens.to_string()),
                    Token::Literal(tokens) => literals.push(tokens.to_string()),
                    Token::Identifier(tokens) => identifiers.push(tokens.to_string()),
                    Token::Comment(comment) => comments.push(comment.to_string()),
                    Token::Separator(tokens) => match tokens {
                        Separator::WhiteSpace | Separator::NewLine => (),
                        _ => separators.push(tokens.to_string()),
                    },
                    Token::Operator(tokens) => operators.push(tokens.to_string()),
                }
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
            let max_length = *[
                reserved_words.len(),
                literals.len(),
                identifiers.len(),
                separators.len(),
                operators.len(),
                comments.len(),
            ]
            .iter()
            .max()
            .unwrap();
            reserved_words.resize(max_length, String::new());
            literals.resize(max_length, String::new());
            identifiers.resize(max_length, String::new());
            separators.resize(max_length, String::new());
            operators.resize(max_length, String::new());
            comments.resize(max_length, String::new());
            let file = std::fs::File::create("output/output.csv").expect("Could not create file.");
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record([
                "Reserved Words",
                "Literals",
                "Identifiers",
                "Separators",
                "Operators",
                "Comments",
            ])
            .unwrap();
            for i in 0..max_length {
                wtr.write_record([
                    &reserved_words[i],
                    &literals[i],
                    &identifiers[i],
                    &separators[i],
                    &operators[i],
                    &comments[i],
                ])
                .unwrap();
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
