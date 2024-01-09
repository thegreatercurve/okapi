use crate::{Parser, ParserError};
use hippo_estree::*;

impl<'a> Parser<'a> {
    pub(crate) fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        todo!()
    }

    pub(crate) fn parse_label_identifier(&mut self) -> Result<Identifier, ParserError> {
        todo!()
    }
}
