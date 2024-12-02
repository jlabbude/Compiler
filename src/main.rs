#![allow(dead_code)]
extern crate core;

use std::fs::File;
use std::io::Write;
use crate::lexer::tokenization::{tokenize, Splitter};
use crate::lexer::tokens::Token;
use crate::parser::function::Function;
use crate::parser::grammar::{Parser, ParsingRule, Symbol};
use crate::parser::program::Program;
use lexer::reserved::Separator;
use std::path::Path;
use crate::parser::enumeration::Enumeration;
use crate::parser::structure::Struct;

mod lexer;
mod parser;

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
        //eprintln!("Please use the input file as the only argument, {args:?}");
        //std::process::exit(1);
        Path::new("input.glim")
    } else {
        Path::new(&args[1])
    };
    match check_file(source_file) {
        Ok(_) => {
            let code =  &std::fs::read_to_string(source_file)
                .unwrap()
                .replace("\r\n", "\n");
            let mut lexical_output = File::create("output/lexical_output.csv").unwrap();
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
            let raw_tokens: Vec<String> = code.split_code().into_iter().map(|raw| {
                let raw = match raw {
                    Some(raw) => raw,
                    None => return "".to_string(),
                };
                match Token::try_from(raw) {
                    Ok(_token) => raw.1.to_string(),
                    Err(e) => {
                        eprintln!("{}", e);
                        std::process::exit(1);
                    }
                }
            }).filter(|raw| {
                raw != "" && raw != "\n" && raw != " " && !raw.starts_with("//") && !raw.starts_with("/*")
            }).collect();
            tokens.iter().zip(raw_tokens.iter()).for_each(|(token, a)| {
                let token = format!("{:?}", token).chars().into_iter().filter(|c| *c != '"' && *c != '\'').collect::<String>();
                let a = a.chars().into_iter().filter(|c| *c != '"' && *c != '\'').collect::<String>();
                lexical_output.write_all(format!("{},{}\n", token, a).as_bytes()).unwrap();
            });
            let table = &mut Program::parsing_table();
            table.append(&mut Function::parsing_table());
            table.append(&mut Enumeration::parsing_table());
            table.append(&mut Struct::parsing_table());
            let mut syntax_output = File::create("output/syntax_output.csv").unwrap();
            match ParsingRule::parse_with_table(&tokens, table) {
                Ok(x) => {
                    x.into_iter().for_each(|(nt, production)| {
                        syntax_output.write_all(format!("<{:?}>,{production}\n", nt, production = {
                            production
                                .iter()
                                .skip(1)
                                .map(|symbol| match symbol {
                                    Symbol::Terminal(terminal) => format!("{terminal:?} ").chars().into_iter().filter(|c| *c != '"' && *c != '\'').collect::<String>(),
                                    Symbol::NonTerminal(non_terminal) => format!("<{non_terminal:?}> ").chars().into_iter().filter(|c| *c != '"' && *c != '\'').collect::<String>(),
                                })
                                .collect::<String>()
                        }).as_bytes()).unwrap();
                    });
                },
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            }
            /*tokens.iter().for_each(|expression| match expression {
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
            });*/
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
