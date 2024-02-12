use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.9 The break Statement
    // https://tc39.es/ecma262/#prod-BreakStatement
    pub(crate) fn parse_break_statement(&mut self) -> Result<Statement, ParserError> {
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
}
