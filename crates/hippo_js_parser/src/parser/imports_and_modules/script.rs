use crate::{Parser, ParserError};
use hippo_estree::*;
// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl<'a> Parser<'a> {
    // 16.1 Scripts
    // https://tc39.es/ecma262/#prod-Script
    pub(crate) fn parse_script_body(&mut self) -> Result<ProgramBody, ParserError> {
        // TODO Parse parser statement of declaration.
        let statement_list_item = self.parse_statement()?;

        Ok(ProgramBody::StatementList(vec![
            StatementListItem::Statement(statement_list_item),
        ]))
    }
}
