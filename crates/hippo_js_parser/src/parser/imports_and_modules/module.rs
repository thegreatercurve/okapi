use crate::{Parser, ParserError};
use hippo_estree::*;

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl<'a> Parser<'a> {
    // 16.2 Module
    // https://tc39.es/ecma262/#prod-ModuleBody
    pub(crate) fn parse_module_body(&mut self) -> Result<ProgramBody, ParserError> {
        // TODO Parse parser statement of declaration.
        let statement_list_item: Statement = self.parse_statement()?;

        Ok(ProgramBody::Module(vec![ModuleItem::StatementListItem(
            StatementListItem::Statement(statement_list_item),
        )]))
    }
}
