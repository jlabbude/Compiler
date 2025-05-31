#![allow(non_upper_case_globals)]

use crate::front::lexer::reserved::{Operator, ReservedWord, Separator};
use crate::front::lexer::tokens::{Literal, Token};
use strum_macros::Display;

pub const id: Terminal = Terminal::Token(Token::Identifier(String::new()));
pub const literal: Terminal = Terminal::Token(Token::Literal(Literal::Int(0)));

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

    Func,
    FuncArgument,
    FuncBody,
    FuncTail,

    Statement,
    StmntList,
    StmntAssign,
    StmntDecl,
    StmntReturn,

    Expr,
    ExprOperation,
    ExprLiteral,
    ExprIdentifier,
    ExprFuncCall,
    ExprArrayAccess,
    ExprFieldAccess,
    ExprOperand,
    ExprFuncCallArgs,
    StmntElse,
    StmntCase,
    ExprCall,
    EnumBody,
    StructBody,
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
    DataType,
    UnaryOperator,
    ReassignOp,
    Any,
    Epsilon,
}

pub struct ParsingRule {
    pub non_terminal: NonTerminal,
    pub token: Terminal,
    pub production: Vec<Symbol>,
}

pub type Grammar = Vec<ParsingRule>;

pub trait Parser {
    fn parsing_table() -> Grammar;
}

impl ParsingRule {
    fn find_rule<'a>(
        table: &'a [ParsingRule],
        non_terminal: &NonTerminal,
        token: &Token,
    ) -> Option<&'a ParsingRule> {
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
            Terminal::DataType => ParsingRule::is_data_type(actual),
            Terminal::UnaryOperator => {
                if let Token::Operator(op) = actual {
                    matches!(
                        op,
                        Operator::Increment
                            | Operator::GreaterThanOrEqual
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
    ) -> Result<Vec<(NonTerminal, Vec<Symbol>)>, SyntaxError> {
        let mut stack = vec![Symbol::NonTerminal(NonTerminal::Program)];
        let mut pos = 0;
        let mut raw_productions: Vec<(NonTerminal, Vec<Symbol>)> = Vec::new();

        while let Some(top) = stack.pop() {
            match top {
                Symbol::Terminal(expected) => {
                    if let Token::Identifier(identifier) = tokens.get(pos).unwrap() {
                        let len = (*raw_productions).len() - 1;
                        raw_productions[len].1.iter_mut().for_each(|symbol| {
                            if let Symbol::Terminal(Terminal::Token(Token::Identifier(_))) = symbol
                            {
                                *symbol = Symbol::Terminal(Terminal::Token(Token::Identifier(
                                    identifier.clone(),
                                )));
                            }
                        });
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
                        let local_tokens = rule.production.clone();
                        rule.production
                            .iter()
                            .rev()
                            .filter(|symbol| **symbol != Symbol::Terminal(Terminal::Epsilon))
                            .for_each(|symbol| {
                                stack.push(symbol.clone());
                            });
                        raw_productions.push((nt, local_tokens));
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
            Ok(raw_productions)
        } else {
            Err(SyntaxError::UnconsumedInput(format!(
                "Unconsumed input at position {}",
                pos
            )))
        }
    }
}

#[deprecated]
fn join_rules(raw_productions: Vec<(NonTerminal, Vec<Symbol>)>) -> Vec<(NonTerminal, Vec<Symbol>)> {
    fn expand_non_terminal(
        nt: &NonTerminal,
        productions: &[(NonTerminal, Vec<Symbol>)],
        visited: &mut Vec<NonTerminal>,
    ) -> Vec<Symbol> {
        if visited.contains(nt) {
            return vec![Symbol::NonTerminal(nt.clone())];
        }
        visited.push(nt.clone());
        if let Some((_, production)) = productions.iter().find(|(lhs, _)| lhs == nt) {
            let mut expanded = Vec::new();
            for symbol in production {
                match symbol {
                    Symbol::NonTerminal(inner_nt) => {
                        expanded.extend(expand_non_terminal(inner_nt, productions, visited));
                    }
                    other => expanded.push(other.clone()),
                }
            }
            visited.pop();
            expanded
        } else {
            panic!("Unmatched NonTerminal: {:?}", nt);
        }
    }

    let mut final_productions = Vec::new();

    for (nt, production) in &raw_productions {
        let mut expanded = Vec::new();
        let mut visited = Vec::new(); // Track visited nodes to prevent infinite recursion
        for symbol in production {
            match symbol {
                Symbol::NonTerminal(inner_nt) => {
                    expanded.extend(expand_non_terminal(
                        inner_nt,
                        &raw_productions,
                        &mut visited,
                    ));
                }
                other => expanded.push(other.clone()),
            }
        }
        final_productions.push((nt.clone(), expanded));
    }

    final_productions
        .iter()
        .map(|(nt, production)| {
            (
                nt.clone(),
                vec![Symbol::NonTerminal(nt.clone())]
                    .into_iter()
                    .chain(production.clone())
                    .collect(),
            )
        })
        .collect()
}
