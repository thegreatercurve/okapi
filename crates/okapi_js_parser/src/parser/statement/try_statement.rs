use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.12 The try Statement
    // https://tc39.es/ecma262/#prod-TryStatement
    pub(crate) fn parse_try_statement(&mut self) -> Result<TryStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Try))?;

        let block = self.parse_block_statement()?;

        let optional_handler = if self.token_kind() == TokenKind::Keyword(KeywordKind::Catch) {
            Some(self.parse_catch_clause()?)
        } else {
            None
        };

        let optional_finalizer = if self.token_kind() == TokenKind::Keyword(KeywordKind::Finally) {
            Some(self.parse_finally()?)
        } else {
            None
        };

        Ok(TryStatement {
            node: self.end_node(start_index)?,
            block,
            handler: optional_handler,
            finalizer: optional_finalizer,
        })
    }

    // https://tc39.es/ecma262/#prod-Catch
    fn parse_catch_clause(&mut self) -> Result<CatchClause, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Catch))?;

        let optional_binding_identifier_or_pattern = match self.token_kind() {
            TokenKind::LeftParenthesis => {
                self.advance_any(); // Eat '(' token.

                let binding_idenfitier_or_pattern =
                    if self.token_kind() != TokenKind::RightParenthesis {
                        Some(match self.token_kind() {
                            token_kind if token_kind.is_binding_identifier() => {
                                Pattern::Identifier(self.parse_binding_identifier()?)
                            }
                            TokenKind::LeftCurlyBrace => {
                                Pattern::Object(self.parse_object_binding_pattern()?)
                            }
                            TokenKind::LeftSquareBracket => {
                                Pattern::Array(self.parse_array_binding_pattern()?)
                            }
                            _ => return Err(self.unexpected_current_token_kind()),
                        })
                    } else {
                        None
                    };

                self.expect_and_advance(TokenKind::RightParenthesis)?;

                binding_idenfitier_or_pattern
            }
            _ => None,
        };

        let body = self.parse_block_statement()?;

        Ok(CatchClause {
            node: self.end_node(start_index)?,
            param: optional_binding_identifier_or_pattern,
            body,
        })
    }

    // https://tc39.es/ecma262/#prod-Finally
    fn parse_finally(&mut self) -> Result<BlockStatement, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Finally))?;

        self.parse_block_statement()
    }
}
