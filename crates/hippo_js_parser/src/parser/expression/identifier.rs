use crate::ast::*;
use crate::{Parser, ParserError};

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 13.1 Identifiers
    // https://tc39.es/ecma262/#prod-IdentifierReference
    pub(crate) fn parse_identifier_reference(&mut self) -> Result<Expression, ParserError> {
        let start_index = self.start_node();

        let identifier_reference = String::from(self.token_value());

        if self.token_kind().is_identifier_reference() {
            self.advance_any(); // Eat identifier reference token.
        }

        Ok(Expression::Identifier(Identifier {
            node: self.end_node(start_index)?,
            name: identifier_reference,
        }))
    }

    // https://tc39.es/ecma262/#prod-BindingIdentifier
    pub(crate) fn parse_binding_identifier(&mut self) -> Result<Identifier, ParserError> {
        let start_index = self.start_node();

        let binding_identifier = String::from(self.token_value());

        if self.token_kind().is_binding_identifier() {
            self.advance_any(); // Eat binding identifier token.
        }

        Ok(Identifier {
            node: self.end_node(start_index)?,
            name: binding_identifier,
        })
    }

    pub(crate) fn parse_label_identifier(&mut self) -> Result<Identifier, ParserError> {
        self.parse_binding_identifier()
    }
}
