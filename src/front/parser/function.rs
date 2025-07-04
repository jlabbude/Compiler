use crate::front::lexer::reserved::{Operator, ReservedWord, Separator};
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::{id, literal, typed, unary_op, NonTerminal, Parser, ParsingRule, Symbol, Terminal};

// todo arraydecl

pub struct Function;

impl Parser for Function {
    /// <Func> :: func <DataType> id ( <FuncArgument> ) { <FuncBody> } <S>
    /// <FuncArgument> :: <DataType> id <FuncArgument> | , <DataType> id <FuncArgument> | e
    /// <FuncBody> :: <StmntList> | ε
    /// <StmntList> :: <Statement> <StmntList> | ε
    /// <Statement> :: return <Expr> ;
    ///             | if ( <Expr> ) { <StmntList> } <StmntElse>
    ///             | match ( <Expr> ) { <StmntCase> }
    ///             | for ( <StmntDecl> ; <Expr> ; <StmntAssign> ) { <StmntList> }
    ///             | while ( <Expr> ) { <StmntList> }
    ///             | <StmntDecl> ;
    ///             | <StmntAssign> ;
    /// <StmntCase> :: case <Literal> { <StmntList> } <StmntCase> | default { <StmntList> } | ε
    /// <Expr> :: <ExprOperand> <ExprOperation>
    /// <ExprOperand> :: id <ExprCall> | literal
    /// <ExprCall> :: <ExprFuncCall> <ExprCall>
    ///             | <ExprArrayAccess> <ExprCall>
    ///             | <ExprFieldAccess> <ExprCall>
    ///             | id <ExprCall>
    ///             | ε
    /// <ExprArrayAccess> :: [ <Expr> ] | ε
    /// <ExprFieldAccess> :: . | ε
    /// <ExprFuncCall> :: ( <ExprFuncCallArgs> ) | ε
    /// <ExprFuncCallArgs> :: <Expr> <ExprFuncCallArgs> | , <Expr> <ExprFuncCallArgs> | ε
    /// <ExprOperation> :: <UnaryOperator> <Expr> <ExprOperation> | ε
    /// <StmntElse> :: elif ( <Expr> ) { <StmntList> } <StmntElse>
    ///              | else { <StmntList> }
    ///              | ε
    /// <StmntDecl> :: let <DataType> id = <Expr> | const <DataType> id = <Expr>
    /// <StmntAssign> :: id = <ExprCall> <ReassignOp> <Expr>
    const PARSING_TABLE: &'static [ParsingRule<'_>] = &[
        /*
            <Func> :: func <DataType> id ( <FuncArgument> ) { <FuncBody> } <S>
        */
        ParsingRule {
            non_terminal: NonTerminal::Func,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Function)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Function))),
                Symbol::Terminal(typed),
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
            token: typed,
            production: &[
                Symbol::Terminal(typed),
                Symbol::Terminal(id),
                Symbol::NonTerminal(NonTerminal::FuncArgument),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::FuncArgument,
            token: Terminal::Token(Token::Separator(Separator::Comma)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                Symbol::Terminal(typed),
                Symbol::Terminal(id),
                Symbol::NonTerminal(NonTerminal::FuncArgument),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::FuncArgument,
            token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        /*

            <FuncBody> :: <StmntList> | e

        */
        ParsingRule {
            non_terminal: NonTerminal::FuncBody,
            token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        ParsingRule {
            non_terminal: NonTerminal::FuncBody,
            token: Terminal::Any,
            production: &[Symbol::NonTerminal(NonTerminal::StmntList)],
        },
        /*

            <StmntList> :: <Statement> <StmntList> | e

        */
        ParsingRule {
            non_terminal: NonTerminal::StmntList,
            token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        ParsingRule {
            non_terminal: NonTerminal::StmntList,
            token: Terminal::Any,
            production: &[
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
            production: &[
                Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Return))),
                Symbol::NonTerminal(NonTerminal::Expr),
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::Statement,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::If)),
            production: &[
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
            production: &[
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
            production: &[
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
            production: &[
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
            production: &[
                Symbol::NonTerminal(NonTerminal::StmntDecl),
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::Statement,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Constant)),
            production: &[
                Symbol::NonTerminal(NonTerminal::StmntDecl),
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Terminator))),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::Statement,
            token: id,
            production: &[
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
            production: &[
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
            production: &[
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
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        /*

            <Expr> :: <ExprOperand> <ExprOperation>

        */
        ParsingRule {
            non_terminal: NonTerminal::Expr,
            token: Terminal::Any,
            production: &[
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
            production: &[
                Symbol::Terminal(id),
                Symbol::NonTerminal(NonTerminal::ExprCall),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprOperand,
            token: literal,
            production: &[Symbol::Terminal(literal)],
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
            production: &[
                Symbol::NonTerminal(NonTerminal::ExprFuncCall),
                Symbol::NonTerminal(NonTerminal::ExprCall),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprCall,
            token: Terminal::Token(Token::Separator(Separator::OpenBrackets)),
            production: &[
                Symbol::NonTerminal(NonTerminal::ExprArrayAccess),
                Symbol::NonTerminal(NonTerminal::ExprCall),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprCall,
            token: Terminal::Token(Token::Separator(Separator::Dot)),
            production: &[
                Symbol::NonTerminal(NonTerminal::ExprFieldAccess),
                Symbol::NonTerminal(NonTerminal::ExprCall),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprCall,
            token: id,
            production: &[
                Symbol::Terminal(id),
                Symbol::NonTerminal(NonTerminal::ExprCall),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprCall,
            token: Terminal::Any,
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        /*

            <ExprArrayAccess> :: [ <Expr> ] | e

        */
        ParsingRule {
            non_terminal: NonTerminal::ExprArrayAccess,
            token: Terminal::Token(Token::Separator(Separator::OpenBrackets)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::OpenBrackets))),
                Symbol::NonTerminal(NonTerminal::Expr),
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::CloseBrackets))),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprArrayAccess,
            token: Terminal::Any,
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        /*

            <ExprFieldAccess> :: . | e

        */
        ParsingRule {
            non_terminal: NonTerminal::ExprFieldAccess,
            token: Terminal::Token(Token::Separator(Separator::Dot)),
            production: &[Symbol::Terminal(Terminal::Token(Token::Separator(
                Separator::Dot,
            )))],
        },
        /*

            <ExprFuncCall> :: ( <ExprFuncCallArgs> ) | e

        */
        ParsingRule {
            non_terminal: NonTerminal::ExprFuncCall,
            token: Terminal::Token(Token::Separator(Separator::OpenParenthesis)),
            production: &[
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
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        /*

            <ExprFuncCallArgs> :: <Expr> <ExprFuncCallArgs> | , <Expr> <ExprFuncCallArgs> | e

        */
        ParsingRule {
            non_terminal: NonTerminal::ExprFuncCallArgs,
            token: id,
            production: &[
                Symbol::NonTerminal(NonTerminal::Expr),
                Symbol::NonTerminal(NonTerminal::ExprFuncCallArgs),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprFuncCallArgs,
            token: literal,
            production: &[
                Symbol::NonTerminal(NonTerminal::Expr),
                Symbol::NonTerminal(NonTerminal::ExprFuncCallArgs),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprFuncCallArgs,
            token: Terminal::Token(Token::Separator(Separator::Comma)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma))),
                Symbol::NonTerminal(NonTerminal::Expr),
                Symbol::NonTerminal(NonTerminal::ExprFuncCallArgs),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprFuncCallArgs,
            token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        /*

            <ExprOperation> :: <UnaryOperator> <Expr> <ExprOperation> | e

        */
        ParsingRule {
            non_terminal: NonTerminal::ExprOperation,
            token: unary_op,
            production: &[
                Symbol::Terminal(unary_op),
                Symbol::NonTerminal(NonTerminal::Expr),
                Symbol::NonTerminal(NonTerminal::ExprOperation),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprOperation,
            token: Terminal::Token(Token::Separator(Separator::Terminator)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprOperation,
            token: Terminal::Token(Token::Separator(Separator::CloseCurlyBraces)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprOperation,
            token: Terminal::Token(Token::Separator(Separator::CloseParenthesis)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprOperation,
            token: Terminal::Token(Token::Separator(Separator::CloseBrackets)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        ParsingRule {
            non_terminal: NonTerminal::ExprOperation,
            token: Terminal::Token(Token::Separator(Separator::Comma)),
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        /*

            <StmntElse> :: elif ( <Expr> ) { <StmntList> } <StmntElse>
                        | else { <StmntList> }
                        | e

        */
        ParsingRule {
            non_terminal: NonTerminal::StmntElse,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Elif)),
            production: &[
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
            production: &[
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
            production: &[Symbol::Terminal(Terminal::Epsilon)],
        },
        /*

            <StmntDecl> :: let <DataType> id = <Expr>
                        | const <DataType> id = <Expr>

        */
        ParsingRule {
            non_terminal: NonTerminal::StmntDecl,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Let)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Let))),
                Symbol::Terminal(typed),
                Symbol::Terminal(id),
                Symbol::Terminal(Terminal::Token(Token::Operator(Operator::Assignment))),
                Symbol::NonTerminal(NonTerminal::Expr),
            ],
        },
        ParsingRule {
            non_terminal: NonTerminal::StmntDecl,
            token: Terminal::Token(Token::ReservedWord(ReservedWord::Constant)),
            production: &[
                Symbol::Terminal(Terminal::Token(Token::ReservedWord(ReservedWord::Constant))),
                Symbol::Terminal(typed),
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
            production: &[
                Symbol::Terminal(id),
                Symbol::NonTerminal(NonTerminal::ExprCall),
                Symbol::Terminal(Terminal::ReassignOp),
                Symbol::NonTerminal(NonTerminal::Expr),
            ],
        },
    ];
}
