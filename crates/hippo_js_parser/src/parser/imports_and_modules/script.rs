use crate::ast::*;
use crate::{Parser, ParserError, TokenKind};
// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl Parser {
    // 16.1 Scripts
    // https://tc39.es/ecma262/#prod-Script
    pub(crate) fn parse_script_body(&mut self) -> Result<ProgramBody, ParserError> {
        let mut statement_list = vec![];

        while self.token_kind() != TokenKind::EOF {
            statement_list.push(self.parse_statement_list_item()?);
        }

        Ok(ProgramBody::StatementList(statement_list))
    }
}
