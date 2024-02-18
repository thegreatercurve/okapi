use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.6 The if Statement
    // https://tc39.es/ecma262/#prod-IfStatement
    pub(crate) fn parse_if_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::If))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let test = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let consequent = self.parse_statement()?;

        let alternate = if self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::Else)
        {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::Else))?;

            Some(self.parse_statement()?)
        } else {
            None
        };

        Ok(Statement::If(Box::new(IfStatement {
            node: self.end_node()?,
            test: test,
            consequent,
            alternate,
        })))
    }
}
