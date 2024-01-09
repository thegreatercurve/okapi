use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // https://tc39.es/ecma262/#prod-Statement
    pub(crate) fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token_kind() {
            TokenKind::Keyword(keyword) => match keyword {
                KeywordKind::Let | KeywordKind::Const | KeywordKind::Var => {
                    self.parse_lexical_declaration()
                }
                KeywordKind::If => self.parse_if_statement(),
                KeywordKind::For => self.parse_for_or_for_of_in_statement(),
                KeywordKind::Do => self.parse_do_while_statement(),
                KeywordKind::While => self.parse_while_statement(),
                KeywordKind::Return => self.parse_return_statement(),
                KeywordKind::Debugger => self.parse_debugger_statement(),
                _ => return Err(ParserError::UnexpectedToken(self.current_token_kind())),
            },

            TokenKind::LeftCurlyBrace => self.parse_block_statement(),
            TokenKind::Semicolon => self.parse_empty_statement(),

            _ => return Err(ParserError::UnexpectedToken(self.current_token_kind())),
        }
    }

    // https://tc39.es/ecma262/#prod-BlockStatement
    fn parse_block_statement(&mut self) -> Result<Statement, ParserError> {
        let node = self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let mut body = Vec::new();

        while self.current_token_kind() != TokenKind::RightCurlyBrace {
            body.push(self.parse_statement()?);
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Statement::Block(BlockStatement {
            node: self.finish_node(&node),
            body,
        }))
    }

    // https://tc39.es/ecma262/#prod-IfStatement
    fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        let node = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::If))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let test = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let consequent = self.parse_statement()?;

        let alternate = if self.current_token_kind() == TokenKind::Keyword(KeywordKind::Else) {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::Else))?;

            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(Statement::If(IfStatement {
            node: self.finish_node(&node),
            test: test,
            consequent: Box::new(consequent),
            alternate,
        }))
    }

    // https://tc39.es/ecma262/#prod-ForStatement
    fn parse_for_or_for_of_in_statement(&mut self) -> Result<Statement, ParserError> {
        let _node = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::For))?;

        todo!()
    }

    // https://tc39.es/ecma262/#prod-DoWhileStatement
    fn parse_do_while_statement(&mut self) -> Result<Statement, ParserError> {
        let node = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Do))?;

        let body = self.parse_statement()?;

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::While))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let test = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        Ok(Statement::DoWhile(DoWhileStatement {
            node: self.finish_node(&node),
            test: test,
            body: Box::new(body),
        }))
    }

    // https://tc39.es/ecma262/#prod-WhileStatement
    fn parse_while_statement(&mut self) -> Result<Statement, ParserError> {
        let node = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::While))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let test = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let body = self.parse_statement()?;

        Ok(Statement::While(WhileStatement {
            node: self.finish_node(&node),
            test,
            body: Box::new(body),
        }))
    }

    // https://tc39.es/ecma262/#prod-ReturnStatement
    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        let node = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Return))?;

        let argument = if self.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };

        self.expect_and_advance(TokenKind::Semicolon)?;

        Ok(Statement::Return(ReturnStatement {
            node: self.finish_node(&node),
            argument,
        }))
    }

    // https://tc39.es/ecma262/#prod-EmptyStatement
    fn parse_empty_statement(&mut self) -> Result<Statement, ParserError> {
        let node = self.start_node();

        self.expect_and_advance(TokenKind::Semicolon)?;

        Ok(Statement::Empty(EmptyStatement {
            node: self.finish_node(&node),
        }))
    }

    // 14.16 The `debugger` Statement
    // https://tc39.github.io/ecma262/#sec-debugger-statement
    fn parse_debugger_statement(&mut self) -> Result<Statement, ParserError> {
        let node = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Debugger))?;

        Ok(Statement::Debugger(DebuggerStatement {
            node: self.finish_node(&node),
        }))
    }
}
