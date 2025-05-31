use crate::front::lexer::reserved::{ReservedWord, Separator};
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::id;
use crate::front::parser::grammar::{NonTerminal, Parser, ParsingRule, Symbol, Terminal};

pub struct Enumeration;

impl Parser for Enumeration {
    /// <Enum> :: enum <Identifier> { <EnumBody> }
    const PARSING_TABLE: &'static [ParsingRule<'_>] = &[
        ParsingRule {
            non_terminal: NonTerminal::Enum,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Enum)),
            production: &[
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
            production: &[
                Symbol::Terminal(id),
                Symbol::NonTerminal(NonTerminal::EnumBody),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::EnumBody,
            token: Terminal::Token(Token::Separator(Separator::Comma)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                Symbol::NonTerminal(NonTerminal::EnumBody),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::EnumBody,
            token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
    ];
}
