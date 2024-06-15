use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 15.5 Generator Function Definitions
    // https://tc39.es/ecma262/#prod-GeneratorExpression
    pub(crate) fn parse_generator_declaration(
        &mut self,
    ) -> Result<FunctionDeclaration, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        self.expect_and_advance(TokenKind::Multiplication)?;

        let optional_binding_identifier = if self.token_kind().is_binding_identifier() {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let formal_parameters = self
            .parse_formal_parameters()?
            .into_iter()
            .map(FunctionParameter::try_from)
            .collect::<Result<Vec<FunctionParameter>, ParserError>>()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let generator_body = self.parse_function_body()?;

        Ok(FunctionDeclaration {
            node: self.end_node(start_index)?,
            id: optional_binding_identifier,
            params: formal_parameters,
            body: generator_body,
            generator: true,
            asynchronous: false,
            expression: false,
        })
    }

    // https://tc39.es/ecma262/#prod-GeneratorExpression
    pub(crate) fn parse_generator_expression(&mut self) -> Result<FunctionExpression, ParserError> {
        let start_index = self.start_node();

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
            params: formal_parameters,
            body: generator_body,
            generator: true,
            expression: false,
            is_async: false,
        })
    }

    // https://tc39.es/ecma262/#prod-GeneratorMethod
    pub(crate) fn parse_generator_method(
        &mut self,
        start_index: usize,
        is_computed: bool,
        is_async: bool,
        is_static: bool,
        kind: MethodDefinitionKind,
    ) -> Result<MethodDefinition, ParserError> {
        self.expect_and_advance(TokenKind::Multiplication)?;

        // Handle computed generator methods like `*[foo]() {}`.
        let is_computed = is_computed || self.token_kind() == TokenKind::LeftSquareBracket;

        let class_element_name = self.parse_class_element_name()?;

        let function_expression_start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let formal_parameters = self
            .parse_formal_parameters()?
            .into_iter()
            .map(FunctionParameter::try_from)
            .collect::<Result<Vec<FunctionParameter>, ParserError>>()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let function_body = self.parse_function_body()?;

        let function_expression = FunctionExpression {
            node: self.end_node(function_expression_start_index)?,
            id: None,
            expression: false,
            generator: true,
            is_async,
            params: formal_parameters,
            body: function_body,
        };

        Ok(MethodDefinition {
            node: self.end_node(start_index)?, // End class element node.
            is_static,
            computed: is_computed,
            key: Some(class_element_name),
            kind,
            value: Some(function_expression),
        })
    }

    // https://tc39.es/ecma262/#prod-YieldExpression
    pub(crate) fn parse_yield_expression(&mut self) -> Result<YieldExpression, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Yield))?;

        if self.token_kind() == TokenKind::Semicolon {
            let node = self.end_node(start_index)?;

            self.expect_optional_semicolon_and_advance(); // Eat ';' token.

            return Ok(YieldExpression {
                node,
                argument: None,
                delegate: false,
            });
        }

        if self.has_previous_token_line_terminator() {
            return Err(ParserError::UnexpectedLineTerminator);
        }

        let is_generator = if self.token_kind() == TokenKind::Multiplication {
            self.advance_any(); // Eat '*' token.

            true
        } else {
            false
        };

        let assignment_expression = self.parse_assignment_expression()?;

        let node = self.end_node(start_index)?;

        self.expect_optional_semicolon_and_advance(); // Eat ';' token.

        Ok(YieldExpression {
            node,
            argument: Some(Box::new(assignment_expression)),
            delegate: is_generator,
        })
    }
}
