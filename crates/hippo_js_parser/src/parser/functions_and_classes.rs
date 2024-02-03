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
    // https://tc39.es/ecma262/#prod-FunctionExpression
    pub(crate) fn parse_function_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        if self.peek_token_kind() == TokenKind::Identifier {
            let identifier = self.parse_binding_identifier()?;
        }

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

    // 15.3 Arrow Function Definitions
    // https://tc39.es/ecma262/#sec-arrow-function-definitions
    pub(crate) fn parse_arrow_function(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_arrow_function")
    }

    // 15.4 Method Definitions
    // https://tc39.es/ecma262/#sec-method-definitions
    pub(crate) fn parse_method_definition(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_method_definition")
    }

    // 15.5 Generator Function Definitions
    // https://tc39.es/ecma262/#prod-GeneratorExpression
    pub(crate) fn parse_generator_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_generator_expression")
    }

    // https://tc39.es/ecma262/#prod-YieldExpression
    pub(crate) fn parse_yield_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_yield_expression")
    }

    // 15.6 Async Generator Function Definitions
    // https://tc39.es/ecma262/#prod-AsyncGeneratorExpression
    pub(crate) fn parse_async_generator_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_async_generator_expression")
    }

    // 15.7 Class Definitions
    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_class_expression")
    }

    // 15.8 Async Function Definitions
    // https://tc39.es/ecma262/#prod-AsyncFunctionExpression
    pub fn parse_async_function_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_async_function_expression")
    }
}
