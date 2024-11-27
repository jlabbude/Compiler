use crate::lexer::reserved::{ReservedWord, Separator};
use crate::lexer::tokens::Token;
use crate::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, TerminalTokens};

pub struct Function;

impl Parser for Function {
    /// \<Function> :: func \<DataType> id ( \<FuncArgumentList> ) { \<FuncBody> } <br/>
    /// \<FuncArgumentList> :: \<FuncArgument> \<FuncArgumentsTail> | e <br/>
    /// \<FuncArgument> :: \<DataType> \<FuncArgumentsTail> <br/>
    /// \<FuncArgumentsTail> :: , \<FuncArgument> \<FuncArgumentsTail> | e <br/>
    /// \<FuncBody> :: e TODO
    fn parsing_table() -> Grammar {
        vec![
            ParsingRule {
                non_terminal: NonTerminal::Function,
                token: TerminalTokens::Token(Token::ReservedWord(ReservedWord::Function)),
                production: vec![
                    Symbol::Terminal(TerminalTokens::Token(Token::ReservedWord(
                        ReservedWord::Function,
                    ))),
                    Symbol::Terminal(TerminalTokens::DataType),
                    Symbol::Terminal(TerminalTokens::Token(Token::Identifier(String::new()))),
                    Symbol::Terminal(TerminalTokens::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::FuncArgumentList),
                    Symbol::Terminal(TerminalTokens::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(TerminalTokens::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::FuncBody),
                    Symbol::Terminal(TerminalTokens::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                    Symbol::Start,
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgumentList,
                token: TerminalTokens::DataType,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                    Symbol::NonTerminal(NonTerminal::FuncArgumentsTail),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgumentList,
                token: TerminalTokens::Token(Token::Separator(Separator::CloseParenthesis)),
                production: vec![Symbol::Start],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgumentsTail,
                token: TerminalTokens::Token(Token::Separator(Separator::Comma)),
                production: vec![
                    Symbol::Terminal(TerminalTokens::Token(Token::Separator(Separator::Comma))),
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                    Symbol::NonTerminal(NonTerminal::FuncArgumentsTail),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgumentsTail,
                token: TerminalTokens::Token(Token::Separator(Separator::CloseParenthesis)),
                production: vec![Symbol::Start],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgument,
                token: TerminalTokens::DataType,
                production: vec![
                    Symbol::Terminal(TerminalTokens::DataType),
                    Symbol::Terminal(TerminalTokens::Token(Token::Identifier(String::new()))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: TerminalTokens::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Start],
            },
        ]
    }
}
