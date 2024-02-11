use crate::{Parser, ParserError};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.4 Method Definitions
    // https://tc39.es/ecma262/#sec-method-definitions
    pub(crate) fn parse_method_definition(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_method_definition")
    }
}
