use std::array;

use crate::{KeywordKind, Parser, ParserError, Token, TokenKind, TokenValue};
use hippo_estree::*;

fn march_token_kind_to_update_operator(token_kind: &TokenKind) -> Option<UpdateOperator> {
    match token_kind {
        TokenKind::Increment => Some(UpdateOperator::PlusPlus),
        TokenKind::Decrement => Some(UpdateOperator::MinusMinus),
        _ => None,
    }
}

// 13.5 Unary Operators
// https://tc39.es/ecma262/#prod-UnaryExpression
fn is_token_kind_unary_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Increment | TokenKind::Decrement => true,
        _ => false,
    }
}

fn match_token_kind_to_unary_operator(token_kind: &TokenKind) -> Option<UnaryOperator> {
    match token_kind {
        TokenKind::Keyword(KeywordKind::Delete) => Some(UnaryOperator::Delete),
        TokenKind::Keyword(KeywordKind::Void) => Some(UnaryOperator::Void),
        TokenKind::Keyword(KeywordKind::Typeof) => Some(UnaryOperator::Typeof),
        TokenKind::Addition => Some(UnaryOperator::Plus),
        TokenKind::Subtraction => Some(UnaryOperator::Minus),
        TokenKind::BitwiseNot => Some(UnaryOperator::Tilde),
        TokenKind::LogicalNot => Some(UnaryOperator::Bang),
        _ => None,
    }
}

// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#table
fn match_token_kind_to_operator_precedence(token_kind: &TokenKind) -> Option<(u8, u8)> {
    match token_kind {
        TokenKind::NullishCoalescing => Some((0, 1)),
        TokenKind::LogicalOr => Some((2, 3)),
        TokenKind::LogicalAnd => Some((4, 5)),
        TokenKind::BitwiseOr => Some((6, 7)),
        TokenKind::BitwiseXor => Some((8, 9)),
        TokenKind::BitwiseAnd => Some((10, 11)),
        TokenKind::Equal
        | TokenKind::NotEqual
        | TokenKind::StrictEqual
        | TokenKind::StrictNotEqual => Some((12, 13)),
        TokenKind::LessThan
        | TokenKind::GreaterThan
        | TokenKind::LessThanOrEqual
        | TokenKind::GreaterThanOrEqual
        | TokenKind::Keyword(KeywordKind::Instanceof)
        | TokenKind::Keyword(KeywordKind::In) => Some((14, 15)),
        TokenKind::LeftShift | TokenKind::RightShift | TokenKind::UnsignedRightShift => {
            Some((16, 17))
        }
        TokenKind::Addition | TokenKind::Subtraction => Some((18, 19)),
        TokenKind::Multiplication | TokenKind::Division | TokenKind::Modulus => Some((20, 21)),
        TokenKind::Exponentiation => Some((23, 22)),
        _ => None,
    }
}

fn match_token_kind_to_binary_operator(token_kind: &TokenKind) -> Option<BinaryOperator> {
    match token_kind {
        TokenKind::BitwiseOr => Some(BinaryOperator::Bar),
        TokenKind::BitwiseXor => Some(BinaryOperator::Caret),
        TokenKind::BitwiseAnd => Some(BinaryOperator::Ampersand),
        TokenKind::Equal => Some(BinaryOperator::EqualEqual),
        TokenKind::NotEqual => Some(BinaryOperator::NotEqual),
        TokenKind::StrictEqual => Some(BinaryOperator::EqualEqualEqual),
        TokenKind::StrictNotEqual => Some(BinaryOperator::NotEqualEqual),
        TokenKind::LessThan => Some(BinaryOperator::LessThan),
        TokenKind::GreaterThan => Some(BinaryOperator::GreaterThan),
        TokenKind::LessThanOrEqual => Some(BinaryOperator::LessThanEqual),
        TokenKind::GreaterThanOrEqual => Some(BinaryOperator::GreaterThan),
        TokenKind::Keyword(KeywordKind::Instanceof) => Some(BinaryOperator::Instanceof),
        TokenKind::Keyword(KeywordKind::In) => Some(BinaryOperator::In),
        TokenKind::LeftShift => Some(BinaryOperator::LessThanLessThan),
        TokenKind::RightShift => Some(BinaryOperator::GreaterThanGreaterThan),
        TokenKind::UnsignedRightShift => Some(BinaryOperator::GreaterThanGreaterThanGreaterThan),
        TokenKind::Addition => Some(BinaryOperator::Plus),
        TokenKind::Subtraction => Some(BinaryOperator::Minus),
        TokenKind::Multiplication => Some(BinaryOperator::Star),
        TokenKind::Division => Some(BinaryOperator::Slash),
        TokenKind::Modulus => Some(BinaryOperator::Percent),
        TokenKind::Exponentiation => Some(BinaryOperator::StarStar),
        _ => None,
    }
}

fn match_token_kind_to_logical_operator(token_kind: &TokenKind) -> Option<LogicalOperator> {
    match token_kind {
        TokenKind::NullishCoalescing => Some(LogicalOperator::NullishCoalescing),
        TokenKind::LogicalOr => Some(LogicalOperator::Or),
        TokenKind::LogicalAnd => Some(LogicalOperator::And),
        _ => None,
    }
}

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
        _ => None,
    }
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // https://tc39.es/ecma262/#prod-Expression
    pub(crate) fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let assignment_expression = self.parse_assignment_expression()?;

        if self.current_token_kind() == TokenKind::Comma {
            self.expect_and_advance(TokenKind::Comma)?;

            let right = self.parse_expression()?;

            return Ok(Expression::Sequence(SequenceExpression {
                node: self.create_node(&start_token, &self.current_token),
                expressions: vec![assignment_expression, right],
            }));
        } else {
            Ok(assignment_expression)
        }
    }

    // 13.1 Identifiers
    // https://tc39.es/ecma262/#prod-IdentifierReference
    fn parse_identifier_reference(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete. Nede to handle yield and await.

        let start_token = self.start_token();

        let token_value = self.current_token_value();

        self.expect_one_of_and_advance(vec![
            TokenKind::Identifier,
            TokenKind::Keyword(KeywordKind::Await),
            TokenKind::Keyword(KeywordKind::Yield),
        ])?;

        match token_value {
            TokenValue::String(name) => Ok(Expression::Identifier(Identifier {
                node: self.create_node(&start_token, &self.previous_token),
                name,
            })),
            _ => Err(ParserError::UnexpectedTokenValue),
        }
    }

    pub(crate) fn parse_label_identifier(&mut self) -> Result<Identifier, ParserError> {
        todo!("parse_label_identifier")
    }

    // 13.2 Primary Expression
    // https://tc39.es/ecma262/#prod-PrimaryExpression
    fn parse_primary_expresison(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete. Handle template literals and Regex.
        let token_kind = self.current_token_kind();

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
                if self.peek_token_kind() == TokenKind::Multiplication {
                    self.parse_generator_expression()
                } else {
                    self.parse_function_expression()
                }
            }
            TokenKind::Keyword(KeywordKind::Class) => self.parse_class_expression(),
            TokenKind::Keyword(KeywordKind::Async) => {
                self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

                if self.peek_token_kind() == TokenKind::Multiplication {
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
        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::This))?;

        Ok(Expression::This(ThisExpression {
            node: self.create_node(&start_token, &self.current_token),
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
        let start_token = self.start_token();

        let token_value = self.current_token_value();

        let node = self.create_node(&start_token, &self.current_token);

        let literal = match self.current_token_kind() {
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
        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::LeftSquareBracket)?;

        let element_list = self.parse_element_list()?;

        self.expect_and_advance(TokenKind::RightSquareBracket)?;

        Ok(Expression::Array(ArrayExpression {
            node: self.create_node(&start_token, &self.previous_token),
            elements: element_list,
        }))
    }

    // https://tc39.es/ecma262/#prod-ElementList
    fn parse_element_list(
        &mut self,
    ) -> Result<Vec<Option<Box<MemberExpressionElements>>>, ParserError> {
        let mut elements = vec![];

        while self.current_token_kind() != TokenKind::RightSquareBracket {
            match self.current_token_kind() {
                TokenKind::Comma => {
                    self.advance(); // Eat the , token.

                    elements.push(None);

                    continue;
                }
                TokenKind::Ellipsis => {
                    let start_token = self.start_token();

                    self.advance(); // Eat the ... token.

                    let assigment_expression: Expression = self.parse_assignment_expression()?;

                    elements.push(Some(Box::new(MemberExpressionElements::SpreadElement(
                        SpreadElement {
                            node: self.create_node(&start_token, &self.previous_token),
                            argument: assigment_expression,
                        },
                    ))));
                }
                _ => {
                    let assigment_expression: Expression = self.parse_assignment_expression()?;

                    elements.push(Some(Box::new(MemberExpressionElements::Expression(
                        assigment_expression,
                    ))));
                }
            };

            if self.current_token_kind() == TokenKind::RightSquareBracket {
                break;
            }

            self.expect_and_advance(TokenKind::Comma)?;
        }

        Ok(elements)
    }

    // https://tc39.es/ecma262/#prod-ObjectLiteral
    fn parse_object_literal(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let properties = self.parse_property_definition_list()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Expression::Object(ObjectExpression {
            node: self.create_node(&start_token, &self.previous_token),
            properties,
        }))
    }

    // https://tc39.es/ecma262/#prod-PropertyDefinitionList
    fn parse_property_definition_list(
        &mut self,
    ) -> Result<Vec<ObjectExpressionProperties>, ParserError> {
        let mut properties = vec![];

        while self.current_token_kind() != TokenKind::RightCurlyBrace {
            let property = self.parse_property_definition()?;

            properties.push(property);

            if self.current_token_kind() == TokenKind::RightCurlyBrace {
                break;
            }

            self.expect_and_advance(TokenKind::Comma)?;
        }

        Ok(properties)
    }

    // https://tc39.es/ecma262/#prod-PropertyDefinition
    fn parse_property_definition(&mut self) -> Result<ObjectExpressionProperties, ParserError> {
        let start_token = self.start_token();

        match self.current_token_kind() {
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

                match self.current_token_kind() {
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
                    node: self.create_node(&start_token, &self.previous_token),
                    value,
                    kind,
                }))
            }
            TokenKind::Ellipsis => {
                self.advance(); // Eat the ... token.

                let assigment_expression: Expression = self.parse_assignment_expression()?;

                Ok(ObjectExpressionProperties::SpreadElement(SpreadElement {
                    node: self.create_node(&start_token, &self.previous_token),
                    argument: assigment_expression,
                }))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-PropertyName
    fn parse_property_name(&mut self) -> Result<PropertyKey, ParserError> {
        let start_token = self.start_token();

        let token_value = self.current_token_value();

        match self.current_token_kind() {
            TokenKind::Identifier => {
                self.advance(); // Eat the identifier token.

                let name = match token_value {
                    TokenValue::String(value) => value,
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(PropertyKey::Identifier(Identifier {
                    node: self.create_node(&start_token, &self.previous_token),
                    name,
                }))
            }
            TokenKind::StringLiteral => {
                self.advance(); // Eat the string literal token.

                let raw_value = match token_value {
                    TokenValue::String(value) => value,
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(PropertyKey::Literal(Literal {
                    node: self.create_node(&start_token, &self.previous_token),
                    value: LiteralValue::String(raw_value.clone()),
                    raw: format!("\"{}\"", raw_value.to_string()),
                }))
            }
            TokenKind::NumberLiteral => {
                self.advance(); // Eat the number literal token.

                let (raw, value) = match token_value {
                    TokenValue::Number { raw, value } => (raw, value),
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(PropertyKey::Literal(Literal {
                    node: self.create_node(&start_token, &self.previous_token),
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

    // 13.3 Left-Hand-Side Expressions
    // https://tc39.es/ecma262/#prod-LeftHandSideExpression
    fn parse_left_hand_side_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        let _node = self.start_token();

        let current_token_kind = self.current_token_kind();

        if current_token_kind == TokenKind::Keyword(KeywordKind::New) {
            let new_expression = self.parse_new_expression()?;

            Ok(new_expression)
        } else {
            self.parse_member_expression()
        }
    }

    // https://tc39.es/ecma262/#prod-MemberExpression
    fn parse_member_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        let primary_expression = self.parse_primary_expresison();

        primary_expression
    }

    // https://tc39.es/ecma262/#prod-NewExpression
    fn parse_new_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::New))?;

        let callee = self.parse_member_expression()?;

        Ok(Expression::New(NewExpression {
            node: self.create_node(&start_token, &self.current_token),
            callee: Box::new(callee),
            arguments: vec![],
        }))
    }

    // https://tc39.es/ecma262/#prod-CallExpression
    fn parse_call_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        let start_token = self.start_token();

        let current_token_kind = self.current_token_kind();

        match current_token_kind {
            TokenKind::Keyword(KeywordKind::Super) => {
                self.advance(); // Eat the super token.

                let arguments = self.parse_arguments()?;

                return Ok(Expression::Call(CallExpression {
                    node: self.create_node(&start_token, &self.current_token),
                    callee: Box::new(CallExpressionCallee::Super(Super {
                        node: self.create_node(&start_token, &self.current_token),
                    })),
                    arguments,
                }));
            }
            TokenKind::Keyword(KeywordKind::Import) => {
                self.advance(); // Eat the import token.

                todo!()
            }
            TokenKind::LeftParenthesis => {
                let arguments = self.parse_arguments()?;

                let call_expression = self.parse_call_expression()?;

                return Ok(Expression::Call(CallExpression {
                    node: self.create_node(&start_token, &self.current_token),
                    callee: Box::new(CallExpressionCallee::Expression(call_expression)),
                    arguments,
                }));
            }
            _ => todo!(),
        }
    }

    // https://tc39.es/ecma262/#prod-Arguments
    fn parse_arguments(&mut self) -> Result<Vec<CallExpressionArguments>, ParserError> {
        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let mut arguments_list = vec![];

        while self.current_token_kind() != TokenKind::RightParenthesis {
            let start_token = self.start_token();

            let is_spread = if self.current_token_kind() == TokenKind::Ellipsis {
                self.advance(); // Eat the ... token.

                true
            } else {
                false
            };

            let argument = self.parse_assignment_expression()?;

            if is_spread {
                arguments_list.push(CallExpressionArguments::SpreadElement(SpreadElement {
                    node: self.create_node(&start_token, &self.current_token),
                    argument,
                }));
            } else {
                arguments_list.push(CallExpressionArguments::Expression(argument));
            }

            if self.current_token_kind() != TokenKind::Comma {
                break;
            }

            if self.current_token_kind() == TokenKind::RightParenthesis {
                break;
            }

            self.advance();
        }

        Ok(arguments_list)
    }

    // 13.4 Update Expressions
    // https://tc39.es/ecma262/#prod-UpdateExpression
    fn parse_update_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let mut current_token_kind = self.current_token_kind();

        if is_token_kind_unary_operator(&current_token_kind) {
            self.advance(); // Eat the ++ or -- token.

            let unary_expression = self.parse_unary_expression()?;

            let operator = march_token_kind_to_update_operator(&current_token_kind).unwrap();

            Ok(Expression::Update(UpdateExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator,
                argument: Box::new(unary_expression),
                prefix: true,
            }))
        } else {
            let left_hand_side_expression = self.parse_left_hand_side_expression()?;

            current_token_kind = self.current_token_kind();

            if !is_token_kind_unary_operator(&current_token_kind) {
                return Ok(left_hand_side_expression);
            }

            let update_operator = march_token_kind_to_update_operator(&current_token_kind).unwrap();

            Ok(Expression::Update(UpdateExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator: update_operator,
                argument: Box::new(left_hand_side_expression),
                prefix: false,
            }))
        }
    }

    // 13.5 Unary Operators
    // https://tc39.es/ecma262/#prod-UnaryExpression
    fn parse_unary_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let current_token_kind = self.current_token_kind();

        match current_token_kind {
            TokenKind::Keyword(KeywordKind::Delete)
            | TokenKind::Keyword(KeywordKind::Void)
            | TokenKind::Keyword(KeywordKind::Typeof)
            | TokenKind::Addition
            | TokenKind::Subtraction
            | TokenKind::BitwiseNot
            | TokenKind::LogicalNot => {
                self.advance(); // Eat the delete or void or typeof or + or - or ~ or ! token.

                let unary_argument = self.parse_unary_expression()?;

                let operator =
                    match_token_kind_to_unary_operator(&self.current_token_kind()).unwrap();

                self.advance();

                Ok(Expression::Unary(UnaryExpression {
                    node: self.create_node(&start_token, &self.current_token),
                    operator,
                    prefix: true,
                    argument: Box::new(unary_argument),
                }))
            }
            TokenKind::Keyword(KeywordKind::Await) => {
                let unary_expression = self.parse_unary_expression()?;

                // TODO check if is supported by ECMA script version.
                Ok(Expression::Await(AwaitExpression {
                    node: self.create_node(&start_token, &self.current_token),
                    argument: Box::new(unary_expression),
                }))
            }
            _ => self.parse_update_expression(),
        }
    }

    // 13.6 Exponentiation Operators
    // https://tc39.es/ecma262/#prod-ExponentiationExpression

    // 13.7 Multiplicative Operators
    // https://tc39.es/ecma262/#prod-MultiplicativeExpression

    // 13.8 Additive Operators
    // https://tc39.es/ecma262/#prod-AdditiveExpression

    // 13.9 Bitwise Shift Operators
    // https://tc39.es/ecma262/#prod-ShiftExpression

    // 13.10 Relational Operators
    // https://tc39.es/ecma262/#prod-RelationalExpression

    // 13.11 Equality Operators
    // https://tc39.es/ecma262/#prod-EqualityExpression

    // 13.12 Binary Bitwise Operators
    // https://tc39.es/ecma262/#prod-BitwiseANDExpression
    // https://tc39.es/ecma262/#prod-BitwiseXORExpression
    // https://tc39.es/ecma262/#prod-BitwiseORExpression

    // 13.13 Binary Logical Operators
    // https://tc39.es/ecma262/#prod-LogicalANDExpression
    // https://tc39.es/ecma262/#prod-LogicalORExpression

    // A Pratt parser is a top-down parser that parses expressions based on operator precedence.
    fn parse_binary_expression(&mut self, precedence: u8) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let unary_expression = self.parse_unary_expression()?;

        self.parse_binary_expression_recursive_impl(unary_expression, &start_token, precedence)
    }

    fn parse_binary_expression_recursive_impl(
        &mut self,
        mut left_expression: Expression,
        left_start_token: &Token,
        minimum_precedence: u8,
    ) -> Result<Expression, ParserError> {
        while self.current_token_kind() != TokenKind::EOF {
            let current_token_kind = self.current_token_kind();

            let Some((left_precedence, right_precedence)) =
                match_token_kind_to_operator_precedence(&current_token_kind)
            else {
                break;
            };

            if left_precedence < minimum_precedence {
                break;
            }

            self.advance(); // Eat the operator token.

            let right_expression = self.parse_binary_expression(right_precedence)?;

            let node = self.create_node(&left_start_token, &self.previous_token);

            if current_token_kind.is_logical_operator() {
                let Some(operator) = match_token_kind_to_logical_operator(&current_token_kind)
                else {
                    break;
                };

                left_expression = Expression::Logical(LogicalExpression {
                    node,
                    operator,
                    left: Box::new(left_expression),
                    right: Box::new(right_expression),
                });
            } else if current_token_kind.is_binary_operator() {
                let Some(operator) = match_token_kind_to_binary_operator(&current_token_kind)
                else {
                    break;
                };

                left_expression = Expression::Binary(BinaryExpression {
                    node,
                    operator,
                    left: Box::new(left_expression),
                    right: Box::new(right_expression),
                });
            } else {
                break;
            }
        }

        Ok(left_expression)
    }

    // 13.14 Conditional Operator ( ? : )
    // https://tc39.es/ecma262/#prod-ConditionalExpression
    fn parse_conditional_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let short_circuit_expression = self.parse_binary_expression(0)?;

        if self.current_token_kind() == TokenKind::QuestionMark {
            self.advance(); // Eat the ? token.

            let consequent = self.parse_assignment_expression()?;

            self.expect_and_advance(TokenKind::Colon)?;

            let alternate = self.parse_assignment_expression()?;

            return Ok(Expression::Conditional(ConditionalExpression {
                node: self.create_node(&start_token, &self.current_token),
                test: Box::new(short_circuit_expression),
                consequent: Box::new(consequent),
                alternate: Box::new(alternate),
            }));
        }

        Ok(short_circuit_expression)
    }

    // 13.15 Assignment Operators
    // https://tc39.es/ecma262/#prod-AssignmentExpression
    fn parse_assignment_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.
        if self.current_token_kind() == TokenKind::Keyword(KeywordKind::Yield) {
            return self.parse_yield_expression();
        }

        if self.current_token_kind() == TokenKind::Identifier {
            // TODO Handle CoverParenthesizedExpressionAndArrowParameterList.
            if self.peek_token_kind() == TokenKind::ArrowFunction {
                return self.parse_arrow_function();
            }
        }

        // TODO Handle async arrow functions.

        let start_token = self.start_token();

        let left_expression = self.parse_left_hand_side_expression()?;

        match self.current_token_kind() {
            TokenKind::Assignment => {
                self.advance(); // Eat the assignment operator token.

                let right = self.parse_assignment_expression()?;

                // https://tc39.es/ecma262/#sec-assignment-operators-static-semantics-early-errors
                let left_array_or_object_pattern = match &left_expression {
                    Expression::Array(array_expression) => array_expression.to_pattern(),
                    Expression::Object(object_expression) => object_expression.to_pattern(),
                    _ => None,
                }
                .unwrap();

                return Ok(Expression::Assignment(AssignmentExpression {
                    node: self.create_node(&start_token, &self.previous_token),
                    operator: AssignmentOperator::Assignment,
                    left: Box::new(left_array_or_object_pattern),
                    right: Box::new(right),
                }));
            }
            current_token_kind if current_token_kind.is_assignment_operator() => {
                let operator =
                    match_token_kind_to_assignment_operator(&self.current_token_kind()).unwrap();

                self.advance(); // Eat the assignment operator token.

                let right = self.parse_assignment_expression()?;

                return Ok(Expression::Assignment(AssignmentExpression {
                    node: self.create_node(&start_token, &self.previous_token),
                    operator,
                    left: Box::new(AssignmentExpressionLeft::Expression(left_expression)),
                    right: Box::new(right),
                }));
            }
            _ => {}
        }

        Ok(left_expression)
    }
}
