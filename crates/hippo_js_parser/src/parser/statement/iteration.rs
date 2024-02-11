use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.7 Iteration Statements
    // https://tc39.es/ecma262/#sec-iteration-statements
    fn parse_iterartion_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cursor.current_token_kind() {
            TokenKind::Keyword(KeywordKind::Do) => self.parse_do_while_statement(),
            TokenKind::Keyword(KeywordKind::While) => self.parse_while_statement(),
            TokenKind::Keyword(KeywordKind::For) => self.parse_for_or_for_of_in_statement(),
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // 14.7.4 The for Statement
    // https://tc39.es/ecma262/#prod-ForStatement
    pub(crate) fn parse_for_or_for_of_in_statement(&mut self) -> Result<Statement, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::For))?;

        todo!("parse_for_or_for_of_in_statement")
    }

    // 14.7.2 The do-while Statement
    // https://tc39.es/ecma262/#prod-DoWhileStatement
    pub(crate) fn parse_do_while_statement(&mut self) -> Result<Statement, ParserError> {
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

    // 14.7.3 The while Statement
    // https://tc39.es/ecma262/#prod-WhileStatement
    pub(crate) fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
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
}
