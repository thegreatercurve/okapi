use crate::{KeywordKind, Parser, ParserError, TokenKind};
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
        match self.cursor.current_token_kind() {
            TokenKind::Semicolon => {
                self.cursor.advance();

                return Ok(None);
            }
            TokenKind::Keyword(KeywordKind::Static) => {
                // TODO Check if is method definition, field definition or static block.
                todo!("parse_class_element")
            }
            _ => {
                todo!("parse_class_element")
            }
        }

        // Ok(Some())
    }
}
