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
            TokenKind::Multiplication => {
                let export_from_clause = self.parse_export_from_clause_export_all_declaration()?;

                let from_clause = self.parse_from_clause()?;

                self.expect_optional_semicolon_and_advance();

                Ok(ExportDeclaration::ExportAll(ExportAllDeclaration {
                    node: self.end_node(start_index)?,
                    source: from_clause,
                    exported: export_from_clause,
                }))
            }
            TokenKind::LeftCurlyBrace => {
                let named_exports = self.parse_named_exports()?;

                let mut from_clause = None;

                if self.token_kind() == TokenKind::Keyword(KeywordKind::From) {
                    from_clause = Some(self.parse_from_clause()?);
                }

                self.expect_optional_semicolon_and_advance();

                Ok(ExportDeclaration::ExportNamed(ExportNamedDeclaration {
                    node: self.end_node(start_index)?,
                    declaration: None,
                    specifiers: named_exports,
                    source: from_clause,
                }))
            }
            token_kind if token_kind.is_declaration_start() => {
                let variable_declaration = self.parse_lexical_declaration(true)?;

                Ok(ExportDeclaration::ExportNamed(ExportNamedDeclaration {
                    node: self.end_node(start_index)?,
                    declaration: Some(ExportNamedDeclarationDeclaration::Variable(
                        variable_declaration,
                    )),
                    specifiers: vec![],
                    source: None,
                }))
            }
            TokenKind::Keyword(KeywordKind::Var) => {
                let variable_declaration = self.parse_lexical_declaration(true)?;

                Ok(ExportDeclaration::ExportNamed(ExportNamedDeclaration {
                    node: self.end_node(start_index)?,
                    declaration: Some(ExportNamedDeclarationDeclaration::Variable(
                        variable_declaration,
                    )),
                    specifiers: vec![],
                    source: None,
                }))
            }
            TokenKind::Keyword(KeywordKind::Function) => {
                let function_declaration = self.parse_function_declaration()?;

                self.expect_optional_semicolon_and_advance();

                Ok(ExportDeclaration::ExportNamed(ExportNamedDeclaration {
                    node: self.end_node(start_index)?,
                    declaration: Some(ExportNamedDeclarationDeclaration::Function(
                        function_declaration,
                    )),
                    specifiers: vec![],
                    source: None,
                }))
            }
            TokenKind::Keyword(KeywordKind::Class) => {
                let class_declaration = self.parse_class_declaration()?;

                Ok(ExportDeclaration::ExportNamed(ExportNamedDeclaration {
                    node: self.end_node(start_index)?,
                    declaration: Some(ExportNamedDeclarationDeclaration::Class(class_declaration)),
                    specifiers: vec![],
                    source: None,
                }))
            }
            // TODO Handle default exports properly.
            _ => Err(self.unexpected_current_token_kind()),
        }
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

        return Ok(None);
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
