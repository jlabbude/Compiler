use crate::lexer::reserved::ReservedWord;
use crate::lexer::tokens::Token;
use crate::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, Terminal};

pub struct Program;

impl Parser for Program {
    fn parsing_table() -> Grammar {
        vec![
            ParsingRule {
                non_terminal: NonTerminal::Program,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Function)),
                production: vec![Symbol::NonTerminal(NonTerminal::Func)],
            },
            // has to be last
            ParsingRule {
                non_terminal: NonTerminal::Program,
                token: Terminal::Any,
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
        ]
    }
}
