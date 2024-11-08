use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, Token, TokenKind};

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
        TokenKind::GreaterThanOrEqual => Some(BinaryOperator::GreaterThanEqual),
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

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
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

    // Use a Pratt Parser for concision and speed.
    pub(crate) fn parse_binary_expression(
        &mut self,
        precedence: u8,
    ) -> Result<Expression, ParserError> {
        let start_token = self.cursor.current_token.clone();

        // https://tc39.es/ecma262/#prod-RelationalExpression
        if !self.params.has_allow_in() && self.token_kind().is_private_identifier() {
            let private_identifier = self.parse_private_identifier()?;

            self.expect_and_advance(TokenKind::Keyword(KeywordKind::In))?;

            let unary_expression = self.parse_unary_expression()?;

            let node = Node::new(start_token.start, self.cursor.previous_token.end);

            return Ok(Expression::Binary(BinaryExpression {
                node,
                left: BinaryExpressionLeft::PrivateIdentifier(private_identifier),
                operator: BinaryOperator::In,
                right: Box::new(unary_expression),
            }));
        };

        let unary_expression = self.parse_unary_expression()?;

        self.parse_binary_expression_recursive_impl(unary_expression, &start_token, precedence)
    }

    fn parse_binary_expression_recursive_impl(
        &mut self,
        mut left_expression: Expression,
        left_start_token: &Token,
        minimum_precedence: u8,
    ) -> Result<Expression, ParserError> {
        while self.token_kind() != TokenKind::EOF {
            let token_kind = self.token_kind();

            let Some((left_precedence, right_precedence)) =
                match_token_kind_to_operator_precedence(&self.token_kind())
            else {
                break;
            };

            if left_precedence < minimum_precedence {
                break;
            }

            // The [In] grammar parameter is needed to avoid confusing the in operator in a relational expression with the in operator in a for statement.
            // https://tc39.es/ecma262/#sec-relational-operators
            if self.params.has_allow_in()
                && self.token_kind() == TokenKind::Keyword(KeywordKind::In)
            {
                break;
            }

            self.advance_any(); // Eat operator token.

            let right_expression = self.parse_binary_expression(right_precedence)?;

            let node = Node::new(left_start_token.start, self.cursor.previous_token.end);

            if token_kind.is_logical_operator() {
                let Some(operator) = match_token_kind_to_logical_operator(&token_kind) else {
                    break;
                };

                left_expression = Expression::Logical(LogicalExpression {
                    node,
                    operator,
                    left: Box::new(left_expression),
                    right: Box::new(right_expression),
                });
            } else if token_kind.is_binary_operator() {
                let Some(operator) = match_token_kind_to_binary_operator(&token_kind) else {
                    break;
                };

                left_expression = Expression::Binary(BinaryExpression {
                    node,
                    operator,
                    left: BinaryExpressionLeft::Expression(Box::new(left_expression)),
                    right: Box::new(right_expression),
                });
            } else {
                break;
            }
        }

        Ok(left_expression)
    }
}
