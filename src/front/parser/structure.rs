use crate::front::lexer::reserved::{ReservedWord, Separator};
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::id;
use crate::front::parser::grammar::{NonTerminal, Parser, ParsingRule, Symbol, Terminal};

pub struct Struct;

impl Parser for Struct {
    /// <Struct> :: struct <Identifier> { <StructBody> }
    const PARSING_TABLE: &'static [ParsingRule<'_>] = &[
        ParsingRule {
            non_terminal: NonTerminal::Struct,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Struct)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Struct))),
                Symbol::Terminal(id),
                Symbol::Terminal(Terminal::Token(Token::Separator(
                    Separator::OpenCurlyBraces,
                ))),
                Symbol::NonTerminal(NonTerminal::StructBody),
                Symbol::Terminal(Terminal::Token(Token::Separator(
                    Separator::CloseCurlyBraces,
                ))),
                Symbol::NonTerminal(NonTerminal::Program),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::StructBody,
            token: Terminal::DataType,
            production: &[
                Symbol::Terminal(Terminal::DataType),
                Symbol::Terminal(id),
                Symbol::NonTerminal(NonTerminal::StructBody),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::StructBody,
            token: Terminal::Token(Token::Separator(Separator::Comma)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                Symbol::NonTerminal(NonTerminal::StructBody),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::StructBody,
            token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
    ];
}
