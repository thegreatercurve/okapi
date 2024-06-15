use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.6 The if Statement
    // https://tc39.es/ecma262/#prod-IfStatement
    pub(crate) fn parse_if_statement(&mut self) -> Result<IfStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::If))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let test = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let consequent = self.parse_statement()?;

        let alternate = if self.token_kind() == TokenKind::Keyword(KeywordKind::Else) {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::Else))?;

            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };

        Ok(IfStatement {
            node: self.end_node(start_index)?,
            test,
            consequent: Box::new(consequent),
            alternate,
        })
    }
}
