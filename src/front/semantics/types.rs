use crate::front::lexer::reserved::{ReservedWord, Separator};
use crate::front::lexer::tokens::Token;
use crate::front::parser::grammar::{DataType, NonTerminal, Symbol, Terminal, AST};
use crate::front::semantics::semantic_errors::SemanticError;

pub type TypeTable = Vec<TypeCell>;
pub type IdentifierTypeTable = Vec<IdentifierTypeCell>;

#[derive(Debug, Clone)]
pub struct TypeCell {
    identifier: String,
    data_type: DataType,
    mutable: bool,
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

    pub fn type_check(self) -> Result<(), Box<SemanticError>> {
        let table = self.build_type_table()?;
        let identifiers_as_types = self.get_valid_identifiers_as_types()?;
        println!("Type table: {{");
        table.iter().for_each(|cell| println!("  {:?}", cell));
        println!("}}\n");
        println!("ID as types: {{");
        identifiers_as_types
            .iter()
            .for_each(|identifier| println!("  {:?}", identifier));
        println!("}}\n");
        self.validate_identifiers_as_types_usage(&table, &identifiers_as_types)?;
        self.validate_declaration_expression(&table, &identifiers_as_types)
    }

    fn not_epsilon(prod: &[Symbol]) -> bool {
        prod.first()
            .is_some_and(|s| !matches!(s, Symbol::Terminal(Terminal::Epsilon)))
    }

    fn get_identifier_in_production(production: &[Symbol]) -> Option<String> {
        production.iter().find_map(|symbol| {
            if let Symbol::Terminal(Terminal::Token(Token::Identifier(identifier))) = symbol {
                Some(identifier.clone())
            } else {
                None
            }
        })
    }

    fn build_type_table(&self) -> Result<TypeTable, Box<SemanticError>> {
        let mut table = Vec::new();
        for (non_terminal, production) in self.as_vec() {
            match non_terminal {
                NonTerminal::StmntDecl => table.push(TypeCell {
                    identifier: match Self::get_identifier_in_production(production) {
                        Some(name) => name,
                        None => continue,
                    },
                    data_type: match production.iter().find_map(|symbol| {
                        if let Symbol::Terminal(Terminal::DataType(dt)) = symbol {
                            Some(dt.clone())
                        } else {
                            None
                        }
                    }) {
                        Some(dt) => dt,
                        None => continue,
                    },
                    mutable: production
                        .iter()
                        .find_map(|symbol| match symbol {
                            Symbol::Terminal(Terminal::Token(Token::ReservedWord(
                                ReservedWord::Let,
                            ))) => Some(true),
                            Symbol::Terminal(Terminal::Token(Token::ReservedWord(
                                ReservedWord::Constant,
                            ))) => Some(false),
                            _ => None,
                        })
                        .unwrap(),
                }),
                // NonTerminal::
                _ => continue,
            }
        }
        Ok(table)
    }
    //fixme check for redeclaring id on variants and fields if variants have same id as struct name, as well as variant repetition
    //fixme check for redeclaring id on struct fields
    fn get_valid_identifiers_as_types(&self) -> Result<IdentifierTypeTable, Box<SemanticError>> {
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
                            let mut prod_iter = prod.iter();
                            match prod_iter.next()? {
                                Symbol::Terminal(Terminal::DataType(dt)) => Some(TypeCell {
                                    identifier: Self::get_identifier_in_production(prod).unwrap(),
                                    data_type: dt.clone(),
                                    mutable: true,
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
        type_table: &TypeTable,
        valid_id_types: &IdentifierTypeTable,
    ) -> Result<(), Box<SemanticError>> {
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
                Err(Box::from(SemanticError::UndeclaredType(invalid_id.clone())))
            })
    }

    fn validate_declaration_expression(
        &self,
        type_table: &TypeTable,
        valid_id_types: &IdentifierTypeTable,
    ) -> Result<(), Box<SemanticError>> {
        let mut ast_vec = self.as_vec().iter().peekable();

        while let Some((nt, prod)) = ast_vec.by_ref().next() {
            match nt {
                NonTerminal::StmntDecl => {
                    let var = Self::find_var_in_table(
                        type_table,
                        &Self::get_identifier_in_production(prod).unwrap(),
                    )
                    .unwrap();

                    let expr_operands: Vec<&(NonTerminal, Vec<Symbol>)> = ast_vec
                        .by_ref()
                        .peekable()
                        .take_while(|(_, prod)| Self::not_epsilon(prod))
                        .collect();

                    for (nt, prod) in expr_operands {
                        let mut prod = prod.iter().peekable();
                        match nt {
                            NonTerminal::ExprOperand => {
                                if let Some(symbol) = prod.next() {
                                    match symbol {
                                        Symbol::Terminal(Terminal::Token(Token::Identifier(
                                            expr_id,
                                        ))) => {
                                            if let DataType::Identifier(_) = var.data_type {
                                                return Self::validate_id_type_expr(
                                                    valid_id_types,
                                                    var,
                                                    expr_id,
                                                );
                                            }
                                            match Self::find_var_in_table(type_table, expr_id) {
                                                Some(expr_var) => {
                                                    if expr_var.data_type != var.data_type {
                                                        return Err(Box::from(
                                                            SemanticError::IncompatibleTypes(
                                                                var.clone(),
                                                                expr_var.clone(),
                                                            ),
                                                        ));
                                                    }
                                                }
                                                None => {
                                                    return Err(Box::from(
                                                        SemanticError::UndeclaredIdentifier(
                                                            expr_id.clone(),
                                                        ),
                                                    ))
                                                }
                                            }
                                        }
                                        Symbol::Terminal(Terminal::Token(Token::Literal(
                                            expr_literal,
                                        ))) => {
                                            if var.data_type
                                                != DataType::from(expr_literal.to_owned())
                                                || matches!(var.data_type, DataType::Identifier(_))
                                            {
                                                return Err(Box::from(
                                                    SemanticError::IncompatibleTypes(
                                                        var.to_owned(),
                                                        TypeCell {
                                                            identifier: "".to_string(),
                                                            data_type: DataType::from(
                                                                expr_literal.to_owned(),
                                                            ),
                                                            mutable: false,
                                                        },
                                                    ),
                                                ));
                                            }
                                            return Ok(());
                                        }
                                        symbol => {
                                            panic!("Unexpected symbol: {:?}", symbol);
                                        }
                                    }
                                }
                            }
                            NonTerminal::ExprOperation => {
                                if let Some(symbol) = prod.next() {
                                    //fixme deal with operator types by refactoring Terminal::UnaryOperator
                                    match symbol {
                                        Symbol::NonTerminal(_) => {}
                                        Symbol::Terminal(_) => {}
                                    }
                                }
                            }
                            NonTerminal::Expr
                            | NonTerminal::StmntList
                            | NonTerminal::ExprCall => continue,
                            symbol => {
                                // Handle other unexpected symbols gracefully
                                panic!("Unexpected symbol: {:?}", symbol);
                            }
                        }
                    }
                }
                NonTerminal::StmntAssign => {}
                _ => continue,
            }
        }
        Ok(())
    }

    fn validate_id_type_expr(
        valid_id_types: &IdentifierTypeTable,
        var: &TypeCell,
        expr_id: &String,
    ) -> Result<(), Box<SemanticError>> {
        for valid_type in valid_id_types {
            match valid_type {
                IdentifierTypeCell::EnumType {
                    identifier,
                    variants,
                } => {
                    if var.data_type == DataType::Identifier(identifier.to_owned()) {
                        if variants.iter().any(|variant| variant == expr_id) {
                            return Ok(());
                        }
                        return Err(Box::from(SemanticError::VariantNotDeclared(
                            var.to_owned(),
                            expr_id.to_owned(),
                            IdentifierTypeCell::EnumType {
                                identifier: identifier.to_owned(),
                                variants: variants.to_owned(),
                            },
                        )));
                    }
                }
                IdentifierTypeCell::StructType { identifier, fields } => {
                    // fixme match case on &var could provide more and better error treatment concisely
                    if identifier == &var.identifier {
                        return Err(Box::from(SemanticError::RedeclaredIdentifier(
                            identifier.clone(),
                            var.identifier.clone(),
                        )));
                    }

                    if identifier == expr_id
                        || fields
                            .iter()
                            .any(|field| field.identifier == var.identifier)
                    {
                        return Err(Box::from(SemanticError::InvalidStructUsage(
                            IdentifierTypeCell::StructType {
                                identifier: identifier.to_owned(),
                                fields: fields.to_owned(),
                            },
                            var.clone(),
                        )));
                    }
                }
            }
        }
        Err(Box::from(SemanticError::UndeclaredType(
            var.data_type.to_string(),
        )))
    }

    fn find_var_in_table<'a>(
        type_table: &'a [TypeCell],
        identifier: &String,
    ) -> Option<&'a TypeCell> {
        type_table
            .iter()
            .find(|cell| cell.identifier == *identifier)
    }
}
