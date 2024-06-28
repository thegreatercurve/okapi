use crate::ast::*;
use crate::{Parser, ParserError};

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl Parser {
    // 13.1 Identifiers
    // https://tc39.es/ecma262/#prod-IdentifierReference
    pub(crate) fn parse_identifier_reference(&mut self) -> Result<Identifier, ParserError> {
        let start_index = self.start_node();

        let identifier_reference = String::from(self.token_value());

        if self.token_kind().is_identifier_reference() {
            self.advance_any(); // Eat identifier reference token.
        } else {
            return Err(self.unexpected_current_token_kind());
        }

        Ok(Identifier {
            node: self.end_node(start_index)?,
            name: identifier_reference,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingIdentifier
    pub(crate) fn parse_binding_identifier(&mut self) -> Result<Identifier, ParserError> {
        let start_index = self.start_node();
        let binding_identifier = String::from(self.token_value());

        if self.token_kind().is_binding_identifier() {
            self.advance_any(); // Eat binding identifier token.
        } else {
            return Err(self.unexpected_current_token_kind());
        }

        Ok(Identifier {
            node: self.end_node(start_index)?,
            name: binding_identifier,
        })
    }

    // https://tc39.es/ecma262/#prod-LabelIdentifier
    pub(crate) fn parse_label_identifier(&mut self) -> Result<Identifier, ParserError> {
        self.parse_binding_identifier()
    }

    // 12.7 Names and Keywords
    // https://tc39.es/ecma262/#prod-PrivateIdentifier
    pub(crate) fn parse_private_identifier(&mut self) -> Result<PrivateIdentifier, ParserError> {
        let start_index = self.start_node();

        let private_idententifier = String::from(self.token_value());

        if self.token_kind().is_private_identifier() {
            self.advance_any(); // Eat private identifier token.
        } else {
            return Err(self.unexpected_current_token_kind());
        }

        Ok(PrivateIdentifier {
            node: self.end_node(start_index)?,
            name: private_idententifier,
        })
    }

    // https://tc39.es/ecma262/#prod-IdentifierName
    pub(crate) fn parse_identifier_name(&mut self) -> Result<Identifier, ParserError> {
        let start_index = self.start_node();

        let identifier_name = String::from(self.token_value());

        if self.token_kind().is_identifier_name() {
            self.advance_any(); // Eat identifier or reserved keyword token.
        } else {
            return Err(self.unexpected_current_token_kind());
        }

        Ok(Identifier {
            node: self.end_node(start_index)?,
            name: identifier_name,
        })
    }
}
