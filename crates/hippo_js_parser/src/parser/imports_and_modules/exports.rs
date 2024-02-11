use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl<'a> Parser<'a> {
    // 16.2.2 Imports
    // https://tc39.es/ecma262/#prod-ExportDeclaration
    pub(crate) fn parse_export_declaration(&mut self) -> Result<ExportDeclaration, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Export))?;

        match self.cursor.current_token_kind() {
            TokenKind::Multiplication => {
                let export_from_clause = self.parse_export_from_clause_export_all_declaration()?;

                let from_clause = self.parse_from_clause()?;

                self.expect_optional_semicolon_and_advance();

                Ok(ExportDeclaration::ExportAllDeclaration(
                    ExportAllDeclaration {
                        node: self.end_node()?,
                        source: from_clause,
                        exported: match export_from_clause {
                            Some(mode_export_name) => {
                                Some(mode_export_name.to_export_all_declaration_exported())
                            }
                            _ => None,
                        },
                    },
                ))
            }
            TokenKind::LeftCurlyBrace => {
                let named_exports = self.parse_named_exports()?;

                let mut from_clause = None;

                if self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::From) {
                    from_clause = Some(self.parse_from_clause()?);
                }

                self.expect_optional_semicolon_and_advance();

                Ok(ExportDeclaration::ExportNamedDeclaration(
                    ExportNamedDeclaration {
                        node: self.end_node()?,
                        declaration: None,
                        specifiers: named_exports,
                        source: from_clause,
                    },
                ))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ExportFromClause
    fn parse_export_from_clause_export_all_declaration(
        &mut self,
    ) -> Result<Option<ModuleExportName>, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Multiplication)?;

        if self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::As) {
            self.cursor.advance(); // Eat `*` token.

            let module_export_name = self.parse_module_export_name()?;

            self.end_node()?;

            return Ok(Some(module_export_name));
        }

        self.end_node()?;

        return Ok(None);
    }

    fn parse_named_exports(&mut self) -> Result<Vec<ExportSpecifier>, ParserError> {
        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let mut exports_list = vec![];

        if self.cursor.current_token_kind() == TokenKind::RightCurlyBrace {
            self.start_node();

            self.cursor.advance();

            self.end_node()?;

            return Ok(exports_list);
        }

        exports_list.extend(self.parse_exports_list()?);

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(exports_list)
    }

    // https://tc39.es/ecma262/#prod-ImportsList
    fn parse_exports_list(&mut self) -> Result<Vec<ExportSpecifier>, ParserError> {
        let mut exports_list = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightCurlyBrace {
            exports_list.push(self.parse_export_specifier()?);

            if self.cursor.current_token_kind() == TokenKind::RightCurlyBrace {
                break;
            }

            if self.cursor.current_token_kind() == TokenKind::Comma {
                self.cursor.advance();
            }
        }

        Ok(exports_list)
    }

    // https://tc39.es/ecma262/#prod-ExportSpecifier
    fn parse_export_specifier(&mut self) -> Result<ExportSpecifier, ParserError> {
        match self.cursor.peek_token_kind() {
            TokenKind::Keyword(KeywordKind::As) => {
                self.start_node();

                let local_module_export_name = self.parse_module_export_name()?;

                self.expect_and_advance(TokenKind::Keyword(KeywordKind::As))?;

                let export_module_export_name = self.parse_module_export_name()?;

                Ok(ExportSpecifier {
                    node: self.end_node()?,
                    local: Box::new(local_module_export_name.to_export_specifier_local()),
                    exported: Box::new(export_module_export_name.to_export_specifier_exported()),
                })
            }
            _ => {
                self.start_node();

                let module_export_name = self.parse_module_export_name()?;

                Ok(ExportSpecifier {
                    node: self.end_node()?,
                    local: Box::new(module_export_name.clone().to_export_specifier_local()),
                    exported: Box::new(module_export_name.to_export_specifier_exported()),
                })
            }
        }
    }
}
