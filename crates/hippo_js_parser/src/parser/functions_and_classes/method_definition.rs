use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind, TokenValue};

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl Parser {
    // 15.4 Method Definitions
    // https://tc39.es/ecma262/#prod-MethodDefinition
    pub(crate) fn parse_method_definition(
        &mut self,
        start_index: usize,
        key: Option<PropertyDefinitionKey>,
        value: Option<FunctionExpression>,
        method_definition_kind: MethodDefinitionKind,
        is_static: bool,
        is_computed: bool,
    ) -> Result<MethodDefinition, ParserError> {
        Ok(MethodDefinition {
            node: self.end_node(start_index)?,
            kind: method_definition_kind,
            value,
            is_static,
            computed: is_computed,
            key,
        })
    }

    pub(crate) fn parse_method_definition_method_body(
        &mut self,
    ) -> Result<FunctionExpression, ParserError> {
        let start_index = self.start_node();

        let formal_parameters = self.parse_parenthesized_formal_parameters()?;

        let function_body = self.parse_function_body()?;

        Ok(FunctionExpression {
            node: self.end_node(start_index)?,
            id: None,
            expression: false,
            generator: false,
            is_async: false,
            params: formal_parameters,
            body: function_body,
        })
    }

    pub(crate) fn parse_method_definition_getter_body(
        &mut self,
    ) -> Result<FunctionExpression, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let function_body = self.parse_function_body()?;

        Ok(FunctionExpression {
            node: self.end_node(start_index)?,
            id: None,
            expression: false,
            generator: false,
            is_async: false,
            params: vec![],
            body: function_body,
        })
    }

    pub(crate) fn parse_method_definition_setter_body(
        &mut self,
    ) -> Result<FunctionExpression, ParserError> {
        let start_index = self.start_node();

        let formal_parameters = self.parse_parenthesized_formal_parameters()?;

        if formal_parameters.len() != 1 {
            return Err(self.unexpected_current_token_kind());
        }

        let function_body = self.parse_function_body()?;

        Ok(FunctionExpression {
            node: self.end_node(start_index)?,
            id: None,
            expression: false,
            generator: false,
            is_async: false,
            params: formal_parameters,
            body: function_body,
        })
    }
}
