use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 13.3 Left-Hand-Side Expressions
    // https://tc39.es/ecma262/#prod-LeftHandSideExpression
    pub(crate) fn parse_left_hand_side_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        let _node = self.start_token();

        let current_token_kind = self.cursor.current_token_kind();

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

        let primary_expression = self.parse_primary_expression();

        primary_expression
    }

    // https://tc39.es/ecma262/#prod-NewExpression
    fn parse_new_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::New))?;

        let callee = self.parse_member_expression()?;

        Ok(Expression::New(NewExpression {
            node: self.create_node(&start_token, &self.cursor.current_token),
            callee: Box::new(callee),
            arguments: vec![],
        }))
    }

    // https://tc39.es/ecma262/#prod-CallExpression
    fn parse_call_expression(&mut self) -> Result<Expression, ParserError> {
        // TODO This is currently incomplete.

        let start_token = self.start_token();

        let current_token_kind = self.cursor.current_token_kind();

        match current_token_kind {
            TokenKind::Keyword(KeywordKind::Super) => {
                self.cursor.advance(); // Eat the super token.

                let arguments = self.parse_arguments()?;

                return Ok(Expression::Call(CallExpression {
                    node: self.create_node(&start_token, &self.cursor.current_token),
                    callee: Box::new(CallExpressionCallee::Super(Super {
                        node: self.create_node(&start_token, &self.cursor.current_token),
                    })),
                    arguments,
                }));
            }
            TokenKind::Keyword(KeywordKind::Import) => {
                self.cursor.advance(); // Eat the import token.

                todo!()
            }
            TokenKind::LeftParenthesis => {
                let arguments = self.parse_arguments()?;

                let call_expression = self.parse_call_expression()?;

                return Ok(Expression::Call(CallExpression {
                    node: self.create_node(&start_token, &self.cursor.current_token),
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

        while self.cursor.current_token_kind() != TokenKind::RightParenthesis {
            let start_token = self.start_token();

            let is_spread = if self.cursor.current_token_kind() == TokenKind::Ellipsis {
                self.cursor.advance(); // Eat the ... token.

                true
            } else {
                false
            };

            let argument = self.parse_assignment_expression()?;

            if is_spread {
                arguments_list.push(CallExpressionArguments::SpreadElement(SpreadElement {
                    node: self.create_node(&start_token, &self.cursor.current_token),
                    argument,
                }));
            } else {
                arguments_list.push(CallExpressionArguments::Expression(argument));
            }

            if self.cursor.current_token_kind() != TokenKind::Comma {
                break;
            }

            if self.cursor.current_token_kind() == TokenKind::RightParenthesis {
                break;
            }

            self.cursor.advance();
        }

        Ok(arguments_list)
    }
}
