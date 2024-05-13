use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.11 The with Statement
    // https://tc39.es/ecma262/#prod-WithStatement
    pub(crate) fn parse_with_statement(&mut self) -> Result<WithStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::With))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let expression = self.parse_expression()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let statement = self.parse_statement()?;

        Ok(WithStatement {
            node: self.end_node(start_index)?,
            object: expression,
            body: Box::new(statement),
        })
    }
}
