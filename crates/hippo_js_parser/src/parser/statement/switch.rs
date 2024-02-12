use crate::{Parser, ParserError};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.14 The switch Statement
    // https://tc39.es/ecma262/#prod-SwitchStatement
    pub(crate) fn parse_switch_statement(&mut self) -> Result<Statement, ParserError> {
        todo!("parse_switch_statement")
    }
}
