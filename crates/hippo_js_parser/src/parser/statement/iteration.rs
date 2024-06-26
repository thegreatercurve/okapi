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

        // `for await`
        if self.params.has_allow_await()
            && self.token_kind() == TokenKind::Keyword(KeywordKind::Await)
        {
            is_async = true;

            self.advance_any();
        }

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let previous_cursor = self.cursor.clone();

        // `for ( LexicalDeclaration Expression`
        let is_head_lexical_declaration = self.token_kind().is_lexical_declaration_start();

        // Early check to see if we potentially have to reparse these as a `AssignmentPattern`.
        // `for ( {} in|of`
        // `for ( [] in|of`
        let is_head_object_or_array_literal = self.token_kind().is_binding_pattern_start();

        // `for ( ;`
        if self.token_kind() == TokenKind::Semicolon {
            self.advance_any(); // Eat `;` token.

            return Ok(ForStatementKind::Classic(
                self.parse_classic_for_statement(start_index, None)?,
            ));
        }

        let left_expression_or_lexical_declaration = match self.token_kind() {
            // `for ( var VariableDeclarationList`
            token_kind if token_kind.is_variable_declaration_start() => {
                let variable_statement: VariableDeclaration =
                    self.parse_variable_statement(false)?;

                ForStatementInit::VariableDeclaration(variable_statement)
            }
            // `for ( LexicalDeclaration Expression`
            token_kind if token_kind.is_lexical_declaration_start() => {
                let lexical_declaration = self.parse_lexical_declaration(false)?;

                ForStatementInit::VariableDeclaration(lexical_declaration)
            }
            _ => {
                if is_async {
                    // `for await ( LeftHandSideExpression`

                    let left_hand_side_expression = self.with_params(
                        self.params.clone().add_allow_in(true),
                        Self::parse_left_hand_side_expression,
                    )?;

                    ForStatementInit::Expression(left_hand_side_expression)
                } else {
                    // `for ( Expression`
                    let expression: Expression = self.with_params(
                        self.params.clone().add_allow_in(true),
                        Self::parse_expression,
                    )?;

                    ForStatementInit::Expression(expression)
                }
            }
        };

        match self.token_kind() {
            TokenKind::Keyword(KeywordKind::In) | TokenKind::Keyword(KeywordKind::Of) => self
                .parse_for_in_of_statement(
                    start_index,
                    left_expression_or_lexical_declaration,
                    is_head_object_or_array_literal,
                    previous_cursor,
                    is_async,
                ),
            TokenKind::Semicolon => {
                self.advance_any(); // Eat `;` token.

                Ok(ForStatementKind::Classic(
                    self.parse_classic_for_statement(
                        start_index,
                        Some(left_expression_or_lexical_declaration),
                    )?,
                ))
            }
            // `for ( LexicalDeclaration Expression`
            _ if is_head_lexical_declaration => Ok(ForStatementKind::Classic(
                self.parse_classic_for_statement(
                    start_index,
                    Some(left_expression_or_lexical_declaration),
                )?,
            )),
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    fn parse_classic_for_statement(
        &mut self,
        start_index: usize,
        optional_initializer: Option<ForStatementInit>,
    ) -> Result<ForStatement, ParserError> {
        let optional_test_expression = if self.token_kind() != TokenKind::Semicolon {
            let expression = self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_expression,
            )?;

            Some(expression)
        } else {
            None
        };

        self.expect_and_advance(TokenKind::Semicolon)?;

        let optional_update_expression = if self.token_kind() != TokenKind::RightParenthesis {
            let expression = self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_expression,
            )?;

            Some(expression)
        } else {
            None
        };

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let statement_body = self.parse_statement()?;

        Ok(ForStatement {
            node: self.end_node(start_index)?,
            init: optional_initializer,
            test: optional_test_expression,
            update: optional_update_expression,
            body: Box::new(statement_body),
        })
    }

    // https://tc39.es/ecma262/#prod-ForInOfStatement
    fn parse_for_in_of_statement(
        &mut self,
        start_index: usize,
        left_expression_init: ForStatementInit,
        is_left_assignment_pattern: bool,
        previous_cursor: Cursor,
        is_async: bool,
    ) -> Result<ForStatementKind, ParserError> {
        let left_expression = match left_expression_init {
            // `for ( {} in|of`
            // `for ( [] in|of`
            _ if is_left_assignment_pattern => {
                // If LeftHandSideExpression is either an ObjectLiteral or an ArrayLiteral, LeftHandSideExpression must cover an AssignmentPattern.
                // https://tc39.es/ecma262/#sec-for-in-and-for-of-statements-static-semantics-early-errors
                self.cursor = previous_cursor;

                let assignment_pattern = match self.parse_binding_pattern()? {
                    BindingPattern::Object(object_pattern) => Pattern::Object(object_pattern),
                    BindingPattern::Array(array_pattern) => Pattern::Array(array_pattern),
                };

                ForInStatementLeft::Pattern(assignment_pattern)
            }
            // `for ( const foo in|of`
            // `for ( foo.bar in|of`
            for_statement_init => ForInStatementLeft::from(for_statement_init),
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
            self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_expression,
            )?
        } else {
            self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_assignment_expression,
            )?
        };

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let statement_body = self.parse_statement()?;

        let node = self.end_node(start_index)?;

        let body = Box::new(statement_body);

        if is_for_in {
            Ok(ForStatementKind::In(ForInStatement {
                node,
                left: left_expression,
                right: expression_or_assignment_expression,
                body,
            }))
        } else {
            Ok(ForStatementKind::Of(ForOfStatement {
                node,
                left: left_expression,
                right: expression_or_assignment_expression,
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

        let expression = self.with_params(
            self.params.clone().add_allow_in(false),
            Self::parse_expression,
        )?;

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

        let expression = self.with_params(
            self.params.clone().add_allow_in(false),
            Self::parse_expression,
        )?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let statement = self.parse_statement()?;

        Ok(WhileStatement {
            node: self.end_node(start_index)?,
            test: expression,
            body: Box::new(statement),
        })
    }
}
