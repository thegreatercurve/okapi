use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

fn match_token_kind_to_assignment_operator(token_kind: &TokenKind) -> Option<AssignmentOperator> {
    match token_kind {
        TokenKind::MultiplyAssignment => Some(AssignmentOperator::MultiplyAssignment),
        TokenKind::DivisionAssignment => Some(AssignmentOperator::DivisionAssignment),
        TokenKind::ModulusAssignment => Some(AssignmentOperator::ModulusAssignment),
        TokenKind::AdditionAssignment => Some(AssignmentOperator::AdditionAssignment),
        TokenKind::MinusAssignment => Some(AssignmentOperator::MinusAssignment),
        TokenKind::LeftShiftAssignment => Some(AssignmentOperator::LeftShiftAssignment),
        TokenKind::RightShiftAssignment => Some(AssignmentOperator::RightShiftAssignment),
        TokenKind::UnsignedRightShiftAssignment => {
            Some(AssignmentOperator::UnsignedRightShiftAssignment)
        }
        TokenKind::BitwiseAndAssignment => Some(AssignmentOperator::BitwiseAndAssignment),
        TokenKind::BitwiseOrAssignment => Some(AssignmentOperator::BitwiseOrAssignment),
        TokenKind::BitwiseXorAssignment => Some(AssignmentOperator::BitwiseXorAssignment),
        TokenKind::ExponentiationAssignment => Some(AssignmentOperator::ExponentiationAssignment),
        TokenKind::LogicalOrAssignment => Some(AssignmentOperator::LogicalOrAssignment),
        TokenKind::LogicalAndAssignment => Some(AssignmentOperator::LogicalAndAssignment),
        TokenKind::NullishCoalescingAssignment => {
            Some(AssignmentOperator::NullishCoalescingAssignment)
        }
        TokenKind::Assignment => Some(AssignmentOperator::Assignment),
        _ => None,
    }
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 13.15 Assignment Operators
    // https://tc39.es/ecma262/#prod-AssignmentExpression
    pub(crate) fn parse_assignment_expression(&mut self) -> Result<Expression, ParserError> {
        if self.params.has_allow_yield()
            && self.token_kind() == TokenKind::Keyword(KeywordKind::Yield)
        {
            return Ok(Expression::Yield(self.parse_yield_expression()?));
        }

        let is_maybe_assignment_pattern = self.token_kind().is_assignment_pattern_start();
        let is_binding_identifier = self.token_kind().is_binding_identifier();

        let is_maybe_parenthesized_arrow_function = self.token_kind() == TokenKind::LeftParenthesis;
        let is_maybe_async_parenthesized_arrow_function = self.token_kind()
            == TokenKind::Keyword(KeywordKind::Async)
            && self.peek_token_kind() == TokenKind::LeftParenthesis;

        let start_index = self.start_node();

        let previous_cursor = self.cursor.clone();
        let previous_context = self.context.clone();

        // Short circuit simple arrow functions.
        // `( ) [no LineTerminator here] =>`
        if is_maybe_parenthesized_arrow_function
            && self.peek_token_kind() == TokenKind::RightParenthesis
            && !self.has_peek_token_line_terminator()
            && self.peek_nth_kind(2) == TokenKind::ArrowFunction
        {
            return self.parse_arrow_function();
        }

        //  Short circuit simple binding identifier arrow functions.
        // `BindingIdentifier [no LineTerminator here] =>`
        if is_binding_identifier
            && !self.has_current_token_line_terminator()
            && self.peek_token_kind() == TokenKind::ArrowFunction
        {
            return self.parse_arrow_function();
        }

        // Short circuit simple binding identifier async arrow functions.
        // `async [no LineTerminator here] BindingIdentifier [no LineTerminator here] =>`
        if self.token_kind() == TokenKind::Keyword(KeywordKind::Async)
            && !self.has_current_token_line_terminator()
            && self.peek_token_kind().is_binding_identifier()
            && !self.has_peek_token_line_terminator()
            && self.peek_nth_kind(2) == TokenKind::ArrowFunction
        {
            return self.parse_async_arrow_function_declaration();
        }

        let is_arrow_function = if is_maybe_parenthesized_arrow_function
            || is_maybe_async_parenthesized_arrow_function
        {
            if is_maybe_async_parenthesized_arrow_function {
                self.expect_and_advance(TokenKind::Keyword(KeywordKind::Async))?;
                // Eat 'async' token.
            }

            let is_arrow_function = self.maybe_arrow_function();

            self.cursor = previous_cursor.clone();
            self.context = previous_context.clone();

            is_arrow_function
        } else {
            false
        };

        if is_arrow_function {
            if is_maybe_async_parenthesized_arrow_function {
                return self.parse_async_arrow_function_declaration();
            } else {
                return self.parse_arrow_function();
            }
        };

        let left_expression = self.parse_conditional_expression();

        match self.token_kind() {
            TokenKind::Assignment if is_maybe_assignment_pattern => {
                // If LeftHandSideExpression is either an ObjectLiteral or an ArrayLiteral, it must be reparsed as an AssignmentPattern.
                // https://tc39.es/ecma262/#sec-assignment-operators-static-semantics-early-errors
                self.cursor = previous_cursor;
                self.context = previous_context;

                let left_pattern = self.parse_assignment_pattern()?;

                self.expect_and_advance(TokenKind::Assignment)?;

                let right = self.parse_assignment_expression()?;

                return Ok(Expression::Assignment(AssignmentExpression {
                    node: self.end_node(start_index)?,
                    operator: AssignmentOperator::Assignment,
                    left: Box::new(AssignmentExpressionLeft::Pattern(left_pattern)),
                    right: Box::new(right),
                }));
            }
            token_kind if token_kind.is_assignment_operator() => {
                let operator = match_token_kind_to_assignment_operator(&self.token_kind()).unwrap();

                self.advance_any(); // Eat assignment operator token.

                let right = self.parse_assignment_expression()?;

                return Ok(Expression::Assignment(AssignmentExpression {
                    node: self.end_node(start_index)?,
                    operator,
                    left: Box::new(AssignmentExpressionLeft::Expression(left_expression?)),
                    right: Box::new(right),
                }));
            }
            _ => {}
        }

        self.end_node(start_index)?;

        left_expression
    }

    fn maybe_arrow_function(&mut self) -> bool {
        let mut parentheses_depth = 0;

        while !matches!(self.token_kind(), TokenKind::Illegal | TokenKind::EOF) {
            match self.token_kind() {
                TokenKind::LeftParenthesis => parentheses_depth += 1,
                TokenKind::RightParenthesis => parentheses_depth -= 1,
                _ => {}
            }

            match self.peek_token_kind() {
                TokenKind::ArrowFunction if parentheses_depth == 0 => return true,
                _ if parentheses_depth < 1 => return false,
                _ => {}
            }

            self.advance_any();
        }

        false
    }

    // 13.15.5 Destructuring Assignment
    // Supplemental Syntax
    // https://tc39.es/ecma262/#prod-AssignmentPattern
    pub(crate) fn parse_assignment_pattern(&mut self) -> Result<Pattern, ParserError> {
        let assignment_pattern = match self.token_kind() {
            TokenKind::LeftCurlyBrace => Pattern::Object(self.parse_object_assignment_pattern()?),
            TokenKind::LeftSquareBracket => Pattern::Array(self.parse_array_assignment_pattern()?),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(assignment_pattern)
    }

    // https://tc39.es/ecma262/#prod-ObjectAssignmentPattern
    fn parse_object_assignment_pattern(&mut self) -> Result<ObjectPattern, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?; // Eat '{' token.

        let mut properties = self.parse_assignment_property_list()?;

        if self.token_kind() == TokenKind::Ellipsis {
            properties.push(self.parse_assignment_rest_property()?);
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?; // Eat '}' token.

        Ok(ObjectPattern {
            node: self.end_node(start_index)?,
            properties,
        })
    }

    // https://tc39.es/ecma262/#prod-AssignmentRestProperty
    fn parse_assignment_rest_property(&mut self) -> Result<ObjectPatternProperty, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?; // Eat '...' token.

        let expression = self.parse_left_hand_side_expression()?;

        let pattern = Pattern::try_from(expression)?;

        Ok(ObjectPatternProperty::Rest(RestElement {
            node: self.end_node(start_index)?,
            argument: Box::new(pattern),
        }))
    }

    // https://tc39.es/ecma262/#prod-ArrayAssignmentPattern
    fn parse_array_assignment_pattern(&mut self) -> Result<ArrayPattern, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftSquareBracket)?; // Eat '[' token.

        let mut elements = self.parse_assignment_element_list()?;

        if self.token_kind() == TokenKind::Ellipsis {
            elements.push(Some(self.parse_assignment_rest_element()?));
        }

        self.expect_and_advance(TokenKind::RightSquareBracket)?; // Eat ']' token.

        Ok(ArrayPattern {
            node: self.end_node(start_index)?,
            elements,
        })
    }

    // https://tc39.es/ecma262/#prod-AssignmentPropertyList
    fn parse_assignment_property_list(
        &mut self,
    ) -> Result<Vec<ObjectPatternProperty>, ParserError> {
        let mut properties = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            if self.token_kind() == TokenKind::Ellipsis {
                break;
            }

            properties.push(self.parse_assignment_property()?);

            if self.token_kind() == TokenKind::Comma {
                self.advance_any(); // Eat ',' token.
            }
        }

        Ok(properties)
    }

    // https://tc39.es/ecma262/#prod-AssignmentElementList
    fn parse_assignment_element_list(
        &mut self,
    ) -> Result<Vec<Option<ArrayPatternElement>>, ParserError> {
        let mut elements = vec![];

        while self.token_kind() != TokenKind::RightSquareBracket {
            if self.token_kind() == TokenKind::Ellipsis {
                break;
            }

            if self.token_kind() == TokenKind::Comma {
                self.advance_any(); // Eat ellision token.

                elements.push(None);

                continue;
            }

            elements.push(Some(self.parse_assignment_element()?));

            if self.token_kind() == TokenKind::Comma {
                self.advance_any(); // Eat ',' token.
            }
        }

        Ok(elements)
    }

    // https://tc39.es/ecma262/#prod-AssignmentProperty
    fn parse_assignment_property(&mut self) -> Result<ObjectPatternProperty, ParserError> {
        let mut is_computed = false;

        let start_index = self.start_node(); // Start property node.

        let left_hand_expression = match self.token_kind() {
            TokenKind::LeftSquareBracket => {
                is_computed = true;

                self.parse_computed_property_name()?
            }
            token_kind if token_kind.is_property_name() => self.parse_property_name()?,
            _ => return Err(self.unexpected_current_token_kind()),
        };

        if self.token_kind() != TokenKind::Assignment {
            self.end_node(start_index)?; // End potential assignment pattern node.
        }

        match self.token_kind() {
            TokenKind::Colon => {
                self.expect_and_advance(TokenKind::Colon)?; // Eat ':' token.

                let assignment_element = self.parse_assignment_element()?;

                Ok(ObjectPatternProperty::Property(Property {
                    node: self.end_node(start_index)?,
                    method: false,
                    shorthand: false,
                    computed: is_computed,
                    key: left_hand_expression,
                    kind: PropertyKind::Init,
                    value: PropertyValue::Pattern(Pattern::try_from(assignment_element)?),
                }))
            }
            TokenKind::Assignment => {
                self.advance_any(); // Eat '=' token.

                let assignment_expression = self.with_params(
                    self.params.clone().add_allow_in(false),
                    Self::parse_assignment_expression,
                )?;

                let assignment_pattern = AssignmentPattern {
                    node: self.end_node(start_index)?,
                    left: Box::new(Pattern::try_from(left_hand_expression.clone())?),
                    right: assignment_expression,
                };

                Ok(ObjectPatternProperty::Property(Property {
                    node: self.end_node(start_index)?,
                    method: false,
                    shorthand: true,
                    computed: is_computed,
                    key: left_hand_expression,
                    kind: PropertyKind::Init,
                    value: PropertyValue::Pattern(Pattern::Assignment(assignment_pattern)),
                }))
            }
            _ => Ok(ObjectPatternProperty::Property(Property {
                node: self.end_node(start_index)?,
                method: false,
                shorthand: true,
                computed: is_computed,
                key: left_hand_expression.clone(),
                kind: PropertyKind::Init,
                value: PropertyValue::Pattern(Pattern::try_from(left_hand_expression)?),
            })),
        }
    }

    // https://tc39.es/ecma262/#prod-AssignmentElement
    fn parse_assignment_element(&mut self) -> Result<ArrayPatternElement, ParserError> {
        let start_index = self.start_node();

        let left_hand_side_expression_or_assignment_pattern =
            if self.token_kind().is_assignment_pattern_start() {
                self.parse_assignment_pattern()?
            } else {
                Pattern::try_from(self.parse_left_hand_side_expression()?)?
            };

        let assignment_element = if self.token_kind() == TokenKind::Assignment {
            self.advance_any(); // Eat '=' token.

            let assignment_expression = self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_assignment_expression,
            )?;

            ArrayPatternElement::Assignment(AssignmentPattern {
                node: self.end_node(start_index)?,
                left: Box::new(left_hand_side_expression_or_assignment_pattern),
                right: assignment_expression,
            })
        } else {
            self.end_node(start_index)?;

            ArrayPatternElement::try_from(left_hand_side_expression_or_assignment_pattern)?
        };

        Ok(assignment_element)
    }

    // https://tc39.es/ecma262/#prod-AssignmentRestElement
    fn parse_assignment_rest_element(&mut self) -> Result<ArrayPatternElement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?;

        let left_hand_side_expression = self.parse_left_hand_side_expression()?;

        Ok(ArrayPatternElement::RestElement(RestElement {
            node: self.end_node(start_index)?,
            argument: Box::new(Pattern::try_from(left_hand_side_expression)?),
        }))
    }
}
