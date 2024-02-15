use crate::{KeywordKind, Parser, ParserError, TokenKind, TokenValue};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.7 Class Definitions
    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_declaration(&mut self) -> Result<ClassDeclaration, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Class))?;

        let binding_identifier = self.parse_binding_identifier()?;

        let super_class =
            if self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::Extends) {
                Some(self.parse_class_heritage()?)
            } else {
                None
            };

        let class_tail = self.parse_class_tail()?;

        Ok(ClassDeclaration {
            node: self.end_node()?,
            // TODO Handle default exports properly which don't have an identifier.
            id: Some(binding_identifier),
            super_class,
            body: class_tail,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_class_expression")
    }

    // https://tc39.es/ecma262/#prod-ClassTail
    fn parse_class_tail(&mut self) -> Result<ClassBody, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let class_body = self.parse_class_body()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(ClassBody {
            node: self.end_node()?,
            body: class_body,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassHeritage
    fn parse_class_heritage(&mut self) -> Result<Expression, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Extends))?;

        self.parse_left_hand_side_expression()
    }

    // https://tc39.es/ecma262/#prod-ClassBody
    fn parse_class_body(&mut self) -> Result<Vec<ClassBodyBody>, ParserError> {
        // TODO This should parse method definitions.
        let mut class_element_list = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightCurlyBrace {
            let Some(class_element) = self.parse_class_element()? else {
                continue;
            };

            class_element_list.push(class_element);
        }

        Ok(class_element_list)
    }

    // https://tc39.es/ecma262/#prod-ClassElement
    fn parse_class_element(&mut self) -> Result<Option<ClassBodyBody>, ParserError> {
        let is_static = self.cursor.current_token_kind() == TokenKind::Keyword(KeywordKind::Static);
        let is_computed = self.cursor.current_token_kind() == TokenKind::LeftSquareBracket;

        match self.cursor.current_token_kind() {
            TokenKind::Semicolon => {
                self.cursor.advance(); // Eat ';' token.

                return Ok(None);
            }
            TokenKind::Keyword(KeywordKind::Static)
                if self.cursor.peek_token_kind() == TokenKind::LeftCurlyBrace =>
            {
                let static_block = self.parse_static_block()?;

                Ok(Some(ClassBodyBody::StaticBlock(static_block)))
            }
            // TODO Parse method definitions.
            current_token_kind if current_token_kind.is_class_element_name_start() => {
                if self.cursor.peek_token_kind() == TokenKind::LeftParenthesis {
                    Ok(Some(ClassBodyBody::MethodDefinition(
                        self.parse_method_definition()?,
                    )))
                } else {
                    Ok(Some(ClassBodyBody::PropertyDefinition(
                        self.parse_field_definition(is_static, is_computed)?,
                    )))
                }
            }
            _ => return Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-FieldDefinition
    fn parse_field_definition(
        &mut self,
        is_static: bool,
        is_computed: bool,
    ) -> Result<PropertyDefinition, ParserError> {
        self.start_node();

        let property_name = match self.cursor.current_token_kind() {
            TokenKind::PrivateIdentifier => {
                self.start_node();

                let TokenValue::String(token_value) = self.cursor.current_token_value() else {
                    return Err(ParserError::UnexpectedTokenValue);
                };

                self.cursor.advance(); // Eat private identifier token.

                PropertyDefinitionKey::PrivateIdentifier(PrivateIdentifier {
                    node: self.end_node()?,
                    name: token_value,
                })
            }
            TokenKind::LeftSquareBracket => {
                self.cursor.advance(); // Eat '[' token.

                let expression = self.parse_expression()?;

                self.expect_and_advance(TokenKind::RightSquareBracket)?;

                PropertyDefinitionKey::Expression(expression)
            }
            _ => PropertyDefinitionKey::Expression(self.parse_property_name()?),
        };

        let optional_assignment_expression =
            if self.cursor.current_token_kind() == TokenKind::Assignment {
                self.cursor.advance(); // Eat  '=' token.

                Some(self.parse_assignment_expression()?)
            } else {
                None
            };

        self.expect_optional_semicolon_and_advance(); // Not part of the spec but required by ESTree.

        Ok(PropertyDefinition {
            node: self.end_node()?,
            stattic: is_static,
            computed: is_computed,
            key: Some(property_name),
            value: optional_assignment_expression,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassStaticBlock
    fn parse_static_block(&mut self) -> Result<StaticBlock, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Static))?;

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let statement_list = self.parse_statement_list()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(StaticBlock {
            node: self.end_node()?,
            body: statement_list,
        })
    }
}
