use crate::lexer::tokens::Token;
use crate::lexer::reserved::{ReservedWord, Separator};

#[derive(Debug, Clone, PartialEq)]
pub enum NonTerminal {
    Function,
    DataType,
    Arguments,
    Argument,
    Body,
    Statement,
    ArgumentsTail,
}

#[derive(Debug, Clone)]
pub enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Test),
    Epsilon,
}

#[derive(Clone, Debug)]
pub enum Test {
    Token(Token),
    DataType,
}

pub struct ParsingRule {
    pub non_terminal: NonTerminal,
    pub token: Test,
    pub production: Vec<Symbol>,
}

pub fn construct_parsing_table() -> Vec<ParsingRule> {
    vec![
        ParsingRule {
            non_terminal: NonTerminal::Function,
            token: Test::Token(Token::ReservedWord(ReservedWord::Function)),
            production: vec![
                Symbol::Terminal(Test::Token(Token::ReservedWord(ReservedWord::Function))),
                Symbol::NonTerminal(NonTerminal::DataType),
                Symbol::Terminal(Test::Token(Token::Identifier(String::new()))),
                Symbol::Terminal(Test::Token(Token::Separator(Separator::OpenParenthesis))),
                Symbol::NonTerminal(NonTerminal::Arguments),
                Symbol::Terminal(Test::Token(Token::Separator(Separator::CloseParenthesis))),
                Symbol::Terminal(Test::Token(Token::Separator(Separator::OpenCurlyBraces))),
                Symbol::NonTerminal(NonTerminal::Body),
                Symbol::Terminal(Test::Token(Token::Separator(Separator::CloseCurlyBraces))),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::Arguments,
            token: Test::DataType,
            production: vec![
                Symbol::NonTerminal(NonTerminal::Argument),
                Symbol::NonTerminal(NonTerminal::ArgumentsTail),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ArgumentsTail,
            token: Test::Token(Token::Separator(Separator::Comma)),
            production: vec![
                Symbol::Terminal(Test::Token(Token::Separator(Separator::Comma))),
                Symbol::NonTerminal(NonTerminal::Argument),
                Symbol::NonTerminal(NonTerminal::ArgumentsTail),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ArgumentsTail,
            token: Test::Token(Token::Separator(Separator::CloseParenthesis)),
            production: vec![Symbol::Epsilon],
        },
        ParsingRule {
            non_terminal: NonTerminal::Argument,
            token: Test::DataType,
            production: vec![
                Symbol::Terminal(Test::DataType),
                Symbol::Terminal(Test::Token(Token::Identifier(String::new()))),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::DataType,
            token: Test::DataType,
            production: vec![Symbol::Terminal(Test::DataType)],
        },
        ParsingRule {
            non_terminal: NonTerminal::DataType,
            token: Test::DataType,
            production: vec![Symbol::Terminal(Test::DataType)],
        },
        ParsingRule {
            non_terminal: NonTerminal::Body,
            token: Test::Token(Token::Separator(Separator::OpenCurlyBraces)),
            production: vec![
                Symbol::Terminal(Test::Token(Token::Separator(Separator::OpenCurlyBraces))),
                Symbol::Terminal(Test::Token(Token::Separator(Separator::CloseCurlyBraces))),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::Body,
            token: Test::Token(Token::Separator(Separator::CloseCurlyBraces)),
            production: vec![Symbol::Epsilon],
        },
    ]
}

fn find_rule<'a>(
    table: &'a [ParsingRule],
    non_terminal: &NonTerminal,
    token: &Token,
) -> Option<&'a ParsingRule> {
    table.iter().find(|rule| {
        rule.non_terminal == *non_terminal
            && (matches_token(&rule.token, token) || is_dynamic_match(rule, token))
    })
}

fn is_dynamic_match(rule: &ParsingRule, token: &Token) -> bool {
    match rule.non_terminal {
        NonTerminal::DataType => is_data_type(token),
        NonTerminal::Arguments => matches!(token, Token::Separator(Separator::CloseParenthesis)),
        _ => false,
    }
}

fn is_data_type(token: &Token) -> bool {
    matches!(
        token,
        Token::ReservedWord(ReservedWord::Int)
            | Token::ReservedWord(ReservedWord::Float)
            | Token::ReservedWord(ReservedWord::Void)
            | Token::ReservedWord(ReservedWord::Double)
            | Token::ReservedWord(ReservedWord::Str)
            | Token::ReservedWord(ReservedWord::Char)
            | Token::ReservedWord(ReservedWord::Bool)
    )
}

fn matches_token(expected: &Test, actual: &Token) -> bool {
    match expected {
        Test::Token(expected) => {
            match (expected, actual) {
                (Token::ReservedWord(expected), Token::ReservedWord(actual)) => expected == actual,
                (Token::Literal(_), Token::Literal(_)) => true,
                (Token::Identifier(_), Token::Identifier(_)) => true,
                (Token::Separator(expected), Token::Separator(actual)) => expected == actual,
                (Token::Operator(expected), Token::Operator(actual)) => expected == actual,
                _ => false,
            }
        }
        Test::DataType => is_data_type(actual),
    }
}

pub fn parse_with_table(tokens: &[Token], table: &[ParsingRule]) -> Result<(), String> {
    let mut stack = vec![Symbol::NonTerminal(NonTerminal::Function)];
    let mut pos = 0;

    while let Some(top) = stack.pop() {
        match top {
            Symbol::Terminal(expected) => {
                if matches_token(
                    &expected.clone(),
                    tokens
                        .get(pos)
                        .unwrap_or(&Token::Separator(Separator::NewLine)),
                ) {
                    pos += 1;
                } else {
                    return Err(format!(
                        "Expected {:?}, found {:?}",
                        expected,
                        tokens.get(pos)
                    ));
                }
            }
            Symbol::NonTerminal(nt) => {
                if let Some(rule) = find_rule(
                    table,
                    &nt,
                    tokens
                        .get(pos)
                        .unwrap_or(&Token::Separator(Separator::NewLine)),
                ) {
                    for symbol in rule.production.iter().rev() {
                        stack.push(symbol.clone());
                    }
                } else {
                    return Err(format!(
                        "No rule for NonTerminal {:?} with token {:?}",
                        nt,
                        tokens.get(pos)
                    ));
                }
            }
            Symbol::Epsilon => {}
        }
    }

    if pos == tokens.len() {
        Ok(())
    } else {
        Err(format!("Unconsumed input at position {}", pos))
    }
}
