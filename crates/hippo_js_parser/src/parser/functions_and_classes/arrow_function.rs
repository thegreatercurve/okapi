use std::vec;

use crate::ast::*;
use crate::{Parser, ParserError, TokenKind};

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl Parser {
    // 15.3 Arrow Function Definitions
    // https://tc39.es/ecma262/#prod-ArrowFunction
    // https://tc39.es/ecma262/#prod-ArrowFormalParameters
    pub(crate) fn parse_arrow_function(&mut self) -> Result<Expression, ParserError> {
        let start_index = self.start_node();

        let parameters = if self.token_kind().is_binding_identifier() {
            vec![Pattern::Identifier(self.parse_binding_identifier()?)]
        } else {
            self.parse_parenthesized_formal_parameters()?
                .into_iter()
                .map(Pattern::try_from)
                .collect::<Result<Vec<Pattern>, ParserError>>()?
        };

        if self.has_previous_token_line_terminator() {
            return Err(ParserError::UnexpectedLineTerminator);
        }

        self.expect_and_advance(TokenKind::ArrowFunction)?;

        let is_expression = self.token_kind() != TokenKind::LeftCurlyBrace;

        let arrow_function_body = if self.token_kind() == TokenKind::LeftCurlyBrace {
            ArrowFunctionExpressionBody::BlockStatement(self.parse_function_body()?)
        } else {
            ArrowFunctionExpressionBody::Expression(Box::new(self.parse_assignment_expression()?))
        };

        Ok(Expression::ArrowFunction(ArrowFunctionExpression {
            node: self.end_node(start_index)?,
            id: None,
            body: arrow_function_body,
            params: parameters,
            expression: is_expression,
            generator: false,
            is_async: false,
        }))
    }
}
