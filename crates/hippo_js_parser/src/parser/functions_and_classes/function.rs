use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 15.2 Function Definitions
    // https://tc39.es/ecma262/#prod-FunctionDeclaration
    pub(crate) fn parse_function_declaration(
        &mut self,
    ) -> Result<FunctionDeclaration, ParserError> {
        let start_index = self.start_node();

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
            asynchronous: false,
            expression: false,
        })
    }

    // https://tc39.es/ecma262/#prod-FunctionExpression
    pub(crate) fn parse_function_expression(&mut self) -> Result<FunctionExpression, ParserError> {
        let start_index = self.start_node();

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
            is_async: false,
        })
    }

    // https://tc39.es/ecma262/#prod-FunctionBody
    pub(crate) fn parse_function_body(&mut self) -> Result<BlockStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let mut body = self.parse_directive_prologue()?;

        while self.token_kind() != TokenKind::RightCurlyBrace {
            body.push(self.parse_statement_list_item()?);
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(BlockStatement {
            node: self.end_node(start_index)?,
            body,
        })
    }

    // https://tc39.es/ecma262/#prod-AsyncMethod
    pub(crate) fn parse_async_method(
        &mut self,
        start_index: usize,
        is_computed: bool,
        is_static: bool,
        kind: MethodDefinitionKind,
    ) -> Result<MethodDefinition, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Async))?;

        let class_element_name = self.parse_class_element_name()?;

        let function_expression_start_index = self.start_node();

        let formal_parameters = self.parse_parenthesized_formal_parameters()?;

        let function_body = self.parse_function_body()?;

        let function_expression = FunctionExpression {
            node: self.end_node(function_expression_start_index)?,
            id: None,
            expression: false,
            generator: false,
            is_async: true,
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

    pub(crate) fn parse_parenthesized_formal_parameters(
        &mut self,
    ) -> Result<Vec<FunctionParameter>, ParserError> {
        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let formal_parameters = self
            .parse_formal_parameters()?
            .into_iter()
            .map(FunctionParameter::try_from)
            .collect::<Result<Vec<FunctionParameter>, ParserError>>()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        Ok(formal_parameters)
    }
}
