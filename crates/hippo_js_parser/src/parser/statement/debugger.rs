use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.13 The debugger Statement
    // https://tc39.github.io/ecma262/#sec-debugger-statement
    pub(crate) fn parse_debugger_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Debugger))?;

        self.expect_and_advance(TokenKind::Semicolon)?;

        Ok(Statement::Debugger(DebuggerStatement {
            node: self.end_node()?,
        }))
    }
}
