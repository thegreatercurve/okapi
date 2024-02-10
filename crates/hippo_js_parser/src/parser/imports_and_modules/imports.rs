use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl<'a> Parser<'a> {
    // 16.2.2 Imports
    // https://tc39.es/ecma262/#prod-ImportDeclaration
    pub(crate) fn parse_import_declaration(&mut self) -> Result<ImportDeclaration, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Import))?;

        if self.cursor.current_token_kind() == TokenKind::StringLiteral {
            let module_specifier = self.parse_module_specifier()?;

            self.expect_optional_semicolon_and_advance();

            return Ok(ImportDeclaration {
                node: self.end_node()?,
                source: module_specifier,
                specifiers: vec![],
            });
        }

        let mut import_clause = self.parse_import_clause()?;

        if self.cursor.current_token_kind() == TokenKind::Comma {
            self.cursor.advance(); // Eat comma token.

            if self.cursor.current_token_kind() == TokenKind::Multiplication {
                let name_space_import = self.parse_name_space_import()?;

                import_clause.push(name_space_import);
            } else if self.cursor.current_token_kind() == TokenKind::LeftCurlyBrace {
                if let Some(named_imports) = self.parse_named_imports()? {
                    import_clause.extend(named_imports);
                };
            }
        }

        let module_specifier = self.parse_from_clause()?;

        self.expect_optional_semicolon_and_advance();

        Ok(ImportDeclaration {
            node: self.end_node()?,
            source: module_specifier,
            specifiers: import_clause,
        })
    }

    // https://tc39.es/ecma262/#prod-ModuleExportName
    pub(crate) fn parse_module_export_name(&mut self) -> Result<ModuleExportName, ParserError> {
        self.start_node();

        match self.cursor.current_token_kind() {
            TokenKind::Identifier => {
                let identifier = self.parse_binding_identifier()?;

                self.end_node()?;

                Ok(ModuleExportName::Identifier(identifier))
            }
            TokenKind::StringLiteral => {
                let current_token_value = self.cursor.current_token_value();

                self.expect_and_advance(TokenKind::StringLiteral)?;

                let node = self.end_node()?;

                let string_literal = self.parse_string_literal(current_token_value, node)?;

                Ok(ModuleExportName::Literal(string_literal))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ImportClause
    fn parse_import_clause(&mut self) -> Result<Vec<ImportSpecifier>, ParserError> {
        let mut import_specifiers = vec![];

        match self.cursor.current_token_kind() {
            TokenKind::Identifier => {
                import_specifiers.push(self.parse_imported_default_binding()?);
            }
            TokenKind::Multiplication => {
                import_specifiers.push(self.parse_name_space_import()?);
            }
            TokenKind::LeftCurlyBrace => {
                if let Some(named_imports) = self.parse_named_imports()? {
                    import_specifiers.extend(named_imports);
                };
            }
            _ => return Err(self.unexpected_current_token_kind()),
        }

        return Ok(import_specifiers);
    }

    // https://tc39.es/ecma262/#prod-ImportedDefaultBinding
    fn parse_imported_default_binding(&mut self) -> Result<ImportSpecifier, ParserError> {
        self.start_node();

        let imported_binding_identifier = self.parse_imported_binding()?;

        return Ok(ImportSpecifier {
            node: self.end_node()?,
            kind: ImportSpecifierKind::ImportDefaultSpecifier,
            local: imported_binding_identifier,
            imported: None,
        });
    }

    // https://tc39.es/ecma262/#prod-NamedImports
    fn parse_name_space_import(&mut self) -> Result<ImportSpecifier, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Multiplication)?;

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::As))?;

        let imported_binding_identifier = self.parse_imported_binding()?;

        return Ok(ImportSpecifier {
            node: self.end_node()?,
            kind: ImportSpecifierKind::ImportNamespaceSpecifier,
            local: imported_binding_identifier,
            imported: None,
        });
    }

    // https://tc39.es/ecma262/#prod-NamedImports
    fn parse_named_imports(&mut self) -> Result<Option<Vec<ImportSpecifier>>, ParserError> {
        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        if self.cursor.current_token_kind() == TokenKind::RightCurlyBrace {
            self.start_node();

            self.cursor.advance();

            self.end_node()?;

            return Ok(None);
        }

        let imports_list = self.parse_imports_list()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Some(imports_list))
    }

    // https://tc39.es/ecma262/#prod-FromClause
    pub(crate) fn parse_from_clause(&mut self) -> Result<Literal, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::From))?;

        self.parse_module_specifier()
    }

    // https://tc39.es/ecma262/#prod-ImportsList
    fn parse_imports_list(&mut self) -> Result<Vec<ImportSpecifier>, ParserError> {
        let mut imports_list = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightCurlyBrace {
            imports_list.push(self.parse_import_specifier()?);

            if self.cursor.current_token_kind() == TokenKind::RightCurlyBrace {
                break;
            }

            if self.cursor.current_token_kind() == TokenKind::Comma {
                self.cursor.advance();
            }
        }

        Ok(imports_list)
    }

    // https://tc39.es/ecma262/#prod-ImportSpecifier
    fn parse_import_specifier(&mut self) -> Result<ImportSpecifier, ParserError> {
        match self.cursor.peek_token_kind() {
            TokenKind::Keyword(KeywordKind::As) => {
                self.start_node();

                let module_export_name = self.parse_module_export_name()?;

                self.expect_and_advance(TokenKind::Keyword(KeywordKind::As))?;

                let import_specifier = self.parse_imported_binding()?;

                Ok(ImportSpecifier {
                    node: self.end_node()?,
                    kind: ImportSpecifierKind::ImportSpecifier,
                    local: import_specifier,
                    imported: Some(module_export_name.to_import_specifier_imported()),
                })
            }
            _ => {
                self.start_node();

                let import_specifier = self.parse_imported_binding()?;

                let module_export_name = ModuleExportName::Identifier(import_specifier.clone());

                Ok(ImportSpecifier {
                    node: self.end_node()?,
                    kind: ImportSpecifierKind::ImportSpecifier,
                    local: import_specifier.clone(),
                    imported: Some(module_export_name.to_import_specifier_imported()),
                })
            }
        }
    }

    // https://tc39.es/ecma262/#prod-ModuleSpecifier
    fn parse_module_specifier(&mut self) -> Result<Literal, ParserError> {
        // TODO conversion logic should live in AST module.
        let source_literal = match self.parse_literal()? {
            Expression::Literal(literal) => literal,
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(source_literal)
    }

    // https://tc39.es/ecma262/#prod-ImportedBinding
    pub(crate) fn parse_imported_binding(&mut self) -> Result<Identifier, ParserError> {
        self.parse_binding_identifier()
    }
}
