use crate::front::lexer::reserved::Separator;
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::{DataType, NonTerminal, Symbol, Terminal, AST};
use crate::front::semantics::semantic_errors::SemanticError;

#[derive(Debug)]
pub struct TypeCell {
    identifier: String,
    data_type: DataType,
}

pub type TypeTable = Vec<TypeCell>;

#[derive(Debug)]
pub enum IdentifierTypeTable {
    StructType {
        identifier: String,
        fields: TypeTable,
    },
    EnumType {
        identifier: String,
        variants: Vec<String>,
    },
}

impl AST {
    pub fn new(raw_ast: Vec<(NonTerminal, Vec<Symbol>)>) -> Self {
        Self(raw_ast)
    }
    pub fn as_vec(&self) -> &Vec<(NonTerminal, Vec<Symbol>)> {
        &self.0
    }

    pub fn type_check(self) -> Result<(), SemanticError> {
        let table = self.build_type_table()?;
        println!("Type table:");
        table.iter().for_each(|cell| println!("{cell:?}"));
        println!("Identifiers as types:");
        let identifiers_as_types = self.get_valid_identifiers_as_types()?;
        identifiers_as_types.iter().for_each(|cell| println!("{cell:?}"));
        Ok(())
    }

    fn build_type_table(&self) -> Result<TypeTable, SemanticError> {
        self.as_vec()
            .iter()
            .filter_map(|(non_terminal, production)| {
                if let NonTerminal::StmntDecl = non_terminal {
                    let var_name = production.iter().find_map(|symbol| {
                        if let Symbol::Terminal(Terminal::Token(Token::Identifier(identifier))) =
                            symbol
                        {
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
            .collect()
    }

    fn get_valid_identifiers_as_types(&self) -> Result<Vec<IdentifierTypeTable>, SemanticError> {
        let mut peekable_vec = self.as_vec().iter().peekable();
        let mut valid_types: Vec<IdentifierTypeTable> = Vec::new();
        while let Some((non_terminal, production)) = peekable_vec.next() {
            match non_terminal {
                NonTerminal::Enum => {
                    let identifier = production
                        .iter()
                        .find_map(|symbol| {
                            if let Symbol::Terminal(Terminal::Token(Token::Identifier(id))) = symbol
                            {
                                Some(id.clone())
                            } else {
                                None
                            }
                        })
                        .ok_or(SemanticError::InvalidType)?; //fixme
                    let mut variants = Vec::new();
                    while let Some((next_nt, next_prod)) = peekable_vec.next() {
                        if next_nt == &NonTerminal::EnumBody {
                            if let Some(symbol) = next_prod.first() {
                                if symbol == &Symbol::Terminal(Terminal::Epsilon) || 
                                    symbol == &Symbol::Terminal(Terminal::Token(Token::Separator(Separator::Comma)))
                                {
                                    break;
                                } else if let Symbol::Terminal(Terminal::Token(Token::Identifier(variant))) = symbol {
                                    variants.push(variant.clone());
                                } else {
                                    unreachable!()
                                }
                            }
                        }
                    }
                    valid_types.push(IdentifierTypeTable::EnumType {
                        identifier,
                        variants,
                    });
                    return Ok(valid_types);
                }
                NonTerminal::Struct => {}
                _ => {}
            }
        }
        Err(SemanticError::InvalidType) // fixme
    }

    // fn validate_identifiers_as_types_usage(&self, valid_types: Vec<DataType>) -> Result<(), SemanticError> {
    //     for (non_terminal, production) in self.as_vec() {
    //         if let NonTerminal::StmntDecl = non_terminal {
    //             for symbol in production {
    //                 if let Symbol::Terminal(Terminal::DataType(dt)) = symbol {
    //                     if !valid_types.contains(dt) {
    //                         return Err(SemanticError::UndeclaredType(dt.clone()));
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }
}
