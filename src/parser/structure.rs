use crate::lexer::reserved::{ReservedWord, Separator};
use crate::lexer::tokens::Token;
use crate::parser::grammar::id;
use crate::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, Terminal};

pub struct Struct;

impl Parser for Struct {
    /// <Struct> :: struct <Identifier> { <StructBody> }
    fn parsing_table() -> Grammar {
        vec![
            ParsingRule {
                non_terminal: NonTerminal::Struct,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Struct)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Struct))),
                    Symbol::Terminal(id),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::OpenCurlyBraces))),
                    Symbol::NonTerminal(NonTerminal::StructBody),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::CloseCurlyBraces))),
                    Symbol::NonTerminal(NonTerminal::Program),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StructBody,
                token: Terminal::DataType,
                production: vec![
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::StructBody),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StructBody,
                token: Terminal::Token(Token::Separator(Separator::Comma)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                    Symbol::NonTerminal(NonTerminal::StructBody),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StructBody,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
        ]
    }
}