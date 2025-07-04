#![allow(non_upper_case_globals)]

use crate::front::lexer::reserved::{Operator, ReservedWord, Separator};
use crate::front::lexer::tokens::{Literal, Token};
use strum_macros::Display;

pub const id: Terminal = Terminal::Token(Token::Identifier(String::new()));
pub const literal: Terminal = Terminal::Token(Token::Literal(Literal::Int(0)));
pub const typed: Terminal = Terminal::DataType(DataType::Int);
pub const unary_op: Terminal = Terminal::UnaryOperator(Operator::Increment);

#[macro_export]
macro_rules! rule {
    ($nt:expr => $t:expr => [ $($sym:expr),* ]) => {
        ParsingRule {
            non_terminal: $nt,
            token: $t,
            production: &[ $( $sym ),* ],
        }
    };
}

#[allow(clippy::upper_case_acronyms)]
pub struct AST(pub(crate) Vec<(NonTerminal, Vec<Symbol>)>);

#[derive(Display)]
pub enum SyntaxError {
    #[strum(serialize = "Syntax error: {0}")]
    UnexpectedToken(String),
    #[strum(serialize = "Syntax error: {0}")]
    NoRule(String),
    #[strum(serialize = "Syntax error: {0}")]
    UnconsumedInput(String),
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum NonTerminal {
    Program,
    Struct,
    Enum,

    EnumBody,
    StructBody,

    Func,
    FuncArgument,
    FuncBody,

    Statement,
    StmntList,
    StmntAssign,
    StmntDecl,
    StmntElse,
    StmntCase,

    Expr,
    ExprOperand,
    ExprOperation,
    ExprCall,
    ExprFuncCall,
    ExprArrayAccess,
    ExprFieldAccess,
    ExprFuncCallArgs,
}

pub enum Statement {
    Declaration,
    Assignment,
    FunctionCall,
    Return,
    If,
    While,
    For,
    Break,
    Continue,
    Block,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    NonTerminal(NonTerminal),
    Terminal(Terminal),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Terminal {
    Token(Token),
    DataType(DataType),
    UnaryOperator(Operator),
    ReassignOp,
    Any,
    Epsilon,
}

#[derive(Clone)]
pub struct ParsingRule<'a> {
    pub non_terminal: NonTerminal,
    pub token: Terminal,
    pub production: &'a [Symbol],
}

#[derive(Clone, Debug, PartialEq, Display)]
pub enum DataType {
    Int,
    Long,
    Float,
    Double,
    Void,
    Str,
    Char,
    Bool,
    Identifier(String),
}

impl TryFrom<ReservedWord> for DataType {
    type Error = ();
    fn try_from(value: ReservedWord) -> Result<Self, Self::Error> {
        match value {
            ReservedWord::Int => Ok(DataType::Int),
            ReservedWord::Long => Ok(DataType::Long),
            ReservedWord::Float => Ok(DataType::Float),
            ReservedWord::Void => Ok(DataType::Void),
            ReservedWord::Double => Ok(DataType::Double),
            ReservedWord::Str => Ok(DataType::Str),
            ReservedWord::Char => Ok(DataType::Char),
            ReservedWord::Bool => Ok(DataType::Bool),
            _ => Err(()),
        }
    }
}

impl From<Literal> for DataType {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Int(_) => DataType::Int,
            Literal::Long(_) => DataType::Long,
            Literal::Float(_) => DataType::Float,
            Literal::Double(_) => DataType::Double,
            Literal::Str(_) => DataType::Str,
            Literal::Char(_) => DataType::Char,
            Literal::Bool(_) => DataType::Bool,
        }
    }
}

pub trait Parser {
    const PARSING_TABLE: &'static [ParsingRule<'_>];
}

impl ParsingRule<'_> {
    fn find_rule<'a>(
        table: &'a [ParsingRule],
        non_terminal: &NonTerminal,
        token: &Token,
    ) -> Option<&'a ParsingRule<'a>> {
        table.iter().find(|rule| {
            rule.non_terminal == *non_terminal && (ParsingRule::matches_token(&rule.token, token))
        })
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
                | Token::Identifier(_)
        )
    }

    fn matches_token(expected: &Terminal, actual: &Token) -> bool {
        match expected {
            Terminal::Token(expected) => match (expected, actual) {
                (Token::ReservedWord(expected), Token::ReservedWord(actual)) => expected == actual,
                (Token::Literal(_), Token::Literal(_)) => true,
                (Token::Identifier(_), Token::Identifier(_)) => true,
                (Token::Separator(expected), Token::Separator(actual)) => expected == actual,
                (Token::Operator(expected), Token::Operator(actual)) => expected == actual,
                _ => false,
            },
            Terminal::DataType(_) => ParsingRule::is_data_type(actual),
            Terminal::UnaryOperator(_) => {
                if let Token::Operator(op) = actual {
                    matches!(
                        op,
                        Operator::GreaterThanOrEqual
                            | Operator::LessThanOrEqual
                            | Operator::Sum
                            | Operator::Subtraction
                            | Operator::Multiplication
                            | Operator::Division
                            | Operator::Inequality
                            | Operator::Negation
                            | Operator::GreaterThan
                            | Operator::LessThan
                            | Operator::Equality
                            | Operator::And
                            | Operator::Or
                    )
                } else {
                    false
                }
            }
            Terminal::ReassignOp => {
                if let Token::Operator(op) = actual {
                    matches!(
                        op,
                        Operator::Assignment | Operator::Increment | Operator::Decrement
                    )
                } else {
                    false
                }
            }
            Terminal::Any => true,
            Terminal::Epsilon => false,
        }
    }

    pub(crate) fn parse_with_table(
        tokens: &[Token],
        table: &[ParsingRule],
    ) -> Result<AST, SyntaxError> {
        let mut stack = vec![Symbol::NonTerminal(NonTerminal::Program)];
        let mut pos = 0;
        let mut raw_productions: Vec<(NonTerminal, Vec<Symbol>)> = Vec::new();

        while let Some(top) = stack.pop() {
            match top {
                Symbol::Terminal(expected) => {
                    if let Some(token) = tokens.get(pos) {
                        update_production_with_token_value(token, &expected, &mut raw_productions);
                    }
                    if ParsingRule::matches_token(
                        &expected.clone(),
                        tokens
                            .get(pos)
                            .unwrap_or(&Token::Separator(Separator::NewLine)),
                    ) {
                        if expected != Terminal::Epsilon {
                            pos += 1;
                        } else {
                            continue;
                        }
                    } else {
                        return Err(SyntaxError::UnexpectedToken(format!(
                            "Expected {:?}, found {:?}, at {pos}",
                            expected,
                            tokens.get(pos),
                        )));
                    }
                }
                Symbol::NonTerminal(nt) => {
                    if let Some(rule) = ParsingRule::find_rule(
                        table,
                        &nt,
                        match tokens.get(pos) {
                            Some(x) => x,
                            None => continue,
                        },
                    ) {
                        rule.production
                            .iter()
                            .rev()
                            .filter(|symbol| **symbol != Symbol::Terminal(Terminal::Epsilon))
                            .for_each(|symbol| {
                                stack.push(symbol.clone());
                            });
                        raw_productions.push((nt, rule.production.to_vec()));
                    } else {
                        return Err(SyntaxError::NoRule(format!(
                            "No rule for NonTerminal {:?} with token {:?} at position {pos}",
                            nt,
                            tokens.get(pos)
                        )));
                    }
                }
            }
        }

        if pos <= tokens.len() {
            Ok(AST(raw_productions))
        } else {
            Err(SyntaxError::UnconsumedInput(format!(
                "Unconsumed input at position {}",
                pos
            )))
        }
    }
}

fn update_production_with_token_value(
    token: &Token,
    expected: &Terminal,
    raw_productions: &mut [(NonTerminal, Vec<Symbol>)],
) {
    if let Some(last_production) = raw_productions.last_mut() {
        match token {
            Token::Identifier(identifier) => {
                if let Terminal::DataType(_) = expected {
                    update_symbols_in_production(
                        &mut last_production.1,
                        |symbol| matches!(symbol, Symbol::Terminal(Terminal::DataType(_))),
                        |_| {
                            Symbol::Terminal(Terminal::DataType(DataType::Identifier(
                                identifier.clone(),
                            )))
                        },
                    );
                } else {
                    update_symbols_in_production(
                        &mut last_production.1,
                        |symbol| {
                            matches!(
                                symbol,
                                Symbol::Terminal(Terminal::Token(Token::Identifier(_)))
                            )
                        },
                        |_| {
                            Symbol::Terminal(Terminal::Token(Token::Identifier(identifier.clone())))
                        },
                    );
                }
            }
            Token::Literal(lit) => {
                update_symbols_in_production(
                    &mut last_production.1,
                    |symbol| matches!(symbol, Symbol::Terminal(Terminal::Token(Token::Literal(_)))),
                    |_| Symbol::Terminal(Terminal::Token(Token::Literal(lit.clone()))),
                );
            }
            Token::ReservedWord(word)
                if ParsingRule::is_data_type(&Token::ReservedWord(word.clone())) =>
            {
                if let Ok(data_type) = DataType::try_from(word.clone()) {
                    update_symbols_in_production(
                        &mut last_production.1,
                        |symbol| matches!(symbol, Symbol::Terminal(Terminal::DataType(_))),
                        |_| Symbol::Terminal(Terminal::DataType(data_type.clone())),
                    );
                }
            }
            Token::Operator(operator) => {
                if let Terminal::UnaryOperator(_) = expected {
                    update_symbols_in_production(
                        &mut last_production.1,
                        |symbol| matches!(symbol, Symbol::Terminal(Terminal::UnaryOperator(_))),
                        |_| Symbol::Terminal(Terminal::UnaryOperator(operator.to_owned())),
                    );
                } else if let Terminal::ReassignOp = expected {
                    update_symbols_in_production(
                        &mut last_production.1,
                        |symbol| matches!(symbol, Symbol::Terminal(Terminal::ReassignOp)),
                        |_| Symbol::Terminal(Terminal::ReassignOp),
                    );
                }
            }
            _ => {}
        }
    }
}

fn update_symbols_in_production<F, G>(symbols: &mut [Symbol], predicate: F, transformer: G)
where
    F: Fn(&Symbol) -> bool,
    G: Fn(&Symbol) -> Symbol,
{
    for symbol in symbols.iter_mut() {
        if predicate(symbol) {
            *symbol = transformer(symbol);
        }
    }
}
