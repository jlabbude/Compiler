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
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Function))),
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
                token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: Terminal::Any,
                production: vec![Symbol::NonTerminal(NonTerminal::StatementList)],
            },
            ParsingRule {
                non_terminal: NonTerminal::StatementList,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::StatementList,
                token: Terminal::Any,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::Statement),
                    Symbol::NonTerminal(NonTerminal::StatementList),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Return)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Return))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::If)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::If))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Statement),
                    Symbol::NonTerminal(NonTerminal::Statement),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Expr,
                token: literal,
                production: vec![
                    Symbol::Terminal(literal),
                    Symbol::NonTerminal(NonTerminal::ExprOperation),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Expr,
                token: id,
                production: vec![
                    Symbol::Terminal(id),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Expr,
                token: id,
                production: vec![
                    Symbol::Terminal(id),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                ],
            },


            ParsingRule {
                non_terminal: NonTerminal::ExprOperation,
                token: Terminal::UnaryOp,
                production: vec![
                    Symbol::Terminal(Terminal::UnaryOp),
                    Symbol::Terminal(literal),
                    Symbol::NonTerminal(NonTerminal::ExprOperation),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprOperation,
                token: Terminal::Token(Token::Separator(Separator::Terminator)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
        ]
    }
}
