use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.8 The continue Statement
    // https://tc39.es/ecma262/#prod-ContinueStatement
    pub(crate) fn parse_continue_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Continue))?;

        let label = if self.cursor.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_label_identifier()?)
        };

        self.expect_optional_semicolon_and_advance();

        Ok(Statement::Continue(ContinueStatement {
            node: self.end_node()?,
            label,
        }))
    }
}
