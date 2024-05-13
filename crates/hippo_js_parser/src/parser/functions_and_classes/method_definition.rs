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
        class_element_name: Option<PropertyDefinitionKey>,
        method_definition_kind: MethodDefinitionKind,
        is_static: bool,
        is_computed: bool,
    ) -> Result<MethodDefinition, ParserError> {
        // Eat 'get' or 'set' keyword if neither are being used as method definition identifiers as we've already calculated the method_definition_kind.
        if matches!(
            self.token_kind(),
            TokenKind::Keyword(KeywordKind::Set) | TokenKind::Keyword(KeywordKind::Get)
        ) && self.peek_token_kind() != TokenKind::LeftParenthesis
        {
            self.advance_any(); // Eat 'get' or 'set' token.
        }

        let is_computed = is_computed || self.token_kind() == TokenKind::LeftSquareBracket;

        let mut is_async = false;

        let class_element_name = match class_element_name {
            Some(class_element_name) => class_element_name,
            None => {
                match self.token_kind() {
                    TokenKind::Keyword(KeywordKind::Async) => {
                        if self.peek_token_kind().is_class_element_name_start() {
                            return Ok(self.parse_async_method(
                                start_index,
                                is_computed,
                                is_static,
                                method_definition_kind,
                            )?);
                        } else {
                            is_async = true;

                            self.advance_any(); // Eat 'async' token.

                            if self.token_kind() == TokenKind::Multiplication {
                                return Ok(self.parse_generator_method(
                                    start_index,
                                    is_computed,
                                    is_async,
                                    is_static,
                                    method_definition_kind,
                                )?);
                            }
                        }
                    }
                    TokenKind::Multiplication => {
                        return Ok(self.parse_generator_method(
                            start_index,
                            is_computed,
                            is_async,
                            is_static,
                            method_definition_kind,
                        )?);
                    }
                    _ => {}
                };

                match self.token_kind() {
                    token_kind if token_kind.is_class_element_name_start() => {
                        self.parse_class_element_name()?
                    }
                    _ => return Err(self.unexpected_current_token_kind()),
                }
            }
        };

        let method_definition_body = match method_definition_kind {
            MethodDefinitionKind::Constructor | MethodDefinitionKind::Method => {
                self.parse_method_definition_method_body()?
            }
            MethodDefinitionKind::Get => self.parse_method_definition_getter_body()?,
            MethodDefinitionKind::Set => self.parse_method_definition_setter_body()?,
        };

        Ok(MethodDefinition {
            node: self.end_node(start_index)?,
            kind: method_definition_kind,
            value: Some(method_definition_body),
            is_static,
            computed: is_computed,
            key: Some(class_element_name),
        })
    }

    // Helper method to detect the type of method definition.
    pub(crate) fn parse_method_definition_kind(&mut self) -> MethodDefinitionKind {
        match self.token_kind() {
            _ if self.token_value()
                == TokenValue::String {
                    raw: "constructor".to_string(),
                    value: "constructor".to_string(),
                } =>
            {
                MethodDefinitionKind::Constructor
            }

            TokenKind::Keyword(KeywordKind::Get)
                if self.peek_token_kind() != TokenKind::LeftParenthesis =>
            {
                MethodDefinitionKind::Get
            }

            TokenKind::Keyword(KeywordKind::Set)
                if self.peek_token_kind() != TokenKind::LeftParenthesis =>
            {
                MethodDefinitionKind::Set
            }
            _ => MethodDefinitionKind::Method,
        }
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
