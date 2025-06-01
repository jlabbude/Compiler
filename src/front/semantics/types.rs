use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::{DataType, NonTerminal, Symbol, Terminal, AST};
use crate::front::semantics::semantic_errors::SemanticError;

#[derive(Debug)]
struct TypeCell {
    identifier: String,
    data_type: DataType,
}

type TypeTable = Vec<TypeCell>;

impl AST {
    pub fn new(raw_ast: Vec<(NonTerminal, Vec<Symbol>)>) -> Self {
        Self(raw_ast)
    }
    pub fn as_vec(&self) -> &Vec<(NonTerminal, Vec<Symbol>)> {
        &self.0
    }

    pub fn type_check(self) -> Result<(), SemanticError> {
        let table = self.build_type_table()?;
        table.iter().for_each(|cell| println!("{cell:?}"));
        Ok(())
    }

    fn build_type_table(&self) -> Result<TypeTable, SemanticError> {
        self.as_vec()
            .iter()
            .filter_map(|(non_terminal, production)| {
                if let NonTerminal::StmntDecl = non_terminal {
                    let var_name = production.iter().find_map(|symbol| {
                        if let Symbol::Terminal(Terminal::Token(Token::Identifier(identifier))) = symbol {
                            Some(identifier.clone())
                        } else {
                            None
                        }
                    })?;

                    let data_type = production.iter().find_map(|symbol| {
                        if let Symbol::Terminal(Terminal::DataType(dt)) = symbol {
                            Some(dt.clone())
                        } else {
                            None
                        }
                    })?;

                    Some(Ok(TypeCell {
                        identifier: var_name,
                        data_type,
                    }))
                } else {
                    None
                }
            })
            .collect::<Result<TypeTable, SemanticError>>()
    }

    fn get_valid_identifiers_as_types(&self) -> Result<TypeTable, SemanticError> {
        let mut table = TypeTable::new();
        for (non_terminal, production) in self.as_vec() {
            if let NonTerminal::StmntDecl = non_terminal {
                for symbol in production {
                    if let Symbol::Terminal(Terminal::Token(Token::Identifier(identifier))) = symbol
                    {
                        table.push(TypeCell {
                            identifier: identifier.clone(),
                            data_type: DataType::Identifier(identifier.clone()),
                        });
                    }
                }
            }
        }
        Ok(table)
    }
}
