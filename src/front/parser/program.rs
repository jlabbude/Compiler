use crate::front::lexer::reserved::ReservedWord;
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::{NonTerminal, Parser, ParsingRule, Symbol, Terminal};

pub struct Program;

impl Parser for Program {
    /// <Program> :: <Func> | <Enum> | <Struct> | ε
    const PARSING_TABLE: &'static [ParsingRule<'_>] = &[
        ParsingRule {
            non_terminal: NonTerminal::Program,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Function)),
            production: &[Symbol::NonTerminal(NonTerminal::Func)],
        },
        ParsingRule {
            non_terminal: NonTerminal::Program,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Enum)),
            production: &[Symbol::NonTerminal(NonTerminal::Enum)],
        },
        ParsingRule {
            non_terminal: NonTerminal::Program,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Struct)),
            production: &[Symbol::NonTerminal(NonTerminal::Struct)],
        },
    ];
}
