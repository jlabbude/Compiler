use crate::lexer::reserved::{Operator, ReservedWord, Separator};
use crate::lexer::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum NonTerminal {
    Program,
    Struct,
    Enum,

    Func,
    FuncArgument,
    FuncBody,
    FuncTail,

    Statement,
    StatementList,
    StmntAssign,
    StmntDecl,
    StmntReturn,

    Expr,
    ExprOperation,
    ExprLiteral,
    ExprIdentifier,
    ExprFunctionCall,
    ExprArrayAccess,
    ExprStructAccess,
    ExprEnumAccess,
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
    UnaryOp,
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
            Terminal::UnaryOp => {
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
            Terminal::Any => true,
            Terminal::Epsilon => false,
        }
    }

    pub(crate) fn parse_with_table<'a>(
        tokens: &'a [Token],
        table: &[ParsingRule],
    ) -> Result<&'a [Token], String> {
        let mut stack = vec![Symbol::NonTerminal(NonTerminal::Program)];
        let mut pos = 0;
        let start_pos = pos;

        while let Some(top) = stack.pop() {
            match top {
                Symbol::Terminal(expected) => {
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
                        return Err(format!(
                            "Expected {:?}, found {:?}",
                            expected,
                            tokens.get(pos)
                        ));
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
                    } else {
                        return Err(format!(
                            "No rule for NonTerminal {:?} with token {:?}",
                            nt,
                            tokens.get(pos)
                        ));
                    }
                }
            }
        }

        if pos <= tokens.len() {
            Ok(&tokens[start_pos..pos])
        } else {
            Err(format!("Unconsumed input at position {}", pos))
        }
    }
}
