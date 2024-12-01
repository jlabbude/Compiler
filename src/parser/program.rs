use crate::lexer::reserved::ReservedWord;
use crate::lexer::tokens::Token;
use crate::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, Terminal};

pub struct Program;

impl Parser for Program {
    /// <Program> :: <Func> | <Enum> | <Struct> | ε
    fn parsing_table() -> Grammar {
        vec![
            ParsingRule {
                non_terminal: NonTerminal::Program,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Function)),
                production: vec![Symbol::NonTerminal(NonTerminal::Func)],
            },
            ParsingRule {
                non_terminal: NonTerminal::Program,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Enum)),
                production: vec![Symbol::NonTerminal(NonTerminal::Enum)],
            },
            ParsingRule {
                non_terminal: NonTerminal::Program,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Struct)),
                production: vec![Symbol::NonTerminal(NonTerminal::Struct)],
            },
            ParsingRule {
                non_terminal: NonTerminal::Program,
                token: Terminal::Any,
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
        ]
    }
}
