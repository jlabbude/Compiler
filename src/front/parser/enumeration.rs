use crate::front::lexer::reserved::{ReservedWord, Separator};
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::id;
use crate::front::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, Terminal};

pub struct Enumeration;

impl Parser for Enumeration {
    /// <Enum> :: enum <Identifier> { <EnumBody> }
    fn parsing_table() -> Grammar {
        vec![
            ParsingRule {
                non_terminal: NonTerminal::Enum,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Enum)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Enum))),
                    Symbol::Terminal(id),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::EnumBody),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Program),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::EnumBody,
                token: id,
                production: vec![
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::EnumBody),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::EnumBody,
                token: Terminal::Token(Token::Separator(Separator::Comma)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                    Symbol::NonTerminal(NonTerminal::EnumBody),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::EnumBody,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
        ]
    }
}
