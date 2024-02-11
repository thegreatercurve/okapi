mod function;
mod generator;
mod parameter_lists;

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

    // 15.4 Method Definitions
    // https://tc39.es/ecma262/#sec-method-definitions
    pub(crate) fn parse_method_definition(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_method_definition")
    }

    // 15.6 Async Generator Function Definitions
    // https://tc39.es/ecma262/#prod-AsyncGeneratorExpression
    pub(crate) fn parse_async_generator_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_async_generator_expression")
    }

    // 15.7 Class Definitions
    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_declaration(&mut self) -> Result<ClassDeclaration, ParserError> {
        todo!("parse_class_declaration")
    }

    pub(crate) fn parse_class_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_class_expression")
    }

    // 15.8 Async Function Definitions
    // https://tc39.es/ecma262/#prod-AsyncFunctionExpression
    pub fn parse_async_function_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_async_function_expression")
    }
}
