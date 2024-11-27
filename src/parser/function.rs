use crate::lexer::reserved::{ReservedWord, Separator};
use crate::lexer::tokens::Token;
use crate::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, Terminal};

pub struct Function;

impl Parser for Function {
    fn parsing_table() -> Grammar {
        vec![
            ParsingRule {
                non_terminal: NonTerminal::Function,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Function)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Function))),
                    Symbol::NonTerminal(NonTerminal::DataType),
                    Symbol::Terminal(Terminal::Token(Token::Identifier(String::new()))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::FuncArguments),
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
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArguments,
                token: Terminal::DataType,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                    Symbol::NonTerminal(NonTerminal::FuncArgumentsTail),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgumentsTail,
                token: Terminal::Token(Token::Separator(Separator::Comma)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                    Symbol::NonTerminal(NonTerminal::FuncArgumentsTail),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgumentsTail,
                token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
                production: vec![Symbol::Epsilon],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgument,
                token: Terminal::DataType,
                production: vec![
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(Terminal::Token(Token::Identifier(String::new()))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::DataType,
                token: Terminal::DataType,
                production: vec![Symbol::Terminal(Terminal::DataType)],
            },
            ParsingRule {
                non_terminal: NonTerminal::DataType,
                token: Terminal::DataType,
                production: vec![Symbol::Terminal(Terminal::DataType)],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: Terminal::Token(Token::Separator(Separator::OpenCurlyBraces)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Epsilon],
            },
        ]
    }
}
