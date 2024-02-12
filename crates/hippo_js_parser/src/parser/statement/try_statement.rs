use crate::{Parser, ParserError};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.12 The try Statement
    // https://tc39.es/ecma262/#prod-TryStatement
    pub(crate) fn parse_try_statement(&mut self) -> Result<Statement, ParserError> {
        todo!("parse_try_statement")
    }
}
