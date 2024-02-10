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
                let export_from_clause = self.parse_export_from_clause()?;

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
                todo!("parse_export_clause")
            }
            _ => todo!(),
        }
    }

    // https://tc39.es/ecma262/#prod-ExportFromClause
    fn parse_export_from_clause(&mut self) -> Result<Option<ModuleExportName>, ParserError> {
        self.start_node();

        match self.cursor.current_token_kind() {
            TokenKind::Multiplication => {
                self.cursor.advance(); // Eat `*` token.

                if self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::As) {
                    let module_export_name = self.parse_module_export_name()?;

                    return Ok(Some(module_export_name));
                } else {
                    self.end_node()?;

                    return Ok(None);
                }
            }
            TokenKind::LeftCurlyBrace => {
                todo!("parse_export_clause")
            }
            _ => return Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ExportSpecifier
    fn parse_export_specifier(&mut self) -> Result<ExportSpecifier, ParserError> {
        match self.cursor.peek_token_kind() {
            TokenKind::Keyword(KeywordKind::As) => {
                self.start_node();

                let local_module_export_name = self.parse_module_export_name()?;

                let export_module_export_name = self.parse_module_export_name()?;

                self.expect_and_advance(TokenKind::Keyword(KeywordKind::As))?;

                Ok(ExportSpecifier {
                    node: self.end_node()?,
                    local: todo!(),
                    exported: todo!(),
                })
            }
            _ => {
                self.start_node();

                let import_specifier = self.parse_imported_binding()?;

                Ok(ExportSpecifier {
                    node: self.end_node()?,
                    local: todo!(),
                    exported: todo!(),
                })
            }
        }
    }
}
