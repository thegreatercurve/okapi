use crate::{ast::*, TokenValue};
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl Parser {
    // 15.7 Class Definitions
    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_declaration(&mut self) -> Result<ClassDeclaration, ParserError> {
        // All parts of a ClassDeclaration or a ClassExpression are strict mode code.
        // https://tc39.es/ecma262/#sec-strict-mode-code
        let previous_strict_mode = self.context.strict_mode;
        self.context.strict_mode = true;

        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Class))?;

        let binding_identifier = self.parse_binding_identifier()?;

        let super_class = if self.token_kind() == TokenKind::Keyword(KeywordKind::Extends) {
            Some(self.parse_class_heritage()?)
        } else {
            None
        };

        let class_tail = self.parse_class_tail()?;

        self.context.strict_mode = previous_strict_mode;

        Ok(ClassDeclaration {
            node: self.end_node(start_index)?,
            // TODO Handle default exports properly which don't have an identifier.
            id: Some(binding_identifier),
            super_class,
            body: class_tail,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_expression(&mut self) -> Result<ClassExpression, ParserError> {
        // All parts of a ClassDeclaration or a ClassExpression are strict mode code.
        // https://tc39.es/ecma262/#sec-strict-mode-code
        let previous_strict_mode = self.context.strict_mode;
        self.context.strict_mode = true;

        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Class))?;

        let binding_identifier = if self.token_kind().is_binding_identifier() {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        let class_tail = self.parse_class_tail()?;

        self.context.strict_mode = previous_strict_mode;

        Ok(ClassExpression {
            node: self.end_node(start_index)?,
            id: binding_identifier,
            super_class: None,
            body: class_tail,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassTail
    fn parse_class_tail(&mut self) -> Result<ClassBody, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let class_body = self.parse_class_body()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(ClassBody {
            node: self.end_node(start_index)?,
            body: class_body,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassHeritage
    fn parse_class_heritage(&mut self) -> Result<Expression, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Extends))?;

        self.parse_left_hand_side_expression()
    }

    // https://tc39.es/ecma262/#prod-ClassBody
    fn parse_class_body(&mut self) -> Result<Vec<ClassBodyBody>, ParserError> {
        let mut class_element_list = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            let Some(class_element) = self.parse_class_element(false, None)? else {
                continue;
            };

            class_element_list.push(class_element);
        }

        Ok(class_element_list)
    }

    // https://tc39.es/ecma262/#prod-ClassElement
    fn parse_class_element(
        &mut self,
        is_static: bool,
        start_index: Option<usize>,
    ) -> Result<Option<ClassBodyBody>, ParserError> {
        let start_index = start_index.unwrap_or(self.start_node());

        match (self.token_kind(), self.peek_token_kind()) {
            // Handle `;`.
            (TokenKind::Semicolon, _) => {
                self.advance_any(); // Eat ';' token.

                Ok(None)
            }
            // Handle `ClassStaticBlock > static {`.
            (TokenKind::Keyword(KeywordKind::Static), TokenKind::LeftCurlyBrace) => {
                let static_block = self.parse_static_block()?;

                Ok(Some(ClassBodyBody::StaticBlock(static_block)))
            }
            // Handle `static MethodDefinition`.
            // Handle `static FieldDefinition`.
            (TokenKind::Keyword(KeywordKind::Static), _) => {
                // Handle `static;` where `static` is used as an identifier.
                if self.peek_token_kind() == TokenKind::Semicolon {
                    let field_definition =
                        self.parse_field_definition(start_index, None, false, false)?;

                    return Ok(Some(ClassBodyBody::PropertyDefinition(field_definition)));
                }

                self.advance_any(); // Eat 'static' token.

                self.parse_class_element(true, Some(start_index))
            }
            // Handle `MethodDefinition > get ClassElementName ... ;`.
            (TokenKind::Keyword(KeywordKind::Get), peek_token_kind)
                if peek_token_kind.is_class_element_name() =>
            {
                self.advance_any(); // Eat 'get' token.

                let is_computed = self.token_kind() == TokenKind::LeftSquareBracket;

                let method_definition_key = self.parse_class_element_name()?;

                let function_expression = self.parse_method_definition_getter_body()?;

                let method_definition = self.parse_method_definition(
                    start_index,
                    Some(method_definition_key),
                    Some(function_expression),
                    MethodDefinitionKind::Get,
                    is_static,
                    is_computed,
                )?;

                Ok(Some(ClassBodyBody::MethodDefinition(method_definition)))
            }
            // Handle `MethodDefinition > set ClassElementName ... ;`.
            (TokenKind::Keyword(KeywordKind::Set), peek_token_kind)
                if peek_token_kind.is_class_element_name() =>
            {
                self.advance_any(); // Eat 'set' token.

                let is_computed = self.token_kind() == TokenKind::LeftSquareBracket;

                let method_definition_key = self.parse_class_element_name()?;

                let function_expression = self.parse_method_definition_setter_body()?;

                let method_definition = self.parse_method_definition(
                    start_index,
                    Some(method_definition_key),
                    Some(function_expression),
                    MethodDefinitionKind::Set,
                    is_static,
                    is_computed,
                )?;

                Ok(Some(ClassBodyBody::MethodDefinition(method_definition)))
            }
            // Handle `MethodDefinition > GeneratorMethod > * ClassElementName`.
            (TokenKind::Multiplication, peek_token_kind)
                if peek_token_kind.is_class_element_name() =>
            {
                let is_computed = self.peek_token_kind() == TokenKind::LeftSquareBracket;

                let generator_method_definition = self.parse_generator_method(
                    start_index,
                    is_computed,
                    false,
                    is_static,
                    MethodDefinitionKind::Method,
                )?;

                Ok(Some(ClassBodyBody::MethodDefinition(
                    generator_method_definition,
                )))
            }
            // Handle `MethodDefinition > AsyncMethod > async [no LineTerminator here] ClassElementName`.
            (TokenKind::Keyword(KeywordKind::Async), peek_token_kind)
                if !self.has_current_token_line_terminator()
                    && peek_token_kind.is_class_element_name() =>
            {
                let is_computed = self.peek_token_kind() == TokenKind::LeftSquareBracket;

                let async_method_definition = self.parse_async_method(
                    start_index,
                    is_computed,
                    is_static,
                    MethodDefinitionKind::Method,
                )?;

                Ok(Some(ClassBodyBody::MethodDefinition(
                    async_method_definition,
                )))
            }
            // Handle `MethodDefinition > AsyncGeneratorMethod > async [no LineTerminator here] * ClassElementName`.
            (TokenKind::Keyword(KeywordKind::Async), TokenKind::Multiplication)
                if !self.has_current_token_line_terminator() =>
            {
                self.advance_any(); // Eat 'async' token.

                let is_computed = self.peek_token_kind() == TokenKind::LeftSquareBracket;

                let async_generator_method_definition = self.parse_generator_method(
                    start_index,
                    is_computed,
                    true,
                    is_static,
                    MethodDefinitionKind::Method,
                )?;

                Ok(Some(ClassBodyBody::MethodDefinition(
                    async_generator_method_definition,
                )))
            }
            // Handle `MethodDefinition > ClassElementName ( ... ;`.
            // Handle `FieldDefinition > ClassElementName;`.
            (token_kind, _) if token_kind.is_class_element_name() => {
                let is_computed = token_kind == TokenKind::LeftSquareBracket;

                let constructor_token_value = TokenValue::String {
                    raw: "constructor".to_string(),
                    value: "constructor".to_string(),
                };

                let method_definition_kind = if self.token_value() == constructor_token_value {
                    MethodDefinitionKind::Constructor
                } else {
                    MethodDefinitionKind::Method
                };

                let method_definition_key = self.parse_class_element_name()?;

                match self.token_kind() {
                    TokenKind::LeftParenthesis => {
                        let function_expression = self.parse_method_definition_method_body()?;

                        let method_definition = self.parse_method_definition(
                            start_index,
                            Some(method_definition_key),
                            Some(function_expression),
                            method_definition_kind,
                            is_static,
                            is_computed,
                        )?;

                        Ok(Some(ClassBodyBody::MethodDefinition(method_definition)))
                    }
                    TokenKind::Assignment | TokenKind::Semicolon => {
                        let field_definition = self.parse_field_definition(
                            start_index,
                            Some(method_definition_key),
                            is_static,
                            is_computed,
                        )?;

                        Ok(Some(ClassBodyBody::PropertyDefinition(field_definition)))
                    }
                    _ => Err(self.unexpected_current_token_kind()),
                }
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-FieldDefinition
    fn parse_field_definition(
        &mut self,
        start_index: usize,
        class_element_name: Option<PropertyDefinitionKey>,
        is_static: bool,
        is_computed: bool,
    ) -> Result<PropertyDefinition, ParserError> {
        let class_element_name = match class_element_name {
            Some(class_element_name) => class_element_name,
            None => self.parse_class_element_name()?,
        };

        let optional_assignment_expression = if self.token_kind() == TokenKind::Assignment {
            self.advance_any(); // Eat  '=' token.

            let assignment_expression = self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_assignment_expression,
            )?;

            Some(assignment_expression)
        } else {
            None
        };

        self.expect_optional_semicolon_and_advance();

        Ok(PropertyDefinition {
            node: self.end_node(start_index)?,
            is_static,
            computed: is_computed,
            key: Some(class_element_name),
            value: optional_assignment_expression,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassElementName
    pub(crate) fn parse_class_element_name(
        &mut self,
    ) -> Result<PropertyDefinitionKey, ParserError> {
        match self.token_kind() {
            token_kind if token_kind.is_private_identifier() => {
                let private_identifier = self.parse_private_identifier()?;

                Ok(PropertyDefinitionKey::PrivateIdentifier(private_identifier))
            }
            token_kind if token_kind.is_property_name() => {
                let property_name = self.parse_property_name()?;

                Ok(PropertyDefinitionKey::Expression(property_name))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ClassStaticBlock
    fn parse_static_block(&mut self) -> Result<StaticBlock, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Static))?;

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let statement_list = self.parse_statement_list()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(StaticBlock {
            node: self.end_node(start_index)?,
            body: statement_list,
        })
    }
}
