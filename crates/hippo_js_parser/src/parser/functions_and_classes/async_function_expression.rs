use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl Parser {
    // 15.8 Async Function Definitions
    // https://tc39.es/ecma262/#prod-AsyncFunctionDeclaration
    pub(crate) fn parse_async_function_declaration(
        &mut self,
    ) -> Result<FunctionDeclaration, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Async))?;

        if self.has_previous_token_line_terminator() {
            return Err(ParserError::UnexpectedLineTerminator);
        }

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        let optional_identifier = if self.token_kind().is_binding_identifier() {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        let formal_parameters = self.parse_parenthesized_formal_parameters()?;

        let body = self.parse_function_body()?;

        Ok(FunctionDeclaration {
            node: self.end_node(start_index)?,
            id: optional_identifier,
            params: formal_parameters,
            body,
            generator: false,
            asynchronous: true,
            expression: false,
        })
    }

    // https://tc39.es/ecma262/#prod-AsyncFunctionExpression
    pub(crate) fn parse_async_function_expression(
        &mut self,
    ) -> Result<FunctionExpression, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Async))?;

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        let optional_identifier = if self.token_kind().is_binding_identifier() {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        let formal_parameters = self.parse_parenthesized_formal_parameters()?;

        let body = self.parse_function_body()?;

        Ok(FunctionExpression {
            node: self.end_node(start_index)?,
            id: optional_identifier,
            body,
            params: formal_parameters,
            expression: false,
            generator: false,
            is_async: true,
        })
    }
}
