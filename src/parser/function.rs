#![allow(non_upper_case_globals)]
use crate::lexer::reserved::{ReservedWord, Separator};
use crate::lexer::tokens::{Literal, Token};
use crate::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, Terminal};

// These are just a hacky way of bypassing the parser by creating "id" and "literal" terminals
// that are always valid because of matches_token() on grammar.rs
const id: Terminal = Terminal::Token(Token::Identifier(String::new()));
const literal: Terminal = Terminal::Token(Token::Literal(Literal::Int(0)));

pub struct Function;

impl Parser for Function {
    /// \<Func> :: func \<DataType> id ( \<FuncArgument> ) { \<FuncBody> } \<S> <br/>
    /// \<FuncArgument> :: \<DataType> id \<FuncArgument> | , \<DataType> id \<FuncArgument> | e
    /// \<FuncBody> :: e TODO
    fn parsing_table() -> Grammar {
        vec![
            ParsingRule {
                non_terminal: NonTerminal::Func,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Function)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(
                        ReservedWord::Function,
                    ))),
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::FuncBody),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Program),
                ],
            },

            // <FuncArgument> :: <DataType> id <FuncArgument> | , <DataType> id <FuncArgument> | e
            ParsingRule {
                non_terminal: NonTerminal::FuncArgument,
                token: Terminal::DataType,
                production: vec![
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgument,
                token: Terminal::Token(Token::Separator(Separator::Comma)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgument,
                token: Terminal::Epsilon,
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },


            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: Terminal::Epsilon,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::StatementList),
                ],
            },

            ParsingRule {
                non_terminal: NonTerminal::StatementList,
                token: Terminal::Epsilon,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::Statement),
                    Symbol::NonTerminal(NonTerminal::StatementList),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StatementList,
                token: Terminal::Epsilon,
                production: vec![
                    Symbol::Terminal(Terminal::Epsilon),
                ],
            },

            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Epsilon,
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Return))),
                    Symbol::NonTerminal(NonTerminal::Expression),
                ],
            },

            ParsingRule {
                non_terminal: NonTerminal::Expression,
                token: literal,
                production: vec![
                    Symbol::Terminal(literal),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                ]
            },

        ]
    }
}
