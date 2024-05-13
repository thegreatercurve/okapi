use crate::{ast::*, GoalSymbol};

use crate::{KeywordKind, Parser, ParserError, TokenKind, TokenValue};

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 13.2 Primary Expression
    // https://tc39.es/ecma262/#prod-PrimaryExpression
    pub(crate) fn parse_primary_expression(&mut self) -> Result<Expression, ParserError> {
        match self.token_kind() {
            TokenKind::Keyword(KeywordKind::This) => {
                Ok(Expression::This(self.parse_this_expression()?))
            }
            TokenKind::Keyword(KeywordKind::Async) => {
                match (self.peek_token_kind(), self.peek_nth_kind(2)) {
                    (TokenKind::Keyword(KeywordKind::Function), TokenKind::Multiplication) => Ok(
                        Expression::Function(self.parse_async_generator_expression()?),
                    ),
                    (TokenKind::Keyword(KeywordKind::Function), _) => Ok(Expression::Function(
                        self.parse_async_function_expression()?,
                    )),
                    _ => self.parse_identifier_reference(),
                }
            }
            token_kind if token_kind.is_identifier_reference() => self.parse_identifier_reference(),
            TokenKind::StringLiteral
            | TokenKind::NumberLiteral
            | TokenKind::Keyword(KeywordKind::Null)
            | TokenKind::Keyword(KeywordKind::True)
            | TokenKind::Keyword(KeywordKind::False) => {
                Ok(Expression::Literal(self.parse_literal()?))
            }
            TokenKind::LeftSquareBracket => Ok(Expression::Array(self.parse_array_literal()?)),
            TokenKind::LeftCurlyBrace => Ok(Expression::Object(self.parse_object_literal()?)),
            TokenKind::Keyword(KeywordKind::Function) => {
                if self.peek_token_kind() == TokenKind::Multiplication {
                    Ok(Expression::Function(self.parse_generator_expression()?))
                } else {
                    Ok(Expression::Function(self.parse_function_expression()?))
                }
            }
            TokenKind::Keyword(KeywordKind::Class) => {
                Ok(Expression::Class(self.parse_class_expression()?))
            }
            TokenKind::LeftParenthesis => self.parse_cover_parenthesized_expression(),
            TokenKind::Division => Ok(Expression::RegExpLiteral(
                self.parse_regular_expression_literal()?,
            )),
            token_kind if token_kind.is_template_part() => {
                Ok(Expression::TemplateLiteral(self.parse_template_literal()?))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-CoverParenthesizedExpressionAndArrowParameterList
    // https://tc39.es/ecma262/#prod-ParenthesizedExpression
    pub(crate) fn parse_cover_parenthesized_expression(
        &mut self,
    ) -> Result<Expression, ParserError> {
        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        // Ensure that parenthezied expression in a `for` statement parses the `in` keyword as a relational expression.
        let previous_allow_in = self.context.allow_in;
        self.context.allow_in = true;

        let expression = self.parse_expression()?;

        self.context.allow_in = previous_allow_in;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        Ok(expression)
    }

    // 13.2.1 The this Keyword
    // https://tc39.es/ecma262/#sec-this-keyword
    fn parse_this_expression(&mut self) -> Result<ThisExpression, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::This))?;

        Ok(ThisExpression {
            node: self.end_node(start_index)?,
        })
    }

    // 13.2.3 Literals
    // https://tc39.es/ecma262/#prod-Literal
    pub(crate) fn parse_literal(&mut self) -> Result<Literal, ParserError> {
        let start_index = self.start_node();

        let token_value = self.token_value();

        let token_kind = self.token_kind();

        self.expect_one_of_and_advance(vec![
            TokenKind::StringLiteral,
            TokenKind::NumberLiteral,
            TokenKind::Keyword(KeywordKind::Null),
            TokenKind::Keyword(KeywordKind::True),
            TokenKind::Keyword(KeywordKind::False),
        ])?;

        let node = self.end_node(start_index)?;

        let literal = match token_kind {
            TokenKind::StringLiteral => Ok(self.parse_string_literal(token_value, node)?),
            TokenKind::NumberLiteral => Ok(self.parse_number_literal(token_value, node)?),
            TokenKind::Keyword(KeywordKind::Null) => Ok(Literal {
                node,
                value: LiteralValue::Null,
                raw: "null".to_string(),
            }),
            TokenKind::Keyword(KeywordKind::True) => Ok(Literal {
                node,
                value: LiteralValue::Boolean(true),
                raw: "true".to_string(),
            }),
            TokenKind::Keyword(KeywordKind::False) => Ok(Literal {
                node,
                value: LiteralValue::Boolean(false),
                raw: "false".to_string(),
            }),
            _ => Err(self.unexpected_current_token_kind()),
        };

        literal
    }

    // https://tc39.es/ecma262/#prod-NumberLiteral
    pub(crate) fn parse_number_literal(
        &mut self,
        token_value: TokenValue,
        node: Node,
    ) -> Result<Literal, ParserError> {
        let (raw, value) = match token_value {
            TokenValue::Number { raw, value } => (raw, value),
            _ => return Err(self.unexpected_current_token_value()),
        };

        Ok(Literal {
            node,
            value: LiteralValue::Number(value),
            raw: raw,
        })
    }

    // https://tc39.es/ecma262/#prod-StringLiteral
    pub(crate) fn parse_string_literal(
        &mut self,
        token_value: TokenValue,
        node: Node,
    ) -> Result<Literal, ParserError> {
        let (raw, value) = match token_value {
            TokenValue::String { raw, value } => (raw, value),
            _ => return Err(self.unexpected_current_token_value()),
        };

        Ok(Literal {
            node,
            value: LiteralValue::String(value),
            raw,
        })
    }

    // 13.2.4 Array Initializer
    // https://tc39.es/ecma262/#prod-ArrayLiteral
    fn parse_array_literal(&mut self) -> Result<ArrayExpression, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftSquareBracket)?;

        // Ensure that parenthezied expression in a `for` statement parses the `in` keyword as a relational expression.
        let previous_allow_in = self.context.allow_in;
        self.context.allow_in = true;

        let element_list = self.parse_element_list()?;

        self.context.allow_in = previous_allow_in;

        self.expect_and_advance(TokenKind::RightSquareBracket)?;

        Ok(ArrayExpression {
            node: self.end_node(start_index)?,
            elements: element_list,
        })
    }

    // https://tc39.es/ecma262/#prod-ElementList
    fn parse_element_list(&mut self) -> Result<Vec<Option<ArrayExpressionElement>>, ParserError> {
        let mut elements = vec![];

        while self.token_kind() != TokenKind::RightSquareBracket {
            match self.token_kind() {
                TokenKind::Comma => {
                    self.advance_any(); // Eat ',' token.

                    elements.push(None);

                    continue;
                }
                TokenKind::Ellipsis => {
                    let start_index = self.start_node();

                    self.advance_any(); // Eat '...' token.

                    let assigment_expression = self.parse_assignment_expression()?;

                    elements.push(Some(ArrayExpressionElement::SpreadElement(SpreadElement {
                        node: self.end_node(start_index)?,
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

            if self.token_kind() == TokenKind::RightSquareBracket {
                break;
            }

            self.expect_and_advance(TokenKind::Comma)?;
        }

        Ok(elements)
    }

    // 13.2.5 Object Initializer
    // https://tc39.es/ecma262/#prod-ObjectLiteral
    fn parse_object_literal(&mut self) -> Result<ObjectExpression, ParserError> {
        let start_index = self.start_node();

        // Template literal middles or tails are not permitted within an object literal.
        let previous_goal_symbol = self.cursor.lexer.goal_symbol.clone();
        self.cursor.lexer.goal_symbol = GoalSymbol::InputElementDiv;

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        // Ensure that parenthezied expression in a `for` statement parses the `in` keyword as a relational expression.
        let previous_allow_in = self.context.allow_in;
        self.context.allow_in = true;

        let properties = self.parse_property_definition_list()?;

        self.context.allow_in = previous_allow_in;

        self.cursor.lexer.goal_symbol = previous_goal_symbol.clone();

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(ObjectExpression {
            node: self.end_node(start_index)?,
            properties,
        })
    }

    // https://tc39.es/ecma262/#prod-PropertyDefinitionList
    fn parse_property_definition_list(
        &mut self,
    ) -> Result<Vec<ObjectExpressionProperty>, ParserError> {
        let mut properties = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            let property = self.parse_property_definition()?;

            properties.push(property);

            if self.token_kind() == TokenKind::RightCurlyBrace {
                break;
            }

            self.expect_and_advance(TokenKind::Comma)?;
        }

        Ok(properties)
    }

    // https://tc39.es/ecma262/#prod-PropertyDefinition
    fn parse_property_definition(&mut self) -> Result<ObjectExpressionProperty, ParserError> {
        if self.token_kind() == TokenKind::Ellipsis {
            let start_index = self.start_node();

            self.advance_any(); // Eat '...' token.

            let assigment_expression: Expression = self.parse_assignment_expression()?;

            return Ok(ObjectExpressionProperty::SpreadElement(SpreadElement {
                node: self.end_node(start_index)?,
                argument: assigment_expression,
            }));
        }

        // TODO This could probably be a lot better to handle complex async and generator methods combinations. Duplicate of parse_method_definition.
        let mut is_method_definition = match self.token_kind() {
            TokenKind::Keyword(KeywordKind::Async)
            | TokenKind::Keyword(KeywordKind::Get)
            | TokenKind::Keyword(KeywordKind::Set) => {
                let is_generator = self.peek_token_kind() == TokenKind::Multiplication;
                let is_class_element_name = self.peek_token_kind().is_class_element_name_start();
                let is_keyword_as_class_element_name =
                    self.peek_token_kind() == TokenKind::LeftParenthesis;

                is_generator || is_class_element_name || is_keyword_as_class_element_name
            }
            token_kind if token_kind.is_class_element_name_start() => {
                self.peek_token_kind() == TokenKind::LeftParenthesis
            }
            TokenKind::Multiplication => true,
            _ => false,
        };

        let method_definition_kind = self.parse_method_definition_kind();

        let start_index = self.start_node();

        if is_method_definition {
            let method_definition = self.parse_method_definition(
                start_index,
                None,
                method_definition_kind,
                false,
                false,
            )?;

            return Ok(ObjectExpressionProperty::Property(Property::try_from(
                method_definition,
            )?));
        };

        let mut is_computed = self.token_kind() == TokenKind::LeftSquareBracket;

        if self.token_kind().is_property_name() {
            let property_name = self.parse_property_name()?;

            let mut is_shorthand = false;

            let value = match self.token_kind() {
                TokenKind::LeftParenthesis => {
                    is_method_definition = true;
                    is_computed = true; // Must be a computed method name.

                    let function_expression = self.parse_method_definition_method_body()?;

                    PropertyValue::Expression(Expression::Function(function_expression))
                }
                TokenKind::Colon => {
                    self.expect_and_advance(TokenKind::Colon)?;

                    let assignment_expression = self.parse_assignment_expression()?;

                    PropertyValue::Expression(assignment_expression)
                }
                TokenKind::Assignment => {
                    self.expect_and_advance(TokenKind::Assignment)?;

                    let assignment_expression = self.parse_assignment_expression()?;

                    PropertyValue::Expression(assignment_expression)
                }
                _ => {
                    is_shorthand = true;

                    match &property_name {
                        Expression::Identifier(identifier) => {
                            PropertyValue::Expression(Expression::Identifier(identifier.clone()))
                        }
                        _ => return Err(ParserError::InvalidPropertyKey),
                    }
                }
            };

            return Ok(ObjectExpressionProperty::Property(Property {
                method: is_method_definition,
                shorthand: is_shorthand,
                computed: is_computed,
                key: property_name,
                node: self.end_node(start_index)?,
                value,
                kind: PropertyKind::Init,
            }));
        }

        Err(self.unexpected_current_token_kind())
    }

    // https://tc39.es/ecma262/#prod-PropertyName
    pub(crate) fn parse_property_name(&mut self) -> Result<Expression, ParserError> {
        let token_value = self.token_value();

        match self.token_kind() {
            token_kind if token_kind.is_identifier_name() => {
                let start_index = self.start_node();

                self.advance_any(); // Eat identifier token.

                let name = String::from(token_value);

                Ok(Expression::Identifier(Identifier {
                    node: self.end_node(start_index)?,
                    name,
                }))
            }
            TokenKind::StringLiteral => {
                let start_index = self.start_node();

                self.advance_any(); // Eat string literal token.

                let (raw, value) = match token_value {
                    TokenValue::String { raw, value } => (raw, value),
                    _ => return Err(self.unexpected_current_token_value()),
                };

                Ok(Expression::Literal(Literal {
                    node: self.end_node(start_index)?,
                    value: LiteralValue::String(value),
                    raw,
                }))
            }
            TokenKind::NumberLiteral => {
                let start_index = self.start_node();

                self.advance_any(); // Eat number literal token.

                let (raw, value) = match token_value {
                    TokenValue::Number { raw, value } => (raw, value),
                    _ => return Err(self.unexpected_current_token_value()),
                };

                Ok(Expression::Literal(Literal {
                    node: self.end_node(start_index)?,
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

    // 13.2.6 Regular Expression Literals
    // https://tc39.es/ecma262/#sec-literals-regular-expression-literals
    fn parse_regular_expression_literal(&mut self) -> Result<RegExpLiteral, ParserError> {
        //  In the context of a primary expression, the lexer shoudl have the goal symbol of `InputElementRegExp` or `InputElementRegExpOrTemplateTail` which means that division tokens should be reparsed as potential RegexLiteral.
        // https://tc39.es/ecma262/#prod-InputElementRegExpOrTemplateTail
        let start_index = self.start_node();

        self.cursor.lexer.goal_symbol = GoalSymbol::InputElementRegExp;

        self.cursor.rewind();

        let TokenValue::RegularExpression { pattern, flags } = self.token_value() else {
            return Err(self.unexpected_current_token_value());
        };

        self.expect_and_advance(TokenKind::RegularExpressionLiteral)?;

        let node = self.end_node(start_index)?;

        self.cursor.lexer.goal_symbol = GoalSymbol::InputElementDiv;

        let raw_value = format!("/{:}/{:}", pattern, flags);

        Ok(RegExpLiteral {
            node,
            regex: Regex { pattern, flags },
            value: LiteralValue::Regex {},
            raw: raw_value,
        })
    }

    // 13.2.7 Template Literals
    // https://tc39.es/ecma262/#sec-template-literals
    pub(crate) fn parse_template_literal(&mut self) -> Result<TemplateLiteral, ParserError> {
        let start_index = self.start_node();

        let mut expressions = vec![];
        let mut quasis = vec![];

        if self.token_kind() == TokenKind::TemplateNoSubstitution {
            quasis.push(self.parse_template_element(true, 1, 1)?)
        } else {
            while self.token_kind().is_template_part() {
                match self.token_kind() {
                    TokenKind::TemplateHead => {
                        quasis.push(self.parse_template_element(false, 1, 2)?);

                        expressions.push(self.parse_expression()?);
                    }
                    TokenKind::TemplateMiddle => {
                        quasis.push(self.parse_template_element(false, 1, 2)?);

                        expressions.push(self.parse_expression()?);
                    }
                    TokenKind::TemplateTail => {
                        quasis.push(self.parse_template_element(true, 1, 1)?)
                    }
                    _ => return Err(self.unexpected_current_token_kind()),
                }
            }
        }

        let node = self.end_node(start_index)?;

        Ok(TemplateLiteral {
            node,
            expressions,
            quasis,
        })
    }

    fn parse_template_element(
        &mut self,
        has_tail: bool,
        start_offset: usize,
        end_offset: usize,
    ) -> Result<TemplateElement, ParserError> {
        let start_index = self.start_node();

        let TokenValue::Template { cooked, raw } = self.token_value() else {
            return Err(self.unexpected_current_token_value());
        };

        self.expect_one_of_and_advance(vec![
            TokenKind::TemplateNoSubstitution,
            TokenKind::TemplateHead,
            TokenKind::TemplateMiddle,
            TokenKind::TemplateTail,
        ])?;

        let mut node = self.end_node(start_index)?;

        node.loc.start += start_offset;
        node.loc.end -= end_offset;

        Ok(TemplateElement {
            node,
            value: TemplateElementValue { cooked, raw },
            tail: has_tail,
        })
    }
}
