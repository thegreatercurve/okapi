use crate::{KeywordKind, Parser, ParserError, TokenKind, TokenValue};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.4 Method Definitions
    // https://tc39.es/ecma262/#sec-method-definitions
    pub(crate) fn parse_method_definition_with_static(
        &mut self,
    ) -> Result<MethodDefinition, ParserError> {
        let mut is_static = false;

        self.start_node();

        if self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::Static) {
            is_static = true;

            self.cursor.advance(); // Eat `static` token.
        }

        let is_computed = self.cursor.current_token_kind() == TokenKind::LeftSquareBracket;

        let mut method_definition_kind = MethodDefinitionKind::Method;

        match self.cursor.current_token_kind() {
            current_token_kind if current_token_kind.is_class_element_name_start() => {
                if self.cursor.current_token_value()
                    == TokenValue::String("constructor".to_string())
                {
                    method_definition_kind = MethodDefinitionKind::Constructor;
                }

                let class_element_name = self.parse_class_element_name()?;

                self.start_node(); // Start function expression node.

                self.expect_and_advance(TokenKind::LeftParenthesis)?;

                // TODO Figure out a better way of handling these typings.
                let formal_parameters = self
                    .parse_formal_parameters()?
                    .into_iter()
                    .map(|parameter| parameter.to_function_parameter())
                    .collect();

                self.expect_and_advance(TokenKind::RightParenthesis)?;

                let function_body = self.parse_function_body()?;

                let function_expression = FunctionExpression {
                    node: self.end_node()?,
                    id: None,
                    expression: false,
                    generator: false,
                    is_async: false,
                    params: formal_parameters,
                    body: function_body,
                };

                Ok(MethodDefinition {
                    node: self.end_node()?,
                    kind: method_definition_kind,
                    value: Some(function_expression),
                    is_static,
                    computed: is_computed,
                    key: Some(class_element_name),
                    decorators: vec![],
                })
            }
            // TokenKind::Keyword(KeywordKind::Get) => {
            //     method_definition_kind = MethodDefinitionKind::Get
            // }
            // TokenKind::Keyword(KeywordKind::Set) => {
            //     method_definition_kind = MethodDefinitionKind::Set
            // }
            _ => {
                // method_definition_kind = MethodDefinitionKind::Method;

                todo!("parse_method_definition_with_static")
            }
        };

        // let class_element_name = self.parse_class_element_name()?;
    }
}
