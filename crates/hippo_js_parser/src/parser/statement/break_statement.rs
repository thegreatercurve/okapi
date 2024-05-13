use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.9 The break Statement
    // https://tc39.es/ecma262/#prod-BreakStatement
    pub(crate) fn parse_break_statement(&mut self) -> Result<BreakStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Break))?;

        let label = if self.token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_label_identifier()?)
        };

        self.expect_optional_semicolon_and_advance();

        Ok(BreakStatement {
            node: self.end_node(start_index)?,
            label,
        })
    }
}
