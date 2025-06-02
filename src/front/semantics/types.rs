use crate::front::lexer::reserved::Separator;
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::{DataType, NonTerminal, Symbol, Terminal, AST};
use crate::front::semantics::semantic_errors::SemanticError;

pub type TypeTable = Vec<TypeCell>;
pub type IdentifierTypeTable = Vec<IdentifierTypeCell>;

#[derive(Debug)]
pub struct TypeCell {
    identifier: String,
    data_type: DataType,
}

#[derive(Debug)]
pub enum IdentifierTypeCell {
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
        let identifiers_as_types = self.get_valid_identifiers_as_types()?;
        self.validate_identifiers_as_types_usage(table, identifiers_as_types)?;
        Ok(())
    }

    fn not_epsilon(prod: &Vec<Symbol>) -> bool {
        prod.first()
            .map_or(false, |s| !matches!(s, Symbol::Terminal(Terminal::Epsilon)))
    }

    fn get_identifier_in_production(production: &Vec<Symbol>) -> Option<String> {
        production.iter().find_map(|symbol| {
            if let Symbol::Terminal(Terminal::Token(Token::Identifier(identifier))) = symbol {
                Some(identifier.clone())
            } else {
                None
            }
        })
    }

    fn build_type_table(&self) -> Result<TypeTable, SemanticError> {
        let mut ast_vec = self.as_vec().iter();
        let mut table = Vec::new();
        while let Some((non_terminal, production)) = ast_vec.next() {
            match non_terminal {
                NonTerminal::StmntDecl => {
                    let var_name = match Self::get_identifier_in_production(production) {
                        Some(name) => name,
                        None => continue,
                    };

                    let data_type = match production.iter().find_map(|symbol| {
                        if let Symbol::Terminal(Terminal::DataType(dt)) = symbol {
                            Some(dt.clone())
                        } else {
                            None
                        }
                    }) {
                        Some(dt) => dt,
                        None => continue,
                    };

                    ast_vec
                        .by_ref()
                        .take_while(|(nt, prod)| Self::not_epsilon(prod))
                        .for_each(|(nt, prod)| {
                            match nt {
                                NonTerminal::ExprOperand => {

                                }
                                _ => return,
                            }
                        });
                    table.push(TypeCell {
                        identifier: var_name,
                        data_type,
                    })
                }
                // NonTerminal::
                _ => continue,
            }
        }
        Ok(table)
    }

    fn get_valid_identifiers_as_types(&self) -> Result<IdentifierTypeTable, SemanticError> {
            let mut ast_vec = self.as_vec().iter();
            let mut valid_types: Vec<IdentifierTypeCell> = Vec::new();
            while let Some((non_terminal, production)) = ast_vec.next() {
                match non_terminal {
                    NonTerminal::Enum => {
                        let variants: Vec<String> = ast_vec
                            .by_ref()
                            .take_while(|(nt, prod)| {
                                nt == &NonTerminal::EnumBody && Self::not_epsilon(prod)
                            })
                            .filter_map(|(_, prod)| match prod.first()? {
                                Symbol::Terminal(Terminal::Token(Token::Separator(
                                                                     Separator::Comma,
                                                                 ))) => None,
                                Symbol::Terminal(Terminal::Token(Token::Identifier(variant))) => {
                                    Some(variant.clone())
                                }
                                _ => panic!("Unexpected symbol in enum body: {:?}", prod.first()),
                            })
                            .collect();
                        valid_types.push(IdentifierTypeCell::EnumType {
                            identifier: Self::get_identifier_in_production(production).unwrap(),
                            variants,
                        });
                    }
                    NonTerminal::Struct => {
                        // No id in prod returns syntax error, thus, unwrapping makes more sense.
                        let identifier = Self::get_identifier_in_production(production).unwrap();
                        let fields: TypeTable = ast_vec
                            .by_ref()
                            .take_while(|(nt, prod)| {
                                nt == &NonTerminal::StructBody && Self::not_epsilon(prod)
                            })
                            .filter_map(|(_, prod)| {
                                let mut prod = prod.iter();
                                match prod.next()? {
                                    Symbol::Terminal(Terminal::DataType(dt)) => Some(TypeCell {
                                        identifier: identifier.clone(),
                                        data_type: dt.clone(),
                                    }),
                                    _ => None,
                                }
                            })
                            .collect();
                        valid_types.push(IdentifierTypeCell::StructType { identifier, fields })
                    }
                    _ => {}
                }
            }
            Ok(valid_types)
        }

        fn validate_identifiers_as_types_usage(
            &self,
            type_table: TypeTable,
            valid_id_types: IdentifierTypeTable,
        ) -> Result<(), SemanticError> {
            type_table
                .iter()
                .filter_map(|cell| match &cell.data_type {
                    DataType::Identifier(identifier) => Some(identifier),
                    _ => None,
                })
                .find(|&identifier| {
                    !valid_id_types
                        .iter()
                        .map(|valid_t| match valid_t {
                            IdentifierTypeCell::StructType { identifier, .. }
                            | IdentifierTypeCell::EnumType { identifier, .. } => identifier,
                        })
                        .collect::<Vec<&String>>()
                        .contains(&identifier)
                })
                .map_or(Ok(()), |invalid_id| {
                    Err(SemanticError::UndeclaredType(invalid_id.clone()))
                })
        }
    }
