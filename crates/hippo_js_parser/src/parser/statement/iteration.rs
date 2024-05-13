use crate::ast::*;
use crate::{Cursor, KeywordKind, Parser, ParserError, TokenKind};
pub(crate) enum ForStatementKind {
    Classic(ForStatement),
    In(ForInStatement),
    Of(ForOfStatement),
}

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.7 Iteration Statements
    // https://tc39.es/ecma262/#sec-iteration-statements
    // 14.7.4 The for Statement
    // https://tc39.es/ecma262/#prod-ForStatement
    pub(crate) fn parse_for_or_for_of_in_statement(
        &mut self,
    ) -> Result<ForStatementKind, ParserError> {
        let mut is_async = false;

        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::For))?;

        if self.token_kind() == TokenKind::Keyword(KeywordKind::Await) {
            is_async = true;

            self.advance_any();
        }

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let previous_cursor = self.cursor.clone();

        let is_head_object_or_array_literal = self.token_kind().is_binding_pattern_start();

        let optional_lexical_declaration = match self.token_kind() {
            token_kind if token_kind.is_lexical_declaration_start() => Some(
                ForStatementInit::VariableDeclaration(self.parse_lexical_declaration(false)?),
            ),
            TokenKind::Semicolon => None,
            _ => {
                // Prevent the in operator being parsed as a relational expression.
                self.context.allow_in = false;

                let expression = self.parse_expression()?;

                self.context.allow_in = true;

                Some(ForStatementInit::Expression(expression))
            }
        };

        if self.token_kind() == TokenKind::Semicolon {
            return Ok(ForStatementKind::Classic(
                self.parse_classic_for_statement(start_index, optional_lexical_declaration)?,
            ));
        }

        match self.token_kind() {
            TokenKind::Keyword(KeywordKind::In) | TokenKind::Keyword(KeywordKind::Of) => self
                .parse_for_in_of_statement(
                    start_index,
                    optional_lexical_declaration,
                    previous_cursor,
                    is_head_object_or_array_literal,
                    is_async,
                ),
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    fn parse_classic_for_statement(
        &mut self,
        start_index: usize,
        lexical_declaration_initializer: Option<ForStatementInit>,
    ) -> Result<ForStatement, ParserError> {
        self.expect_and_advance(TokenKind::Semicolon)?;

        let optional_test_expression = if self.token_kind() != TokenKind::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_and_advance(TokenKind::Semicolon)?;

        let optional_update_expression = if self.token_kind() != TokenKind::RightParenthesis {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let statement_body = self.parse_statement()?;

        Ok(ForStatement {
            node: self.end_node(start_index)?,
            init: lexical_declaration_initializer,
            test: optional_test_expression,
            update: optional_update_expression,
            body: Box::new(statement_body),
        })
    }

    // https://tc39.es/ecma262/#prod-ForInOfStatement
    fn parse_for_in_of_statement(
        &mut self,
        start_index: usize,
        optional_lexical_declaration: Option<ForStatementInit>,
        previous_cursor: Cursor,
        is_head_object_or_array_literal: bool,
        is_async: bool,
    ) -> Result<ForStatementKind, ParserError> {
        let optional_left_hand_expression = match optional_lexical_declaration {
            _ if is_head_object_or_array_literal => {
                // If LeftHandSideExpression is either an ObjectLiteral or an ArrayLiteral, LeftHandSideExpression must cover an AssignmentPattern.
                // https://tc39.es/ecma262/#sec-for-in-and-for-of-statements-static-semantics-early-errors
                self.cursor = previous_cursor;

                let assignment_pattern = match self.parse_binding_pattern()? {
                    BindingPattern::Object(object_pattern) => Pattern::Object(object_pattern),
                    BindingPattern::Array(array_pattern) => Pattern::Array(array_pattern),
                };

                ForInStatementLeft::Pattern(assignment_pattern)
            }
            Some(for_statement_init) => ForInStatementLeft::from(for_statement_init),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        let is_for_in = self.token_kind() == TokenKind::Keyword(KeywordKind::In);

        if is_for_in && is_async {
            return Err(ParserError::InvalidAwaitForInStatement);
        }

        if is_for_in {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::In))?;
        } else {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::Of))?;
        };

        let expression_or_assignment_expression = if is_for_in {
            self.parse_expression()?
        } else {
            self.parse_assignment_expression()?
        };

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let statement_body = self.parse_statement()?;

        let node = self.end_node(start_index)?;
        let left = optional_left_hand_expression;
        let right = expression_or_assignment_expression;
        let body = Box::new(statement_body);

        if is_for_in {
            Ok(ForStatementKind::In(ForInStatement {
                node,
                left,
                right,
                body,
            }))
        } else {
            Ok(ForStatementKind::Of(ForOfStatement {
                node,
                left,
                right,
                body,
                awaiting: is_async,
            }))
        }
    }

    // 14.7.2 The do-while Statement
    // https://tc39.es/ecma262/#prod-DoWhileStatement
    pub(crate) fn parse_do_while_statement(&mut self) -> Result<DoWhileStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Do))?;

        let body = self.parse_statement()?;

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::While))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let expression = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        self.expect_optional_semicolon_and_advance();

        Ok(DoWhileStatement {
            node: self.end_node(start_index)?,
            test: expression,
            body: Box::new(body),
        })
    }

    // 14.7.3 The while Statement
    // https://tc39.es/ecma262/#prod-WhileStatement
    pub(crate) fn parse_while_statement(&mut self) -> Result<WhileStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::While))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let expression = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let statement = self.parse_statement()?;

        Ok(WhileStatement {
            node: self.end_node(start_index)?,
            test: expression,
            body: Box::new(statement),
        })
    }
}
