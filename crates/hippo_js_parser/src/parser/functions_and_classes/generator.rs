use crate::{Parser, ParserError};
use hippo_estree::*;

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 15.5 Generator Function Definitions
    // https://tc39.es/ecma262/#prod-GeneratorExpression
    pub(crate) fn parse_generator_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_generator_expression")
    }

    // https://tc39.es/ecma262/#prod-YieldExpression
    pub(crate) fn parse_yield_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_yield_expression")
    }
}
