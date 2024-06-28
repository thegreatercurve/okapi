use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl Parser {
    // 16.2.2 Imports
    // https://tc39.es/ecma262/#prod-ExportDeclaration
    pub(crate) fn parse_export_declaration(&mut self) -> Result<ExportDeclaration, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Export))?;

        match self.token_kind() {
            // `export ExportFromClause FromClause ;`
            TokenKind::Multiplication => {
                let export_from_clause = self.parse_export_from_clause_export_all_declaration()?;

                let from_clause = self.parse_from_clause()?;

                self.expect_optional_semicolon_and_advance();

                return Ok(ExportDeclaration::All(ExportAllDeclaration {
                    node: self.end_node(start_index)?,
                    source: from_clause,
                    exported: export_from_clause,
                }));
            }
            // `export NamedExports ;`
            TokenKind::LeftCurlyBrace => {
                let named_exports = self.parse_named_exports()?;

                let mut from_clause = None;

                if self.token_kind() == TokenKind::Keyword(KeywordKind::From) {
                    from_clause = Some(self.parse_from_clause()?);
                }

                self.expect_optional_semicolon_and_advance();

                return Ok(ExportDeclaration::Named(ExportNamedDeclaration {
                    node: self.end_node(start_index)?,
                    declaration: None,
                    specifiers: named_exports,
                    source: from_clause,
                }));
            }
            _ => {}
        }

        let optional_declaration = match self.token_kind() {
            // `export VariableStatement`
            TokenKind::Keyword(KeywordKind::Var) => Some(
                ExportNamedDeclarationDeclaration::Variable(self.parse_variable_statement(true)?),
            ),
            // `export Declaration > HoistableDeclaration`
            token_kind if token_kind.is_hoistable_declaration_start() => Some(
                ExportNamedDeclarationDeclaration::Function(self.parse_function_declaration()?),
            ),
            // `export Declaration > ClassDeclaration`
            TokenKind::Keyword(KeywordKind::Class) => Some(
                ExportNamedDeclarationDeclaration::Class(self.parse_class_declaration()?),
            ),
            // `export Declaration > LexicalDeclaration`
            token_kind if token_kind.is_lexical_declaration_start() => Some(
                ExportNamedDeclarationDeclaration::Variable(self.parse_lexical_declaration(true)?),
            ),
            _ => None,
        };

        if optional_declaration.is_some() {
            return Ok(ExportDeclaration::Named(ExportNamedDeclaration {
                node: self.end_node(start_index)?,
                declaration: optional_declaration,
                specifiers: vec![],
                source: None,
            }));
        };

        match self.token_kind() {
            // `export default HoistableDeclaration`
            // `export default ClassDeclaration`
            // `export default AssignmentExpression`
            TokenKind::Keyword(KeywordKind::Default) => {
                self.parse_default_export_declaration(start_index)
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ExportDeclaration
    fn parse_default_export_declaration(
        &mut self,
        start_index: usize,
    ) -> Result<ExportDeclaration, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Default))?;

        let export_declaration = match self.token_kind() {
            token_kind if token_kind.is_class_declaration_start() => {
                ExportDefaultDeclarationDeclaration::ClassDeclaration(
                    self.parse_class_declaration()?,
                )
            }
            token_kind if token_kind.is_hoistable_declaration_start() => {
                ExportDefaultDeclarationDeclaration::FunctionDeclaration(
                    self.parse_hoistable_declaration()?,
                )
            }
            _ => {
                ExportDefaultDeclarationDeclaration::Expression(self.parse_assignment_expression()?)
            }
        };

        Ok(ExportDeclaration::Default(ExportDefaultDeclaration {
            node: self.end_node(start_index)?,
            declaration: export_declaration,
        }))
    }

    // https://tc39.es/ecma262/#prod-ExportFromClause
    fn parse_export_from_clause_export_all_declaration(
        &mut self,
    ) -> Result<Option<ModuleExportName>, ParserError> {
        self.expect_and_advance(TokenKind::Multiplication)?;

        if self.token_kind() == TokenKind::Keyword(KeywordKind::As) {
            self.advance_any(); // Eat '*' token.

            let module_export_name = self.parse_module_export_name()?;

            return Ok(Some(module_export_name));
        }

        Ok(None)
    }

    fn parse_named_exports(&mut self) -> Result<Vec<ExportSpecifier>, ParserError> {
        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let mut exports_list = vec![];

        if self.token_kind() == TokenKind::RightCurlyBrace {
            self.advance_any();

            return Ok(exports_list);
        }

        exports_list.extend(self.parse_exports_list()?);

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(exports_list)
    }

    // https://tc39.es/ecma262/#prod-ImportsList
    fn parse_exports_list(&mut self) -> Result<Vec<ExportSpecifier>, ParserError> {
        let mut exports_list = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            exports_list.push(self.parse_export_specifier()?);

            if self.token_kind() == TokenKind::RightCurlyBrace {
                break;
            }

            if self.token_kind() == TokenKind::Comma {
                self.advance_any();
            }
        }

        Ok(exports_list)
    }

    // https://tc39.es/ecma262/#prod-ExportSpecifier
    fn parse_export_specifier(&mut self) -> Result<ExportSpecifier, ParserError> {
        match self.peek_token_kind() {
            TokenKind::Keyword(KeywordKind::As) => {
                let start_index = self.start_node();

                let local_module_export_name = self.parse_module_export_name()?;

                self.expect_and_advance(TokenKind::Keyword(KeywordKind::As))?;

                let export_module_export_name = self.parse_module_export_name()?;

                Ok(ExportSpecifier {
                    node: self.end_node(start_index)?,
                    local: Box::new(local_module_export_name),
                    exported: Box::new(export_module_export_name),
                })
            }
            _ => {
                let start_index = self.start_node();

                let module_export_name = self.parse_module_export_name()?;

                Ok(ExportSpecifier {
                    node: self.end_node(start_index)?,
                    local: Box::new(module_export_name.clone()),
                    exported: Box::new(module_export_name),
                })
            }
        }
    }
}
