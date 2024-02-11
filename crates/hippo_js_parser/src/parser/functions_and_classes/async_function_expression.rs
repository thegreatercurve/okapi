use crate::{Parser, ParserError};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.8 Async Function Definitions
    // https://tc39.es/ecma262/#prod-AsyncFunctionDeclaration
    pub(crate) fn parse_async_function_declaration(
        &mut self,
    ) -> Result<FunctionDeclaration, ParserError> {
        todo!("parse_async_function_declaration")
    }

    // https://tc39.es/ecma262/#prod-AsyncFunctionExpression
    pub(crate) fn parse_async_function_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_async_function_expression")
    }
}
