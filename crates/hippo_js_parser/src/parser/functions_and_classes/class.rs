use crate::{Parser, ParserError};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.7 Class Definitions
    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_declaration(&mut self) -> Result<ClassDeclaration, ParserError> {
        todo!("parse_class_declaration")
    }

    pub(crate) fn parse_class_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_class_expression")
    }
}
