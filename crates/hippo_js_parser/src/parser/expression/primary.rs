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
                    _ => Ok(Expression::Identifier(self.parse_identifier_reference()?)),
                }
            }
            token_kind if token_kind.is_identifier_reference() => {
                Ok(Expression::Identifier(self.parse_identifier_reference()?))
            }
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
            TokenKind::Division | TokenKind::DivisionAssignment => Ok(Expression::RegExpLiteral(
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

        let expression = self.with_params(
            self.params.clone().add_allow_in(false),
            Self::parse_expression,
        )?;

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

        match token_kind {
            TokenKind::StringLiteral => self.parse_string_literal(token_value, node),
            TokenKind::NumberLiteral => self.parse_number_literal(token_value, node),
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
        }
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
            raw,
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

        let element_list = self.parse_element_list()?;

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

                    let assignment_expression = self.with_params(
                        self.params.clone().add_allow_in(false),
                        Self::parse_assignment_expression,
                    )?;

                    elements.push(Some(ArrayExpressionElement::SpreadElement(SpreadElement {
                        node: self.end_node(start_index)?,
                        argument: assignment_expression,
                    })));
                }
                _ => {
                    let assignment_expression = self.with_params(
                        self.params.clone().add_allow_in(false),
                        Self::parse_assignment_expression,
                    )?;

                    elements.push(Some(ArrayExpressionElement::Expression(
                        assignment_expression,
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

        let properties = self.parse_property_definition_list()?;

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
        let start_index = self.start_node();

        #[allow(unused_assignments)]
        let mut property_definition_key = None;
        #[allow(unused_assignments)]
        let mut property_definition_value = None;

        let property_definition_kind = PropertyKind::Init;

        let mut is_shorthand = false;
        let mut is_computed = false;

        match (self.token_kind(), self.peek_token_kind()) {
            // `... AssignmentExpression`.
            (TokenKind::Ellipsis, _) => {
                self.advance_any(); // Eat '...' token.

                let assignment_expression = self.with_params(
                    self.params.clone().add_allow_in(false),
                    Self::parse_assignment_expression,
                )?;

                return Ok(ObjectExpressionProperty::SpreadElement(SpreadElement {
                    node: self.end_node(start_index)?,
                    argument: assignment_expression,
                }));
            }
            // `MethodDefinition > get ClassElementName`.
            (TokenKind::Keyword(KeywordKind::Get), peek_token_kind)
                if peek_token_kind.is_class_element_name() =>
            {
                self.advance_any(); // Eat 'get' token.

                is_computed = self.token_kind() == TokenKind::LeftSquareBracket;

                let method_definition_key = self.parse_class_element_name()?;

                let function_expression = self.parse_method_definition_getter_body()?;

                let method_definition = self.parse_method_definition(
                    start_index,
                    Some(method_definition_key),
                    Some(function_expression),
                    MethodDefinitionKind::Get,
                    false,
                    is_computed,
                )?;

                return Ok(ObjectExpressionProperty::Property(Property::try_from(
                    method_definition,
                )?));
            }
            // `MethodDefinition > set ClassElementName`.
            (TokenKind::Keyword(KeywordKind::Set), peek_token_kind)
                if peek_token_kind.is_class_element_name() =>
            {
                self.advance_any(); // Eat 'set' token.

                is_computed = self.token_kind() == TokenKind::LeftSquareBracket;

                let method_definition_key = self.parse_class_element_name()?;

                let function_expression = self.parse_method_definition_setter_body()?;

                let method_definition = self.parse_method_definition(
                    start_index,
                    Some(method_definition_key),
                    Some(function_expression),
                    MethodDefinitionKind::Set,
                    false,
                    is_computed,
                )?;

                return Ok(ObjectExpressionProperty::Property(Property::try_from(
                    method_definition,
                )?));
            }
            // `MethodDefinition > GeneratorMethod > * ClassElementName`.
            (TokenKind::Multiplication, peek_token_kind)
                if peek_token_kind.is_class_element_name() =>
            {
                is_computed = self.peek_token_kind() == TokenKind::LeftSquareBracket;

                let generator_method_definition = self.parse_generator_method(
                    start_index,
                    is_computed,
                    false,
                    false,
                    MethodDefinitionKind::Method,
                )?;

                return Ok(ObjectExpressionProperty::Property(Property::try_from(
                    generator_method_definition,
                )?));
            }
            // `MethodDefinition > AsyncMethod > async [no LineTerminator here] ClassElementName`.
            (TokenKind::Keyword(KeywordKind::Async), peek_token_kind)
                if !self.has_current_token_line_terminator()
                    && peek_token_kind.is_class_element_name() =>
            {
                is_computed = self.peek_token_kind() == TokenKind::LeftSquareBracket;

                let async_method_definition = self.parse_async_method(
                    start_index,
                    is_computed,
                    false,
                    MethodDefinitionKind::Method,
                )?;

                return Ok(ObjectExpressionProperty::Property(Property::try_from(
                    async_method_definition,
                )?));
            }
            // `MethodDefinition > AsyncGeneratorMethod > async [no LineTerminator here] * ClassElementName`.
            (TokenKind::Keyword(KeywordKind::Async), TokenKind::Multiplication)
                if !self.has_current_token_line_terminator() =>
            {
                self.advance_any(); // Eat 'async' token.

                is_computed = self.peek_token_kind() == TokenKind::LeftSquareBracket;

                let async_generator_method_definition = self.parse_generator_method(
                    start_index,
                    is_computed,
                    true,
                    false,
                    MethodDefinitionKind::Method,
                )?;

                return Ok(ObjectExpressionProperty::Property(Property::try_from(
                    async_generator_method_definition,
                )?));
            }
            // `CoverInitializedName > IdentifierReference Initializer`.
            (token_kind, TokenKind::Assignment) if token_kind.is_identifier_reference() => {
                property_definition_key =
                    Some(Expression::Identifier(self.parse_identifier_reference()?));

                self.expect_and_advance(TokenKind::Assignment)?;

                let assignment_expression = self.with_params(
                    self.params.clone().add_allow_in(false),
                    Self::parse_assignment_expression,
                )?;

                property_definition_value = Some(PropertyValue::Expression(assignment_expression));
            }
            // `(PropertyName > LiteralPropertyName) :`.
            (token_kind, TokenKind::Colon) if token_kind.is_property_name() => {
                property_definition_key = Some(self.parse_property_name()?);

                self.expect_and_advance(TokenKind::Colon)?;

                let assignment_expression = self.with_params(
                    self.params.clone().add_allow_in(false),
                    Self::parse_assignment_expression,
                )?;

                property_definition_value = Some(PropertyValue::Expression(assignment_expression));
            }

            // `(PropertyName > ComputedPropertyName) :`.
            (TokenKind::LeftSquareBracket, _) => {
                is_computed = true;

                let property_name = self.parse_computed_property_name()?;

                match self.token_kind() {
                    // `ComputedPropertyName : AssignmentExpression`.
                    TokenKind::Colon => {
                        property_definition_key = Some(property_name);

                        self.expect_and_advance(TokenKind::Colon)?;

                        let assignment_expression = self.with_params(
                            self.params.clone().add_allow_in(false),
                            Self::parse_assignment_expression,
                        )?;

                        property_definition_value =
                            Some(PropertyValue::Expression(assignment_expression));
                    }
                    // `MethodDefinition > ClassElementName > ComputedPropertyName ( UniqueFormalParameters )`.
                    TokenKind::LeftParenthesis => {
                        let function_expression = self.parse_method_definition_method_body()?;

                        let method_definition = self.parse_method_definition(
                            start_index,
                            Some(PropertyDefinitionKey::Expression(property_name)),
                            Some(function_expression),
                            MethodDefinitionKind::Method,
                            false,
                            is_computed,
                        )?;

                        return Ok(ObjectExpressionProperty::Property(Property::try_from(
                            method_definition,
                        )?));
                    }
                    _ => return Err(self.unexpected_current_token_kind()),
                };
            }
            // `IdentifierReference`.
            (token_kind, peek_token_kind)
                if token_kind.is_identifier_reference()
                    && peek_token_kind != TokenKind::LeftParenthesis =>
            {
                is_shorthand = true;

                let identifier_reference =
                    Expression::Identifier(self.parse_identifier_reference()?);

                property_definition_key = Some(identifier_reference.clone());

                property_definition_value = Some(PropertyValue::Expression(identifier_reference));
            }
            // `MethodDefinition > ClassElementName ( UniqueFormalParameters )`.
            (token_kind, _) if token_kind.is_class_element_name() => {
                is_computed = token_kind == TokenKind::LeftSquareBracket;

                let method_definition_key = self.parse_class_element_name()?;

                let function_expression = self.parse_method_definition_method_body()?;

                let method_definition = self.parse_method_definition(
                    start_index,
                    Some(method_definition_key),
                    Some(function_expression),
                    MethodDefinitionKind::Method,
                    false,
                    is_computed,
                )?;

                return Ok(ObjectExpressionProperty::Property(Property::try_from(
                    method_definition,
                )?));
            }
            _ => return Err(self.unexpected_current_token_kind()),
        };

        let Some(property_definition_key) = property_definition_key else {
            return Err(ParserError::InvalidPropertyKey);
        };

        let Some(property_definition_value) = property_definition_value else {
            return Err(ParserError::InvalidPropertyValue);
        };

        Ok(ObjectExpressionProperty::Property(Property {
            method: false,
            shorthand: is_shorthand,
            computed: is_computed,
            key: property_definition_key,
            node: self.end_node(start_index)?,
            value: property_definition_value,
            kind: property_definition_kind,
        }))
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
                    value: LiteralValue::Number(value),
                    raw,
                }))
            }
            TokenKind::LeftSquareBracket => self.parse_computed_property_name(),
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ComputedPropertyName
    pub(crate) fn parse_computed_property_name(&mut self) -> Result<Expression, ParserError> {
        self.expect_and_advance(TokenKind::LeftSquareBracket)?;

        let assignment_expression = self.with_params(
            self.params.clone().add_allow_in(false),
            Self::parse_assignment_expression,
        )?;

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
                        let expression = self.with_params(
                            self.params.clone().add_allow_in(false),
                            Self::parse_expression,
                        )?;

                        expressions.push(expression);
                    }
                    TokenKind::TemplateMiddle => {
                        quasis.push(self.parse_template_element(false, 1, 2)?);

                        let expression = self.with_params(
                            self.params.clone().add_allow_in(false),
                            Self::parse_expression,
                        )?;

                        expressions.push(expression);
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
