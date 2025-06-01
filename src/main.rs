#![allow(dead_code)]
extern crate core;

use front::lexer::reserved::Separator;
use front::lexer::tokenization::tokenize;
use front::lexer::tokens::Token;
use front::parser::enumeration::Enumeration;
use front::parser::function::Function;
use front::parser::grammar::{Parser, ParsingRule};
use front::parser::program::Program;
use front::parser::structure::Struct;
use std::path::Path;

mod csv_output;
mod front;

pub type Tokens = Vec<Token>;

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
    let source_file = if args.len() != 2 {
        Path::new("input.glim")
    } else {
        Path::new(&args[1])
    };
    match check_file(source_file) {
        Ok(_) => {
            let code = &std::fs::read_to_string(source_file)
                .unwrap()
                .replace("\r\n", "\n");

            let tokens = tokenize(code)
                .into_iter()
                .filter(|token| {
                    !matches!(
                        token,
                        Token::Separator(Separator::WhiteSpace)
                            | Token::Separator(Separator::NewLine)
                            | Token::Comment(_)
                    )
                })
                .collect::<Tokens>();

            let ast = ParsingRule::parse_with_table(
                &tokens,
                &[
                    Program::PARSING_TABLE,
                    Function::PARSING_TABLE,
                    Enumeration::PARSING_TABLE,
                    Struct::PARSING_TABLE,
                ]
                .concat(),
            );
            

            match ast {
                Ok(table_output) => {
                    csv_output::lexical_csv_output(code, &tokens);
                    csv_output::ast_csv_output(&table_output);
                    match table_output.type_check() {
                        Ok(_) => println!("Type checking passed."),
                        Err(err) => {
                            eprintln!("Type checking failed: {err}");
                            std::process::exit(1);
                        }
                    }
                }
                Err(err) => {
                    eprintln!("{err}");
                    std::process::exit(1);
                }
            }
        }
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    }
}
