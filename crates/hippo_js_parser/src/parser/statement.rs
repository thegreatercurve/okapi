use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // https://tc39.es/ecma262/#prod-Statement
    pub(crate) fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cursor.current_token_kind() {
            TokenKind::Keyword(keyword) => match keyword {
                KeywordKind::Let | KeywordKind::Const | KeywordKind::Var => {
                    self.parse_lexical_declaration()
                }
                KeywordKind::If => self.parse_if_statement(),
                KeywordKind::For => self.parse_for_or_for_of_in_statement(),
                KeywordKind::Do => self.parse_do_while_statement(),
                KeywordKind::While => self.parse_while_statement(),
                KeywordKind::Continue => self.parse_continue_statement(),
                KeywordKind::Break => self.parse_break_statement(),
                KeywordKind::Return => self.parse_return_statement(),
                KeywordKind::With => todo!(),
                KeywordKind::Throw => self.parse_throw_statement(),
                KeywordKind::Try => self.parse_try_statement(),
                KeywordKind::Switch => self.parse_switch_statement(),
                KeywordKind::Debugger => self.parse_debugger_statement(),
                _ => self.parse_expression_statement_or_labelled_statement(),
            },
            TokenKind::LeftCurlyBrace => self.parse_block_statement(),
            TokenKind::Semicolon => self.parse_empty_statement(),
            _ => self.parse_expression_statement_or_labelled_statement(),
        }
    }

    // https://tc39.es/ecma262/#prod-BlockStatement
    fn parse_block_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        // TODO parse block statement
        let mut body = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightCurlyBrace {
            body.push(self.parse_statement()?);
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Statement::Block(BlockStatement {
            node: self.end_node()?,
            body,
        }))
    }

    // https://tc39.es/ecma262/#prod-EmptyStatement
    fn parse_empty_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        Ok(Statement::Empty(EmptyStatement {
            node: self.end_node()?,
        }))
    }

    // https://tc39.es/ecma262/#prod-ExpressionStatement
    // https://tc39.es/ecma262/#prod-LabelledStatement
    fn parse_expression_statement_or_labelled_statement(
        &mut self,
    ) -> Result<Statement, ParserError> {
        self.start_node();

        let expression = self.parse_expression()?;

        if self.cursor.current_token_kind() == TokenKind::Colon {
            self.end_node()?;

            todo!("parse_labelled_statement")
        } else {
            Ok(Statement::Expression(ExpressionStatement {
                node: self.end_node()?,
                expression: expression,
            }))
        }
    }

    // https://tc39.es/ecma262/#prod-IfStatement
    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::If))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let test = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let consequent = self.parse_statement()?;

        let alternate = if self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::Else)
        {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::Else))?;

            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::If(IfStatement {
            node: self.end_node()?,
            test: test,
            consequent: Box::new(consequent),
            alternate,
        }))
    }

    // https://tc39.es/ecma262/#prod-ForStatement
    fn parse_for_or_for_of_in_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::For))?;

        todo!()
    }

    // https://tc39.es/ecma262/#prod-DoWhileStatement
    fn parse_do_while_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Do))?;

        let body = self.parse_statement()?;

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::While))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let test = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        Ok(Statement::DoWhile(DoWhileStatement {
            node: self.end_node()?,
            test: test,
            body: Box::new(body),
        }))
    }

    // https://tc39.es/ecma262/#prod-WhileStatement
    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::While))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let test = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let body = self.parse_statement()?;

        Ok(Statement::While(WhileStatement {
            node: self.end_node()?,
            test,
            body: Box::new(body),
        }))
    }

    // https://tc39.es/ecma262/#prod-ContinueStatement
    fn parse_continue_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Continue))?;

        let label = if self.cursor.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_label_identifier()?)
        };

        Ok(Statement::Continue(ContinueStatement {
            node: self.end_node()?,
            label,
        }))
    }

    // https://tc39.es/ecma262/#prod-BreakStatement
    fn parse_break_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Break))?;

        let label = if self.cursor.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_label_identifier()?)
        };

        Ok(Statement::Break(BreakStatement {
            node: self.end_node()?,
            label,
        }))
    }

    // https://tc39.es/ecma262/#prod-ReturnStatement
    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Return))?;

        let argument = if self.cursor.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };

        Ok(Statement::Return(ReturnStatement {
            node: self.end_node()?,
            argument,
        }))
    }

    // https://tc39.es/ecma262/#prod-ThrowStatement
    fn parse_throw_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Throw))?;

        let argument = self.parse_expression()?;

        Ok(Statement::Throw(ThrowStatement {
            node: self.end_node()?,
            argument,
        }))
    }

    // https://tc39.es/ecma262/#prod-TryStatement
    fn parse_try_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    // https://tc39.es/ecma262/#prod-SwitchStatement
    fn parse_switch_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    // https://tc39.github.io/ecma262/#sec-debugger-statement
    fn parse_debugger_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Debugger))?;

        Ok(Statement::Debugger(DebuggerStatement {
            node: self.end_node()?,
        }))
    }
}
