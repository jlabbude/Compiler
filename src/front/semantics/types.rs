use crate::front::parser::grammar::{NonTerminal, AST};
use crate::front::semantics::semantic_errors::SemanticError;

pub fn typer(ast: &AST) -> Result<(), SemanticError> {
    ast.iter()
        .for_each(|(non_terminal, _rule)| match non_terminal {
            NonTerminal::StmntDecl => {
                
            }
            _ => {}
        });

    todo!()
}