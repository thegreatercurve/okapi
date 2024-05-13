use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.11 The throw Statement
    // https://tc39.es/ecma262/#prod-ThrowStatement
    pub(crate) fn parse_throw_statement(&mut self) -> Result<ThrowStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Throw))?;

        if self.has_previous_token_line_terminator() {
            return Err(ParserError::UnexpectedLineTerminator);
        }

        let argument = self.parse_expression()?;

        self.expect_optional_semicolon_and_advance();

        Ok(ThrowStatement {
            node: self.end_node(start_index)?,
            argument,
        })
    }
}
