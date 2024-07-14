use crate::ast::*;
use crate::{Parser, ParserError};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.4 Empty Statement
    // https://tc39.es/ecma262/#prod-EmptyStatement
    pub(crate) fn parse_empty_statement(&mut self) -> Result<Statement, ParserError> {
        let start_index = self.start_node();

        self.expect_optional_semicolon_and_advance();

        Ok(Statement::Empty(EmptyStatement {
            node: self.end_node(start_index)?,
        }))
    }
}
