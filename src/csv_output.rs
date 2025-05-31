use std::fs::File;
use std::io::Write;
use crate::front::lexer::tokenization::Splitter;
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::{NonTerminal, Symbol};
use crate::Tokens;

pub fn ast_csv_output(table_output: Vec<(NonTerminal, Vec<Symbol>)>) {
    let mut syntax_output = File::create("output/syntax_output.csv").unwrap();
    syntax_output
        .write_all("\"<Rule>\",\"Production\"\n".as_bytes())
        .unwrap();
    table_output.into_iter().for_each(|(nt, production)| {
        syntax_output
            .write_all(
                format!(
                    "\"<{:?}>\",\"{production}\"\n",
                    nt,
                    production = {
                        production
                            .iter()
                            .map(|symbol| match symbol {
                                Symbol::Terminal(terminal) => format!("{terminal:?} ")
                                    .as_str()
                                    .csv_formatter()
                                    .replace("Epsilon", "\u{03b5}"),
                                Symbol::NonTerminal(non_terminal) => {
                                    format!("<{non_terminal:?}> ").as_str().csv_formatter()
                                }
                            })
                            .collect::<String>()
                    }
                )
                .as_bytes(),
            )
            .unwrap();
    });
}

pub fn lexical_csv_output(code: &str, tokens: &Tokens) {
    let mut lexical_output = File::create("output/lexical_output.csv").unwrap();
    let raw_tokens: Vec<String> = code
        .split_code()
        .into_iter()
        .flatten()
        .map(|raw| match Token::try_from(raw) {
            Ok(_token) => raw.1.to_string(),
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1);
            }
        })
        .filter(|raw| {
            !raw.is_empty()
                && raw != "\n"
                && raw != " "
                && !raw.starts_with("//")
                && !raw.starts_with("/*")
        })
        .collect();
    lexical_output
        .write_all("\"Token\",\"Raw token\"\n".as_bytes())
        .unwrap();
    tokens.iter().zip(raw_tokens.iter()).for_each(|(token, a)| {
        lexical_output
            .write_all(
                format!(
                    "\"{}\",\"{}\"\n",
                    format!("{:?}", token).as_str().csv_formatter(),
                    a.as_str().csv_formatter()
                )
                .as_bytes(),
            )
            .unwrap();
    });
}