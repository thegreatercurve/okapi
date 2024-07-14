use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.8 The continue Statement
    // https://tc39.es/ecma262/#prod-ContinueStatement
    pub(crate) fn parse_continue_statement(&mut self) -> Result<ContinueStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Continue))?;

        let label = if self.token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_label_identifier()?)
        };

        if self.has_previous_token_line_terminator() {
            return Err(ParserError::UnexpectedLineTerminator);
        }

        self.expect_optional_semicolon_and_advance();

        Ok(ContinueStatement {
            node: self.end_node(start_index)?,
            label,
        })
    }
}
