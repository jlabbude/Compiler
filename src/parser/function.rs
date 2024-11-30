#![allow(non_upper_case_globals)]
use crate::lexer::reserved::{Operator, ReservedWord, Separator};
use crate::lexer::tokens::{Literal, Token};
use crate::parser::grammar::{Grammar, NonTerminal, Parser, ParsingRule, Symbol, Terminal};

// todo arraydecl
// todo multidimensional arrayaccess
// todo structdecl
// todo enumdecl

// These are just a hacky way of bypassing the parser by creating "id" and "literal" terminals
// that are always valid because of matches_token() on grammar.rs
const id: Terminal = Terminal::Token(Token::Identifier(String::new()));
const literal: Terminal = Terminal::Token(Token::Literal(Literal::Int(0)));

pub struct Function;

impl Parser for Function {
    /// <Func> :: func <DataType> id ( <FuncArgument> ) { <FuncBody> } <S>
    /// <FuncArgument> :: <DataType> id <FuncArgument> | , <DataType> id <FuncArgument> | e
    /// <FuncBody> :: <StmntList> | e
    /// <StmntList> :: <Statement> <StmntList> | e
    /// <Statement> :: return <Expr> ;
    ///             | if ( <Expr> ) { <StmntList> } <StmntElse>
    ///             | match ( <Expr> ) { <StmntCase> }
    ///             | for ( <StmntDecl> ; <Expr> ; <StmntAssign> ) { <StmntList> }
    ///             | while ( <Expr> ) { <StmntList> }
    ///             | <StmntDecl> ;
    ///             | <StmntAssign> ;
    /// <StmntCase> :: case <Literal> { <StmntList> } <StmntCase> | default { <StmntList> } | e
    /// <Expr> :: <ExprOperand> <ExprOperation>
    /// <ExprOperand> :: id <ExprCall> | literal
    /// <ExprCall> :: <ExprFuncCall> <ExprCall>
    ///             | <ExprArrayAccess> <ExprCall>
    ///             | <ExprFieldAccess> <ExprCall>
    ///             | id <ExprCall>
    ///             | e
    /// <ExprArrayAccess> :: [ <Expr> ] | e
    /// <ExprFieldAccess> :: . | e
    /// <ExprFuncCall> :: ( <ExprFuncCallArgs> ) | e
    /// <ExprFuncCallArgs> :: <Expr> <ExprFuncCallArgs> | , <Expr> <ExprFuncCallArgs> | e
    /// <ExprOperation> :: <UnaryOperator> <Expr> <ExprOperation> | e
    /// <StmntElse> :: elif ( <Expr> ) { <StmntList> } <StmntElse>
    ///              | else { <StmntList> }
    ///              | e
    /// <StmntDecl> :: let <DataType> id = <Expr> | const <DataType> id = <Expr>
    /// <StmntAssign> :: id = <ExprCall> <ReassignOp> <Expr>
    fn parsing_table() -> Grammar {
        vec![
            /*
                <Func> :: func <DataType> id ( <FuncArgument> ) { <FuncBody> } <S>
            */
            ParsingRule {
                non_terminal: NonTerminal::Func,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Function)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Function))),
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::FuncBody),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Program),
                ],
            },
            /*

                <FuncArgument> :: <DataType> id <FuncArgument> | , <DataType> id <FuncArgument> | e

            */
            ParsingRule {
                non_terminal: NonTerminal::FuncArgument,
                token: Terminal::DataType,
                production: vec![
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgument,
                token: Terminal::Token(Token::Separator(Separator::Comma)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::FuncArgument),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncArgument,
                token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            /*

                <FuncBody> :: <StmntList> | e

            */
            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::FuncBody,
                token: Terminal::Any,
                production: vec![Symbol::NonTerminal(NonTerminal::StmntList)],
            },
            /*

                <StmntList> :: <Statement> <StmntList> | e

            */
            ParsingRule {
                non_terminal: NonTerminal::StmntList,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::StmntList,
                token: Terminal::Any,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::Statement),
                    Symbol::NonTerminal(NonTerminal::StmntList),
                ],
            },
            /*

                <Statement> ::
                    return <Expr> ;
                        | if ( <Expr> ) { <StmntList> } <StmntElse>
                        | match ( <Expr> ) { <StmntCase> }
                        | for ( <StmntDecl> ; <Expr> ; <StmntAssign> ) { <StmntList> }
                        | while ( <Expr> ) { <StmntList> }
                        | <StmntDecl> ;
                        | <StmntAssign> ;

            */
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Return)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Return))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::If)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::If))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntList),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntElse),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Match)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Match))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntCase),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::For)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::For))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntDecl),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                    Symbol::NonTerminal(NonTerminal::StmntAssign),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntList),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::While)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::While))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntList),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Let)),
                production: vec![
                    Symbol::NonTerminal(NonTerminal::StmntDecl),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Constant)),
                production: vec![
                    Symbol::NonTerminal(NonTerminal::StmntDecl),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::Statement,
                token: id,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::StmntAssign),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
                ],
            },
            /*

               <StmntCase> :: case <Literal> { <StmntList> } <StmntCase>
                            | default { <StmntList> }
                            | e

            */
            ParsingRule {
                non_terminal: NonTerminal::StmntCase,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Case)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Case))),
                    Symbol::Terminal(literal),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntList),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntCase),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StmntCase,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Default)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Default))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntList),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StmntCase,
                token: Terminal::Any,
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            /*

                <Expr> :: <ExprOperand> <ExprOperation>

            */
            ParsingRule {
                non_terminal: NonTerminal::Expr,
                token: Terminal::Any,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::ExprOperand),
                    Symbol::NonTerminal(NonTerminal::ExprOperation),
                ],
            },
            /*

                <ExprOperand> :: id <ExprCall> | literal

            */
            ParsingRule {
                non_terminal: NonTerminal::ExprOperand,
                token: id,
                production: vec![
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::ExprCall),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprOperand,
                token: literal,
                production: vec![Symbol::Terminal(literal)],
            },
            /*

                <ExprCall> ::
                    <ExprFuncCall> <ExprCall>
                        | <ExprArrayAccess> <ExprCall>
                        | <ExprFieldAccess> <ExprCall>
                        | id <ExprCall>
                        | e

            */
            ParsingRule {
                non_terminal: NonTerminal::ExprCall,
                token: Terminal::Token(Token::Separator(Separator::OpenParenthesis)),
                production: vec![
                    Symbol::NonTerminal(NonTerminal::ExprFuncCall),
                    Symbol::NonTerminal(NonTerminal::ExprCall),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprCall,
                token: Terminal::Token(Token::Separator(Separator::OpenBrackets)),
                production: vec![
                    Symbol::NonTerminal(NonTerminal::ExprArrayAccess),
                    Symbol::NonTerminal(NonTerminal::ExprCall)
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprCall,
                token: Terminal::Token(Token::Separator(Separator::Dot)),
                production: vec![
                    Symbol::NonTerminal(NonTerminal::ExprFieldAccess),
                    Symbol::NonTerminal(NonTerminal::ExprCall),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprCall,
                token: id,
                production: vec![
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::ExprCall)
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprCall,
                token: Terminal::Any,
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            /*

                <ExprArrayAccess> :: [ <Expr> ] | e

            */
            ParsingRule {
                non_terminal: NonTerminal::ExprArrayAccess,
                token: Terminal::Token(Token::Separator(Separator::OpenBrackets)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::OpenBrackets))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::CloseBrackets))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprArrayAccess,
                token: Terminal::Any,
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            /*

                <ExprFieldAccess> :: . | e

            */
            ParsingRule {
                non_terminal: NonTerminal::ExprFieldAccess,
                token: Terminal::Token(Token::Separator(Separator::Dot)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Dot))),
                ],
            },
            /*

                <ExprFuncCall> :: ( <ExprFuncCallArgs> ) | e

            */
            ParsingRule {
                non_terminal: NonTerminal::ExprFuncCall,
                token: Terminal::Token(Token::Separator(Separator::OpenParenthesis)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::ExprFuncCallArgs),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprFuncCall,
                token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            /*

                <ExprFuncCallArgs> :: <Expr> <ExprFuncCallArgs> | , <Expr> <ExprFuncCallArgs> | e

            */
            ParsingRule {
                non_terminal: NonTerminal::ExprFuncCallArgs,
                token: id,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::NonTerminal(NonTerminal::ExprFuncCallArgs),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprFuncCallArgs,
                token: literal,
                production: vec![
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::NonTerminal(NonTerminal::ExprFuncCallArgs),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprFuncCallArgs,
                token: Terminal::Token(Token::Separator(Separator::Comma)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::NonTerminal(NonTerminal::ExprFuncCallArgs),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprFuncCallArgs,
                token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            /*

                <ExprOperation> :: <UnaryOperator> <Expr> <ExprOperation> | e

            */
            ParsingRule {
                non_terminal: NonTerminal::ExprOperation,
                token: Terminal::UnaryOperator,
                production: vec![
                    Symbol::Terminal(Terminal::UnaryOperator),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::NonTerminal(NonTerminal::ExprOperation),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprOperation,
                token: Terminal::Token(Token::Separator(Separator::Terminator)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprOperation,
                token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprOperation,
                token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprOperation,
                token: Terminal::Token(Token::Separator(Separator::CloseBrackets)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            ParsingRule {
                non_terminal: NonTerminal::ExprOperation,
                token: Terminal::Token(Token::Separator(Separator::Comma)),
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            /*

                <StmntElse> :: elif ( <Expr> ) { <StmntList> } <StmntElse>
                            | else { <StmntList> }
                            | e

            */
            ParsingRule {
                non_terminal: NonTerminal::StmntElse,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Elif)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Elif))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenParenthesis,
                    ))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseParenthesis,
                    ))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntList),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntElse),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StmntElse,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Else)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Else))),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::OpenCurlyBraces,
                    ))),
                    Symbol::NonTerminal(NonTerminal::StmntList),
                    Symbol::Terminal(Terminal::Token(Token::Separator(
                        Separator::CloseCurlyBraces,
                    ))),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StmntElse,
                token: Terminal::Any,
                production: vec![Symbol::Terminal(Terminal::Epsilon)],
            },
            /*

                <StmntDecl> :: let <DataType> id = <Expr>
                            | const <DataType> id = <Expr>

            */
            ParsingRule {
                non_terminal: NonTerminal::StmntDecl,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Let)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Let))),
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::Terminal(Terminal::Token(Token::Operator(Operator::Assignment))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                ],
            },
            ParsingRule {
                non_terminal: NonTerminal::StmntDecl,
                token: Terminal::Token(Token::ReservedWord(ReservedWord::Constant)),
                production: vec![
                    Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Constant))),
                    Symbol::Terminal(Terminal::DataType),
                    Symbol::Terminal(id),
                    Symbol::Terminal(Terminal::Token(Token::Operator(Operator::Assignment))),
                    Symbol::NonTerminal(NonTerminal::Expr),
                ],
            },
            /*

                <StmntAssign> :: id = <ExprCall> <ReassignOp> <Expr>

            */
            ParsingRule {
                non_terminal: NonTerminal::StmntAssign,
                token: id,
                production: vec![
                    Symbol::Terminal(id),
                    Symbol::NonTerminal(NonTerminal::ExprCall),
                    Symbol::Terminal(Terminal::ReassignOp),
                    Symbol::NonTerminal(NonTerminal::Expr),
                ],
            },
        ]
    }
}
