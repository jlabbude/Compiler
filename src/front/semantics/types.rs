use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::{DataType, NonTerminal, Symbol, Terminal, AST};
use crate::front::semantics::semantic_errors::SemanticError;

#[derive(Debug)]
struct TypeCell {
    identifier: String,
    data_type: DataType,
}

type TypeTable = Vec<TypeCell>;

pub fn typer(ast: &AST) -> Result<(), SemanticError> {
    let mut table = TypeTable::new();
    for (non_terminal, production) in ast {
        match non_terminal {
            NonTerminal::StmntDecl => {
                let var_name = production
                    .iter()
                    .find_map(|symbol| {
                        if let Symbol::Terminal(Terminal::Token(Token::Identifier(identifier))) = symbol {
                            Some(identifier)
                        } else {
                            None
                        }
                    })
                    .ok_or(SemanticError::MissingIdentifier)?;
                let data_type = production
                    .iter()
                    .find_map(|symbol| {
                        if let Symbol::Terminal(Terminal::DataType(dt)) = symbol {
                            Some(dt)
                        } else {
                            None
                        }
                    })
                    .ok_or(SemanticError::MissingDataType)?;
                table.push(TypeCell {
                    identifier: var_name.clone(),
                    data_type: data_type.clone(),
                })
            }
            _ => {}
        }
    }
    
    println!("{:?}", table);
    Ok(())
}
