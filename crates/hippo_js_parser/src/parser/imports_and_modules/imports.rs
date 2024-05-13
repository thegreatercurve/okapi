use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl Parser {
    // 16.2.2 Imports
    // https://tc39.es/ecma262/#prod-ImportDeclaration
    pub(crate) fn parse_import_declaration(&mut self) -> Result<ImportDeclaration, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Import))?;

        if self.token_kind() == TokenKind::StringLiteral {
            let module_specifier = self.parse_module_specifier()?;

            self.expect_optional_semicolon_and_advance();

            return Ok(ImportDeclaration {
                node: self.end_node(start_index)?,
                source: module_specifier,
                specifiers: vec![],
            });
        }

        let mut import_clause = self.parse_import_clause()?;

        if self.token_kind() == TokenKind::Comma {
            self.advance_any(); // Eat comma token.

            if self.token_kind() == TokenKind::Multiplication {
                let name_space_import = self.parse_name_space_import()?;

                import_clause.push(name_space_import);
            } else if self.token_kind() == TokenKind::LeftCurlyBrace {
                import_clause.extend(self.parse_named_imports()?);
            }
        }

        let module_specifier = self.parse_from_clause()?;

        self.expect_optional_semicolon_and_advance();

        Ok(ImportDeclaration {
            node: self.end_node(start_index)?,
            source: module_specifier,
            specifiers: import_clause,
        })
    }

    // https://tc39.es/ecma262/#prod-ModuleExportName
    pub(crate) fn parse_module_export_name(&mut self) -> Result<ModuleExportName, ParserError> {
        let start_index = self.start_node();

        match self.token_kind() {
            token_kind if token_kind.is_binding_identifier() => {
                let identifier = self.parse_binding_identifier()?;

                Ok(ModuleExportName::Identifier(identifier))
            }
            TokenKind::StringLiteral => {
                let current_token_value = self.token_value();

                self.expect_and_advance(TokenKind::StringLiteral)?;

                let node = self.end_node(start_index)?;

                let string_literal = self.parse_string_literal(current_token_value, node)?;

                Ok(ModuleExportName::Literal(string_literal))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ImportClause
    fn parse_import_clause(&mut self) -> Result<Vec<ImportSpecifier>, ParserError> {
        let mut import_specifiers = vec![];

        match self.token_kind() {
            token_kind if token_kind.is_binding_identifier() => {
                import_specifiers.push(self.parse_imported_default_binding()?);
            }
            TokenKind::Multiplication => {
                import_specifiers.push(self.parse_name_space_import()?);
            }
            TokenKind::LeftCurlyBrace => {
                import_specifiers.extend(self.parse_named_imports()?);
            }
            _ => return Err(self.unexpected_current_token_kind()),
        }

        return Ok(import_specifiers);
    }

    // https://tc39.es/ecma262/#prod-ImportedDefaultBinding
    fn parse_imported_default_binding(&mut self) -> Result<ImportSpecifier, ParserError> {
        let start_index = self.start_node();

        let imported_binding_identifier = self.parse_imported_binding()?;

        return Ok(ImportSpecifier {
            node: self.end_node(start_index)?,
            kind: ImportSpecifierKind::ImportDefaultSpecifier,
            local: imported_binding_identifier,
            imported: None,
        });
    }

    // https://tc39.es/ecma262/#prod-NamedImports
    fn parse_name_space_import(&mut self) -> Result<ImportSpecifier, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Multiplication)?;

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::As))?;

        let imported_binding_identifier = self.parse_imported_binding()?;

        return Ok(ImportSpecifier {
            node: self.end_node(start_index)?,
            kind: ImportSpecifierKind::ImportNamespaceSpecifier,
            local: imported_binding_identifier,
            imported: None,
        });
    }

    // https://tc39.es/ecma262/#prod-NamedImports
    fn parse_named_imports(&mut self) -> Result<Vec<ImportSpecifier>, ParserError> {
        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let mut imports_list = vec![];

        if self.token_kind() == TokenKind::RightCurlyBrace {
            self.advance_any();

            return Ok(imports_list);
        }

        imports_list.extend(self.parse_imports_list()?);

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(imports_list)
    }

    // https://tc39.es/ecma262/#prod-FromClause
    pub(crate) fn parse_from_clause(&mut self) -> Result<Literal, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::From))?;

        self.parse_module_specifier()
    }

    // https://tc39.es/ecma262/#prod-ImportsList
    fn parse_imports_list(&mut self) -> Result<Vec<ImportSpecifier>, ParserError> {
        let mut imports_list = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            imports_list.push(self.parse_import_specifier()?);

            if self.token_kind() == TokenKind::RightCurlyBrace {
                break;
            }

            if self.token_kind() == TokenKind::Comma {
                self.advance_any();
            }
        }

        Ok(imports_list)
    }

    // https://tc39.es/ecma262/#prod-ImportSpecifier
    fn parse_import_specifier(&mut self) -> Result<ImportSpecifier, ParserError> {
        match self.peek_token_kind() {
            TokenKind::Keyword(KeywordKind::As) => {
                let start_index = self.start_node();

                let module_export_name = self.parse_module_export_name()?;

                self.expect_and_advance(TokenKind::Keyword(KeywordKind::As))?;

                let import_specifier = self.parse_imported_binding()?;

                Ok(ImportSpecifier {
                    node: self.end_node(start_index)?,
                    kind: ImportSpecifierKind::ImportSpecifier,
                    local: import_specifier,
                    imported: Some(module_export_name),
                })
            }
            _ => {
                let start_index = self.start_node();

                let import_specifier = self.parse_imported_binding()?;

                let module_export_name = ModuleExportName::Identifier(import_specifier.clone());

                Ok(ImportSpecifier {
                    node: self.end_node(start_index)?,
                    kind: ImportSpecifierKind::ImportSpecifier,
                    local: import_specifier.clone(),
                    imported: Some(module_export_name),
                })
            }
        }
    }

    // https://tc39.es/ecma262/#prod-ModuleSpecifier
    fn parse_module_specifier(&mut self) -> Result<Literal, ParserError> {
        let source_literal = self.parse_literal()?;

        Ok(source_literal)
    }

    // https://tc39.es/ecma262/#prod-ImportedBinding
    pub(crate) fn parse_imported_binding(&mut self) -> Result<Identifier, ParserError> {
        self.parse_binding_identifier()
    }
}
