use crate::{Parser, ParserError};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.3 Arrow Function Definitions
    // https://tc39.es/ecma262/#sec-arrow-function-definitions
    pub(crate) fn parse_arrow_function(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_arrow_function")
    }
}