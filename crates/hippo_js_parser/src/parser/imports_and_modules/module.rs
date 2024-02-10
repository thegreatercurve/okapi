use crate::{Parser, ParserError};
use hippo_estree::*;

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl<'a> Parser<'a> {
    // 16.2 Module
    // https://tc39.es/ecma262/#prod-ModuleBody
    pub(crate) fn parse_module_body(&mut self) -> Result<Vec<StatementListItem>, ParserError> {
        todo!("parse_module_body")
    }
}
