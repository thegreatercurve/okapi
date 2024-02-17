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

    // 13.2.1 The this Keyword
    // https://tc39.es/ecma262/#sec-this-keyword
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

    // 13.2.3 Literals
    // https://tc39.es/ecma262/#prod-Literal
    pub(crate) fn parse_literal(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        let token_value = self.cursor.current_token_value();

        let current_token_kind = self.cursor.current_token_kind();

        self.expect_one_of_and_advance(vec![
            TokenKind::StringLiteral,
            TokenKind::NumberLiteral,
            TokenKind::Keyword(KeywordKind::Null),
            TokenKind::Keyword(KeywordKind::True),
            TokenKind::Keyword(KeywordKind::False),
        ])?;

        let node = self.end_node()?;

        let literal = match current_token_kind {
            TokenKind::StringLiteral => Ok(Expression::Literal(
                self.parse_string_literal(token_value, node)?,
            )),
            TokenKind::NumberLiteral => Ok(Expression::Literal(
                self.parse_number_literal(token_value, node)?,
            )),
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

        literal
    }

    // https://tc39.es/ecma262/#prod-StringLiteral
    pub(crate) fn parse_string_literal(
        &mut self,
        token_value: TokenValue,
        node: Node,
    ) -> Result<Literal, ParserError> {
        let raw_value = match token_value {
            TokenValue::String(value) => value,
            _ => return Err(ParserError::UnexpectedTokenValue),
        };

        Ok(Literal {
            node,
            value: LiteralValue::String(raw_value.clone()),
            raw: format!(r#""{:}""#, raw_value),
        })
    }

    // https://tc39.es/ecma262/#prod-NumberLiteral
    pub(crate) fn parse_number_literal(
        &mut self,
        token_value: TokenValue,
        node: Node,
    ) -> Result<Literal, ParserError> {
        let (raw, value) = match token_value {
            TokenValue::Number { raw, value } => (raw, value),
            _ => return Err(ParserError::UnexpectedTokenValue),
        };

        Ok(Literal {
            node,
            value: LiteralValue::Number(value),
            raw: raw,
        })
    }

    // 13.2.4 Array Initializer
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
    fn parse_element_list(&mut self) -> Result<Vec<Option<ArrayExpressionElement>>, ParserError> {
        let mut elements = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightSquareBracket {
            match self.cursor.current_token_kind() {
                TokenKind::Comma => {
                    self.cursor.advance(); // Eat ',' token.

                    elements.push(None);

                    continue;
                }
                TokenKind::Ellipsis => {
                    self.start_node();

                    self.cursor.advance(); // Eat '...' token.

                    let assigment_expression = self.parse_assignment_expression()?;

                    elements.push(Some(ArrayExpressionElement::SpreadElement(SpreadElement {
                        node: self.end_node()?,
                        argument: assigment_expression,
                    })));
                }
                _ => {
                    let assigment_expression = self.parse_assignment_expression()?;

                    elements.push(Some(ArrayExpressionElement::Expression(
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

    // 13.2.5 Object Initializer
    // https://tc39.es/ecma262/#prod-ObjectLiteral
    fn parse_object_literal(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let properties = self.parse_property_definition_list()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Expression::Object(ObjectExpression {
            node: self.end_node()?,
            properties,
        }))
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
        match self.cursor.current_token_kind() {
            TokenKind::Identifier
            | TokenKind::StringLiteral
            | TokenKind::NumberLiteral
            | TokenKind::LeftSquareBracket => {
                self.start_node();

                let property_name = self.parse_property_name()?;

                let mut _shorthand = matches!(property_name, Expression::Identifier(_));

                let computed = !matches!(
                    property_name,
                    Expression::Identifier(_) | Expression::Literal(_)
                );

                let mut _method = false;
                // TODO Add support for `get` and `set`.
                let kind = PropertyKind::Init;

                let value;

                match self.cursor.current_token_kind() {
                    TokenKind::Colon => {
                        _shorthand = false;

                        self.expect_and_advance(TokenKind::Colon)?;

                        let assignment_expression = self.parse_assignment_expression()?;

                        value = PropertyValue::Expression(assignment_expression);
                    }
                    TokenKind::Equal => {
                        _shorthand = false;

                        let assignment_expression = self.parse_assignment_expression()?;

                        value = PropertyValue::Expression(assignment_expression);
                    }
                    // TokenKind::LeftParenthesis => {
                    //     method = true;

                    //     let method_definition = self.parse_method_definition()?;

                    //     value = PropertyValue::Expression(method_definition);
                    // }
                    _ => {
                        _shorthand = true;

                        match &property_name {
                            Expression::Identifier(identifier) => {
                                value = PropertyValue::Expression(Expression::Identifier(
                                    identifier.clone(),
                                ));
                            }
                            _ => return Err(ParserError::InvalidPropertyKey),
                        };
                    }
                }

                Ok(ObjectExpressionProperties::Property(Property {
                    method: _method,
                    shorthand: _shorthand,
                    computed,
                    key: property_name,
                    node: self.end_node()?,
                    value,
                    kind,
                }))
            }
            TokenKind::Ellipsis => {
                self.start_node();

                self.cursor.advance(); // Eat '..'. token.

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
    pub(crate) fn parse_property_name(&mut self) -> Result<Expression, ParserError> {
        let token_value = self.cursor.current_token_value();

        match self.cursor.current_token_kind() {
            TokenKind::Identifier => {
                self.start_node();

                self.cursor.advance(); // Eat identifier token.

                let name = match token_value {
                    TokenValue::String(value) => value,
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(Expression::Identifier(Identifier {
                    node: self.end_node()?,
                    name,
                }))
            }
            TokenKind::StringLiteral => {
                self.start_node();

                self.cursor.advance(); // Eat string literal token.

                let raw_value = match token_value {
                    TokenValue::String(value) => value,
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(Expression::Literal(Literal {
                    node: self.end_node()?,
                    value: LiteralValue::String(raw_value.clone()),
                    raw: format!("\"{}\"", raw_value.to_string()),
                }))
            }
            TokenKind::NumberLiteral => {
                self.start_node();

                self.cursor.advance(); // Eat number literal token.

                let (raw, value) = match token_value {
                    TokenValue::Number { raw, value } => (raw, value),
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(Expression::Literal(Literal {
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
    fn parse_computed_property_name(&mut self) -> Result<Expression, ParserError> {
        self.expect_and_advance(TokenKind::LeftSquareBracket)?;

        let assignment_expression = self.parse_assignment_expression()?;

        self.expect_and_advance(TokenKind::RightSquareBracket)?;

        Ok(assignment_expression)
    }

    // https://tc39.es/ecma262/#prod-Initializer
    fn parse_initializer(&mut self) -> Result<(), ParserError> {
        todo!("parse_initializer")
    }
}
