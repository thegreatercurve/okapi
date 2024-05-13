use crate::{ast::*, KeywordKind, TokenKind};
use crate::{Parser, ParserError};

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl Parser {
    // 15.6 Async Generator Function Definitions
    // https://tc39.es/ecma262/#prod-AsyncGeneratorDeclaration
    pub(crate) fn parse_async_generator_declaration(
        &mut self,
    ) -> Result<FunctionDeclaration, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Async))?;

        if self.has_previous_token_line_terminator() {
            return Err(ParserError::UnexpectedLineTerminator);
        }

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        self.expect_and_advance(TokenKind::Multiplication)?;

        let optional_binding_identifier = if self.token_kind().is_binding_identifier() {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        let formal_parameters = self.parse_parenthesized_formal_parameters()?;

        let generator_body = self.parse_function_body()?;

        Ok(FunctionDeclaration {
            node: self.end_node(start_index)?,
            id: optional_binding_identifier,
            params: formal_parameters,
            body: generator_body,
            generator: true,
            asynchronous: true,
            expression: false,
        })
    }

    // https://tc39.es/ecma262/#prod-AsyncGeneratorExpression
    pub(crate) fn parse_async_generator_expression(
        &mut self,
    ) -> Result<FunctionExpression, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Async))?;

        if self.has_previous_token_line_terminator() {
            return Err(ParserError::UnexpectedLineTerminator);
        }

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        self.expect_and_advance(TokenKind::Multiplication)?;

        let optional_binding_identifier = if self.token_kind().is_binding_identifier() {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        let formal_parameters = self.parse_parenthesized_formal_parameters()?;

        let generator_body = self.parse_function_body()?;

        Ok(FunctionExpression {
            node: self.end_node(start_index)?,
            id: optional_binding_identifier,
            expression: false,
            generator: true,
            is_async: true,
            params: formal_parameters,
            body: generator_body,
        })
    }
}
