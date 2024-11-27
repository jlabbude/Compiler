use crate::lexer::reserved::ReservedWord;
use crate::lexer::tokens::Token;
use crate::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, TerminalTokens};

pub struct Program;

impl Parser for Program {
    fn parsing_table() -> Grammar {
        vec![
            ParsingRule {
                non_terminal: NonTerminal::Program,
                token: TerminalTokens::Epsilon,
                production: vec![Symbol::Terminal(TerminalTokens::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::Program,
                token: TerminalTokens::Token(Token::ReservedWord(ReservedWord::Function)),
                production: vec![Symbol::NonTerminal(NonTerminal::Function)],
            },
        ]
    }
}
