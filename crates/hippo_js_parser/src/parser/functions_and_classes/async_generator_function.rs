use crate::{Parser, ParserError};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.6 Async Generator Function Definitions
    // https://tc39.es/ecma262/#prod-AsyncGeneratorExpression
    pub(crate) fn parse_async_generator_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_async_generator_expression")
    }
}
