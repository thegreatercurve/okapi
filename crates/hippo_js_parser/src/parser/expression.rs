use std::vec;

use crate::{KeywordKind, Parser, ParserError, TokenKind, TokenValue};
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

// 13.7 Multiplicative Operators
// https://tc39.es/ecma262/#prod-MultiplicativeExpression
fn is_token_kind_multiplicative_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Multiplication | TokenKind::Division | TokenKind::Modulus => true,
        _ => false,
    }
}

fn match_token_kind_to_multiplicative_operator(token_kind: &TokenKind) -> Option<BinaryOperator> {
    match token_kind {
        TokenKind::Multiplication => Some(BinaryOperator::Star),
        TokenKind::Division => Some(BinaryOperator::Slash),
        TokenKind::Modulus => Some(BinaryOperator::Percent),
        _ => None,
    }
}

// 13.8 Additive Operators
// https://tc39.es/ecma262/#prod-AdditiveExpression
fn is_token_kind_additive_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Addition | TokenKind::Subtraction => true,
        _ => false,
    }
}

fn match_token_kind_to_additive_operator(token_kind: &TokenKind) -> Option<BinaryOperator> {
    match token_kind {
        TokenKind::Addition => Some(BinaryOperator::Plus),
        TokenKind::Subtraction => Some(BinaryOperator::Minus),
        _ => None,
    }
}
// 13.9 Bitwise Shift Operators
// https://tc39.es/ecma262/#prod-ShiftExpression
fn is_token_kind_bitwise_shift_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::LeftShift | TokenKind::RightShift | TokenKind::UnsignedRightShift => true,
        _ => false,
    }
}

fn match_token_kind_to_bitwise_shift_operator(token_kind: &TokenKind) -> Option<BinaryOperator> {
    match token_kind {
        TokenKind::LeftShift => Some(BinaryOperator::GreaterThan),
        TokenKind::RightShift => Some(BinaryOperator::GreaterThanGreaterThan),
        TokenKind::UnsignedRightShift => Some(BinaryOperator::GreaterThanGreaterThanGreaterThan),
        _ => None,
    }
}

// 13.10 Relational Operators
// https://tc39.es/ecma262/#prod-RelationalExpression
fn is_token_kind_relational_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::LessThan
        | TokenKind::GreaterThan
        | TokenKind::LessThanOrEqual
        | TokenKind::GreaterThanOrEqual
        | TokenKind::Keyword(KeywordKind::Instanceof)
        | TokenKind::Keyword(KeywordKind::In) => true,
        _ => false,
    }
}

fn match_token_kind_to_relational_operator(token_kind: &TokenKind) -> Option<BinaryOperator> {
    match token_kind {
        TokenKind::LessThan => Some(BinaryOperator::LessThan),
        TokenKind::GreaterThan => Some(BinaryOperator::GreaterThan),
        TokenKind::LessThanOrEqual => Some(BinaryOperator::LessThanEqual),
        TokenKind::GreaterThanOrEqual => Some(BinaryOperator::GreaterThanEqual),
        TokenKind::Keyword(KeywordKind::Instanceof) => Some(BinaryOperator::Instanceof),
        TokenKind::Keyword(KeywordKind::In) => Some(BinaryOperator::In),
        _ => None,
    }
}

// 13.11 Equality Operators
// https://tc39.es/ecma262/#prod-EqualityExpression
fn is_token_kind_equality_operator(token_kind: &TokenKind) -> bool {
    match token_kind {
        TokenKind::Equal
        | TokenKind::NotEqual
        | TokenKind::StrictEqual
        | TokenKind::StrictNotEqual => true,
        _ => false,
    }
}

fn match_token_kind_to_equality_operator(token_kind: &TokenKind) -> Option<BinaryOperator> {
    match token_kind {
        TokenKind::Equal => Some(BinaryOperator::EqualEqual),
        TokenKind::NotEqual => Some(BinaryOperator::NotEqual),
        TokenKind::StrictEqual => Some(BinaryOperator::EqualEqualEqual),
        TokenKind::StrictNotEqual => Some(BinaryOperator::NotEqualEqual),
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

    // https://tc39.es/ecma262/#prod-ConditionalExpression
    fn parse_conditional_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.
        let _node = self.start_token();

        let short_circuit = self.parse_short_circuit_expression()?;

        Ok(short_circuit)
    }

    // https://tc39.es/ecma262/#prod-ShortCircuitExpression
    fn parse_short_circuit_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        let _node = self.start_token();

        let logical = self.parse_logical_or_expression()?;

        Ok(logical)
    }

    pub(crate) fn parse_label_identifier(&mut self) -> Result<Identifier, ParserError> {
        todo!()
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
                let token_value = match token_value {
                    TokenValue::String(value) => value,
                    _ => return Err(ParserError::UnexpectedTokenValue),
                };

                Ok(Expression::Literal(Literal {
                    node,
                    value: LiteralValue::String(token_value.clone()),
                    raw: token_value,
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
        // TODO This is currently incomplete.
        let start_token = self.start_token();

        match self.current_token_kind() {
            TokenKind::Identifier => {
                let identifier_reference = self.parse_identifier_reference()?;

                let peek_token_kind = self.peek_token_kind();

                Ok(ObjectExpressionProperties::Property(Property {
                    method: peek_token_kind == TokenKind::LeftParenthesis,
                    shorthand: peek_token_kind != TokenKind::Colon,
                    computed: false,
                    kind: PropertyKind::Init,
                    key: PropertyKey::Identifier(identifier_reference.clone()),
                    node: self.create_node(&start_token, &self.previous_token),
                    value: Box::new(identifier_reference),
                }))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
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

        let _node = self.start_token();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::New))?;

        let _callee = self.parse_left_hand_side_expression()?;

        // Ok(Expression::New {
        //     node: self.finish_node(&node),
        //     callee: Box::new(callee),
        //     arguments,
        // })

        todo!()
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

                let operator = match_token_kind_to_unary_operator(&current_token_kind).unwrap();

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
    fn parse_exponentiation_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let unary_expression = self.parse_unary_expression()?;

        if self.current_token_kind() == TokenKind::Exponentiation {
            self.advance(); // Eat the ** token.

            let exponentiation_expression = self.parse_exponentiation_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator: BinaryOperator::StarStar,
                left: Box::new(unary_expression),
                right: Box::new(exponentiation_expression),
            }))
        } else {
            Ok(unary_expression)
        }
    }

    // 13.7 Multiplicative Operators
    // https://tc39.es/ecma262/#prod-MultiplicativeExpression
    fn parse_multiplicative_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let exponentiation_expression = self.parse_exponentiation_expression()?;

        let current_token_kind = self.current_token_kind();

        if is_token_kind_multiplicative_operator(&current_token_kind) {
            self.advance(); // Eat the * or / or % token.

            let operator =
                match_token_kind_to_multiplicative_operator(&current_token_kind).unwrap();

            let multiplicative_expression = self.parse_multiplicative_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator,
                left: Box::new(exponentiation_expression),
                right: Box::new(multiplicative_expression),
            }))
        } else {
            Ok(exponentiation_expression)
        }
    }

    // 13.8 Additive Operators
    // https://tc39.es/ecma262/#prod-AdditiveExpression
    fn parse_additive_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let multiplicative_expression = self.parse_multiplicative_expression()?;

        let current_token_kind = self.current_token_kind();

        if is_token_kind_additive_operator(&current_token_kind) {
            self.advance(); // Eat the + or - token.

            let operator = match_token_kind_to_additive_operator(&current_token_kind).unwrap();

            let additive_expression = self.parse_additive_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator,
                left: Box::new(additive_expression),
                right: Box::new(multiplicative_expression),
            }))
        } else {
            Ok(multiplicative_expression)
        }
    }

    // 13.9 Bitwise Shift Operators
    // https://tc39.es/ecma262/#prod-ShiftExpression
    fn parse_bitwise_shift_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let additive_expression = self.parse_additive_expression()?;

        let current_token_kind = self.current_token_kind();

        if is_token_kind_bitwise_shift_operator(&current_token_kind) {
            self.advance(); // Eat the > or >> or >>> token.

            let operator = match_token_kind_to_bitwise_shift_operator(&current_token_kind).unwrap();

            let bitwise_shift_expression = self.parse_bitwise_shift_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator,
                left: Box::new(bitwise_shift_expression),
                right: Box::new(additive_expression),
            }))
        } else {
            Ok(additive_expression)
        }
    }

    // 13.10 Relational Operators
    // https://tc39.es/ecma262/#prod-RelationalExpression
    fn parse_relational_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let bitwise_shift_expression = self.parse_bitwise_shift_expression()?;

        let current_token_kind = self.current_token_kind();

        if is_token_kind_relational_operator(&current_token_kind) {
            self.advance(); // Eat the < or > or <= or >= or instanceof or in token.

            let operator = match_token_kind_to_relational_operator(&current_token_kind).unwrap();

            let relational_expression = self.parse_relational_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator,
                left: Box::new(relational_expression),
                right: Box::new(bitwise_shift_expression),
            }))
        } else {
            Ok(bitwise_shift_expression)
        }
    }

    // 13.11 Equality Operators
    // https://tc39.es/ecma262/#prod-EqualityExpression
    fn parse_equality_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let relational_expression = self.parse_relational_expression()?;

        let current_token_kind = self.current_token_kind();

        if is_token_kind_equality_operator(&current_token_kind) {
            self.advance(); // Eat the == or != or === or !== token.

            let operator = match_token_kind_to_equality_operator(&current_token_kind).unwrap();

            let equality_expression = self.parse_equality_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator,
                left: Box::new(equality_expression),
                right: Box::new(relational_expression),
            }))
        } else {
            Ok(relational_expression)
        }
    }

    // 13.12 Binary Bitwise Operators
    // https://tc39.es/ecma262/#prod-BitwiseANDExpression
    fn parse_bitwise_and_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let equality_expression = self.parse_equality_expression()?;

        if self.current_token_kind() == TokenKind::BitwiseAnd {
            self.advance(); // Eat the & token.

            let bitwise_and_expression = self.parse_bitwise_and_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator: BinaryOperator::Ampersand,
                left: Box::new(bitwise_and_expression),
                right: Box::new(equality_expression),
            }))
        } else {
            Ok(equality_expression)
        }
    }

    // https://tc39.es/ecma262/#prod-BitwiseXORExpression
    fn parse_bitwise_xor_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let bitwise_and_expression = self.parse_bitwise_and_expression()?;

        if self.current_token_kind() == TokenKind::BitwiseXor {
            self.advance(); // Eat the ^ token.

            let bitwise_xor_expression = self.parse_bitwise_xor_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator: BinaryOperator::Caret,
                left: Box::new(bitwise_xor_expression),
                right: Box::new(bitwise_and_expression),
            }))
        } else {
            Ok(bitwise_and_expression)
        }
    }

    // https://tc39.es/ecma262/#prod-BitwiseORExpression
    fn parse_bitwise_or_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let bitwise_xor_expression = self.parse_bitwise_xor_expression()?;

        if self.current_token_kind() == TokenKind::BitwiseOr {
            self.advance(); // Eat the | token.

            let bitwise_or_expression = self.parse_bitwise_or_expression()?;

            Ok(Expression::Binary(BinaryExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator: BinaryOperator::Bar,
                left: Box::new(bitwise_or_expression),
                right: Box::new(bitwise_xor_expression),
            }))
        } else {
            Ok(bitwise_xor_expression)
        }
    }

    // 13.13 Binary Logical Operators
    // https://tc39.es/ecma262/#prod-LogicalANDExpression
    fn parse_logical_and_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let bitwise_or_expression = self.parse_bitwise_or_expression()?;

        if self.current_token_kind() == TokenKind::LogicalAnd {
            self.advance(); // Eat the && token.

            let logical_and_expression = self.parse_logical_and_expression()?;

            Ok(Expression::Logical(LogicalExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator: LogicalOperator::And,
                left: Box::new(bitwise_or_expression),
                right: Box::new(logical_and_expression),
            }))
        } else {
            Ok(bitwise_or_expression)
        }
    }

    // https://tc39.es/ecma262/#prod-LogicalORExpression
    fn parse_logical_or_expression(&mut self) -> Result<Expression, ParserError> {
        let start_token = self.start_token();

        let logical_and_expression = self.parse_logical_and_expression()?;

        if self.current_token_kind() == TokenKind::LogicalOr {
            self.advance(); // Eat the || token.

            let logical_or_expression = self.parse_logical_or_expression()?;

            Ok(Expression::Logical(LogicalExpression {
                node: self.create_node(&start_token, &self.current_token),
                operator: LogicalOperator::Or,
                left: Box::new(logical_or_expression),
                right: Box::new(logical_and_expression),
            }))
        } else {
            Ok(logical_and_expression)
        }
    }

    // 13.15 Assignment Operators
    // https://tc39.es/ecma262/#prod-AssignmentExpression
    fn parse_assignment_expression(&mut self) -> Result<Expression, ParserError> {
        let left = self.parse_conditional_expression()?;

        Ok(left)
    }
}
