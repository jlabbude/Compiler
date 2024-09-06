extern crate core;

use crate::lexer::reserved::Separator;
use crate::lexer::tokenization::tokenize;
use crate::lexer::tokens::{Literal, Token};
use std::path::Path;

mod lexer;

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
                    Token::ReservedWord(word) => {
                        println!("{expression:?}");
                        println!(r#"Value: "{word}""#);
                        println!("------------");
                    }
                    Token::Literal(literal) => {
                        match literal {
                            Literal::Str(_) => println!("{expression:#?}"),
                            _ => println!("{expression:?}"),
                        }
                        println!(r#"Value: "{literal}""#);
                        println!("------------");
                    }
                    Token::Identifier(ident) => {
                        println!("{expression:?}");
                        println!(r#"Value: "{ident}""#);
                        println!("------------");
                    }
                    Token::Separator(separator) => match separator {
                        Separator::NewLine | Separator::WhiteSpace => {}
                        _ => {
                            println!("{expression:?}");
                            println!(r#"Value: "{}""#, separator.as_str().trim());
                            println!("------------");
                        }
                    },
                    Token::Operator(operator) => {
                        println!("{expression:?}");
                        println!(r#"Value: "{operator}""#);
                        println!("------------");
                    }
                });
            println!("\nLexical analysis completed!!");
            std::process::exit(0);
        }
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }
}
