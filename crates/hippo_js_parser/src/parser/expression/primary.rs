use crate::{KeywordKind, Parser, ParserError, TokenKind, TokenValue};
use hippo_estree::*;

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 13.2 Primary Expression
    // https://tc39.es/ecma262/#prod-PrimaryExpression
    pub(crate) fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete. Handle template literals and Regex.
        let token_kind = self.cursor.current_token_kind();

        match token_kind {
            TokenKind::Keyword(KeywordKind::This) => self.parse_this_expression(),
            TokenKind::Identifier => self.parse_identifier_reference(),
            TokenKind::StringLiteral
            | TokenKind::NumberLiteral
            | TokenKind::Keyword(KeywordKind::Null)
            | TokenKind::Keyword(KeywordKind::True)
            | TokenKind::Keyword(KeywordKind::False) => self.parse_literal(),
            TokenKind::LeftSquareBracket => self.parse_array_literal(),
            TokenKind::LeftCurlyBrace => self.parse_object_literal(),
            TokenKind::Keyword(KeywordKind::Function) => {
                if self.cursor.peek_token_kind() == TokenKind::Multiplication {
                    self.parse_generator_expression()
                } else {
                    self.parse_function_expression()
                }
            }
            TokenKind::Keyword(KeywordKind::Class) => self.parse_class_expression(),
            TokenKind::Keyword(KeywordKind::Async) => {
                self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

                if self.cursor.peek_token_kind() == TokenKind::Multiplication {
                    self.parse_async_generator_expression()
                } else {
                    self.parse_async_function_expression()
                }
            }
            TokenKind::LeftParenthesis => {
                self.parse_cover_parenthesized_expression_and_arrow_parameter_list()
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    fn parse_this_expression(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::This))?;

        Ok(Expression::This(ThisExpression {
            node: self.end_node()?,
        }))
    }

    // https://tc39.es/ecma262/#prod-CoverParenthesizedExpressionAndArrowParameterList
    pub(crate) fn parse_cover_parenthesized_expression_and_arrow_parameter_list(
        &mut self,
    ) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let expression = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        Ok(expression)
    }

    // https://tc39.es/ecma262/#prod-Literal
    fn parse_literal(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        let token_value = self.cursor.current_token_value();

        let node = self.end_node()?;

        let literal = match self.cursor.current_token_kind() {
            TokenKind::StringLiteral => {
                let raw_value = match token_value {
                    TokenValue::String(value) => value,
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(Expression::Literal(Literal {
                    node,
                    value: LiteralValue::String(raw_value.clone()),
                    raw: raw_value,
                }))
            }
            TokenKind::NumberLiteral => {
                let (raw, value) = match token_value {
                    TokenValue::Number { raw, value } => (raw, value),
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(Expression::Literal(Literal {
                    node,
                    value: LiteralValue::Number(value),
                    raw: raw,
                }))
            }
            TokenKind::Keyword(KeywordKind::Null) => Ok(Expression::Literal(Literal {
                node,
                value: LiteralValue::Null,
                raw: "null".to_string(),
            })),
            TokenKind::Keyword(KeywordKind::True) => Ok(Expression::Literal(Literal {
                node,
                value: LiteralValue::Boolean(true),
                raw: "true".to_string(),
            })),
            TokenKind::Keyword(KeywordKind::False) => Ok(Expression::Literal(Literal {
                node,
                value: LiteralValue::Boolean(false),
                raw: "false".to_string(),
            })),
            _ => Err(self.unexpected_current_token_kind()),
        };

        self.expect_one_of_and_advance(vec![
            TokenKind::StringLiteral,
            TokenKind::NumberLiteral,
            TokenKind::Keyword(KeywordKind::Null),
            TokenKind::Keyword(KeywordKind::True),
            TokenKind::Keyword(KeywordKind::False),
        ])?;

        literal
    }

    // https://tc39.es/ecma262/#prod-ArrayLiteral
    fn parse_array_literal(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftSquareBracket)?;

        let element_list = self.parse_element_list()?;

        self.expect_and_advance(TokenKind::RightSquareBracket)?;

        Ok(Expression::Array(ArrayExpression {
            node: self.end_node()?,
            elements: element_list,
        }))
    }

    // https://tc39.es/ecma262/#prod-ElementList
    fn parse_element_list(&mut self) -> Result<Vec<Option<MemberExpressionElements>>, ParserError> {
        let mut elements = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightSquareBracket {
            match self.cursor.current_token_kind() {
                TokenKind::Comma => {
                    self.cursor.advance(); // Eat the , token.

                    elements.push(None);

                    continue;
                }
                TokenKind::Ellipsis => {
                    self.start_node();

                    self.cursor.advance(); // Eat the ... token.

                    let assigment_expression: Expression = self.parse_assignment_expression()?;

                    elements.push(Some(MemberExpressionElements::SpreadElement(
                        SpreadElement {
                            node: self.end_node()?,
                            argument: assigment_expression,
                        },
                    )));
                }
                _ => {
                    let assigment_expression: Expression = self.parse_assignment_expression()?;

                    elements.push(Some(MemberExpressionElements::Expression(
                        assigment_expression,
                    )));
                }
            };

            if self.cursor.current_token_kind() == TokenKind::RightSquareBracket {
                break;
            }

            self.expect_and_advance(TokenKind::Comma)?;
        }

        Ok(elements)
    }

    // https://tc39.es/ecma262/#prod-ObjectLiteral
    fn parse_object_literal(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let properties = self.parse_property_definition_list()?;

        let node = self.end_node()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Expression::Object(ObjectExpression { node, properties }))
    }

    // https://tc39.es/ecma262/#prod-PropertyDefinitionList
    fn parse_property_definition_list(
        &mut self,
    ) -> Result<Vec<ObjectExpressionProperties>, ParserError> {
        let mut properties = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightCurlyBrace {
            let property = self.parse_property_definition()?;

            properties.push(property);

            if self.cursor.current_token_kind() == TokenKind::RightCurlyBrace {
                break;
            }

            self.expect_and_advance(TokenKind::Comma)?;
        }

        Ok(properties)
    }

    // https://tc39.es/ecma262/#prod-PropertyDefinition
    fn parse_property_definition(&mut self) -> Result<ObjectExpressionProperties, ParserError> {
        self.start_node();

        match self.cursor.current_token_kind() {
            TokenKind::Identifier
            | TokenKind::StringLiteral
            | TokenKind::NumberLiteral
            | TokenKind::LeftSquareBracket => {
                let property_key = self.parse_property_name()?;

                let mut shorthand = match property_key {
                    PropertyKey::Identifier(_) => true,
                    _ => false,
                };
                let computed = match property_key {
                    PropertyKey::Expression(_) => true,
                    _ => false,
                };
                let mut method = false;
                // TODO Add support for `get` and `set`.
                let kind = PropertyKind::Init;

                let value;

                match self.cursor.current_token_kind() {
                    TokenKind::Colon => {
                        shorthand = false;

                        self.expect_and_advance(TokenKind::Colon)?;

                        let assignment_expression = self.parse_assignment_expression()?;

                        value = Box::new(assignment_expression);
                    }
                    TokenKind::Equal => {
                        shorthand = false;

                        let assignment_expression = self.parse_assignment_expression()?;

                        value = Box::new(assignment_expression);
                    }
                    TokenKind::LeftParenthesis => {
                        method = true;

                        let method_definition = self.parse_method_definition()?;

                        value = Box::new(method_definition);
                    }
                    _ => {
                        shorthand = true;

                        match &property_key {
                            PropertyKey::Identifier(identifier) => {
                                value = Box::new(Expression::Identifier(identifier.clone()));
                            }
                            _ => return Err(ParserError::InvalidPropertyKey),
                        };
                    }
                }

                Ok(ObjectExpressionProperties::Property(Property {
                    method,
                    shorthand,
                    computed,
                    key: property_key,
                    node: self.end_node()?,
                    value,
                    kind,
                }))
            }
            TokenKind::Ellipsis => {
                self.cursor.advance(); // Eat the ... token.

                let assigment_expression: Expression = self.parse_assignment_expression()?;

                Ok(ObjectExpressionProperties::SpreadElement(SpreadElement {
                    node: self.end_node()?,
                    argument: assigment_expression,
                }))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-PropertyName
    fn parse_property_name(&mut self) -> Result<PropertyKey, ParserError> {
        self.start_node();

        let token_value = self.cursor.current_token_value();

        match self.cursor.current_token_kind() {
            TokenKind::Identifier => {
                self.cursor.advance(); // Eat the identifier token.

                let name = match token_value {
                    TokenValue::String(value) => value,
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(PropertyKey::Identifier(Identifier {
                    node: self.end_node()?,
                    name,
                }))
            }
            TokenKind::StringLiteral => {
                self.cursor.advance(); // Eat the string literal token.

                let raw_value = match token_value {
                    TokenValue::String(value) => value,
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(PropertyKey::Literal(Literal {
                    node: self.end_node()?,
                    value: LiteralValue::String(raw_value.clone()),
                    raw: format!("\"{}\"", raw_value.to_string()),
                }))
            }
            TokenKind::NumberLiteral => {
                self.cursor.advance(); // Eat the number literal token.

                let (raw, value) = match token_value {
                    TokenValue::Number { raw, value } => (raw, value),
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(PropertyKey::Literal(Literal {
                    node: self.end_node()?,
                    value: LiteralValue::Number(value.clone()),
                    raw,
                }))
            }
            TokenKind::LeftSquareBracket => self.parse_computed_property_name(),
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-PropertyName
    fn parse_computed_property_name(&mut self) -> Result<PropertyKey, ParserError> {
        self.expect_and_advance(TokenKind::LeftSquareBracket)?;

        let assignment_expression = self.parse_assignment_expression()?;

        self.expect_and_advance(TokenKind::RightSquareBracket)?;

        Ok(PropertyKey::Expression(assignment_expression))
    }
}