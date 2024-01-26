use std::vec;

use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.1 Parameter Lists
    // https://tc39.es/ecma262/#sec-parameter-lists
    fn parse_formal_parameters(&mut self) -> Result<Statement, ParserError> {
        todo!("parse_formal_parameters")
    }

    fn parse_function_body(&mut self) -> Result<Statement, ParserError> {
        todo!("parse_function_body")
    }

    // 15.2 Function Definitions
    // https://tc39.es/ecma262/#sec-function-definitions
    pub(crate) fn parse_function_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        let identifier = self.parse_binding_identifier()?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let formal_paramaters = self.parse_formal_parameters()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let body = self.parse_function_body()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Expression::Function(FunctionExpression {
            node: self.create_node(&start_token, &self.previous_token),
        }))
    }
}
