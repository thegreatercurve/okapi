use crate::{Parser, ParserError};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.1 Parameter Lists
    // https://tc39.es/ecma262/#sec-parameter-lists
    pub(crate) fn parse_formal_parameters(
        &mut self,
    ) -> Result<Vec<FunctionParameter>, ParserError> {
        // todo!("parse_formal_parameters");

        Ok(vec![])
    }
}
