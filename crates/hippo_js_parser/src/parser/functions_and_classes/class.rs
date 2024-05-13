use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl Parser {
    // 15.7 Class Definitions
    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_declaration(&mut self) -> Result<ClassDeclaration, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Class))?;

        let binding_identifier = self.parse_binding_identifier()?;

        let super_class = if self.token_kind() == TokenKind::Keyword(KeywordKind::Extends) {
            Some(self.parse_class_heritage()?)
        } else {
            None
        };

        let class_tail = self.parse_class_tail()?;

        Ok(ClassDeclaration {
            node: self.end_node(start_index)?,
            // TODO Handle default exports properly which don't have an identifier.
            id: Some(binding_identifier),
            super_class,
            body: class_tail,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassExpression
    pub(crate) fn parse_class_expression(&mut self) -> Result<ClassExpression, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Class))?;

        let binding_identifier = if self.token_kind().is_binding_identifier() {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        let class_tail = self.parse_class_tail()?;

        Ok(ClassExpression {
            node: self.end_node(start_index)?,
            id: binding_identifier,
            super_class: None,
            body: class_tail,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassTail
    fn parse_class_tail(&mut self) -> Result<ClassBody, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let class_body = self.parse_class_body()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(ClassBody {
            node: self.end_node(start_index)?,
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
        let mut class_element_list = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            let Some(class_element) = self.parse_class_element()? else {
                continue;
            };

            class_element_list.push(class_element);
        }

        Ok(class_element_list)
    }

    // https://tc39.es/ecma262/#prod-ClassElement
    fn parse_class_element(&mut self) -> Result<Option<ClassBodyBody>, ParserError> {
        let mut is_static = false;

        if self.token_kind() == TokenKind::Semicolon {
            self.advance_any(); // Eat ';' token.

            return Ok(None);
        }

        let start_index = self.start_node(); // Start class element node.

        if self.token_kind() == TokenKind::Keyword(KeywordKind::Static) {
            // Cheap check to see if 'static' is being used as an identifier.
            if self.peek_token_kind() == TokenKind::Semicolon {
                let field_definition =
                    self.parse_field_definition(start_index, None, false, false)?;

                self.expect_optional_semicolon_and_advance();

                return Ok(Some(ClassBodyBody::PropertyDefinition(field_definition)));
            }

            is_static = true;

            if self.peek_token_kind() == TokenKind::LeftCurlyBrace {
                let static_block = self.parse_static_block()?;

                return Ok(Some(ClassBodyBody::StaticBlock(static_block)));
            } else {
                self.advance_any(); // Eat 'static' token.
            }
        }

        // TODO This could probably be a lot better to handle complex async and generator methods combinations.
        // let is_method_definition = match (self.token_kind(), self.peek_token_kind()) {
        //     (TokenKind::Keyword(KeywordKind::Async) | TokenKind::Keyword(KeywordKind::Get) | TokenKind::Keyword(KeywordKind::Set), _) => {
        //         let is_generator = self.peek_token_kind() == TokenKind::Multiplication;
        //         let is_class_element_name = self.peek_token_kind().is_class_element_name_start();
        //         let is_keyword_as_class_element_name =
        //             self.peek_token_kind() == TokenKind::LeftParenthesis;

        //         is_generator || is_class_element_name || is_keyword_as_class_element_name
        //     }
        //     (token_kind , _) if token_kind.is_class_element_name_start() => {
        //         self.peek_token_kind() == TokenKind::LeftParenthesis
        //     }
        //     (TokenKind::Multiplication, _) => true,
        //     _ => false,
        // };

        let method_definition_kind = self.parse_method_definition_kind();

        let is_computed = self.token_kind() == TokenKind::LeftSquareBracket;

        let optional_class_element_name = if self.token_kind().is_class_element_name_start()
            && !self
                .token_kind()
                .is_reserved_keyword_when_identifier_prohibited()
        {
            Some(self.parse_class_element_name()?)
        } else {
            None
        };

        if matches!(
            self.token_kind(),
            TokenKind::Assignment | TokenKind::Semicolon
        ) {
            let field_definition = self.parse_field_definition(
                start_index,
                optional_class_element_name,
                is_static,
                is_computed,
            )?;

            Ok(Some(ClassBodyBody::PropertyDefinition(field_definition)))
        } else {
            let method_definition = self.parse_method_definition(
                start_index,
                optional_class_element_name,
                method_definition_kind,
                is_static,
                is_computed,
            )?;

            Ok(Some(ClassBodyBody::MethodDefinition(method_definition)))
        }
    }

    // https://tc39.es/ecma262/#prod-FieldDefinition
    fn parse_field_definition(
        &mut self,
        start_index: usize,
        class_element_name: Option<PropertyDefinitionKey>,
        is_static: bool,
        is_computed: bool,
    ) -> Result<PropertyDefinition, ParserError> {
        let class_element_name = match class_element_name {
            Some(class_element_name) => class_element_name,
            None => self.parse_class_element_name()?,
        };

        let optional_assignment_expression = if self.token_kind() == TokenKind::Assignment {
            self.advance_any(); // Eat  '=' token.

            Some(self.parse_assignment_expression()?)
        } else {
            None
        };

        self.expect_optional_semicolon_and_advance();

        Ok(PropertyDefinition {
            node: self.end_node(start_index)?,
            is_static,
            computed: is_computed,
            key: Some(class_element_name),
            value: optional_assignment_expression,
        })
    }

    // https://tc39.es/ecma262/#prod-ClassElementName
    pub(crate) fn parse_class_element_name(
        &mut self,
    ) -> Result<PropertyDefinitionKey, ParserError> {
        match self.token_kind() {
            TokenKind::PrivateIdentifier => {
                let private_identifier = self.parse_private_identifier()?;

                Ok(PropertyDefinitionKey::PrivateIdentifier(private_identifier))
            }
            token_kind if token_kind.is_property_name() => {
                let property_name = self.parse_property_name()?;

                Ok(PropertyDefinitionKey::Expression(property_name))
            }
            _ => return Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ClassStaticBlock
    fn parse_static_block(&mut self) -> Result<StaticBlock, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Static))?;

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let statement_list = self.parse_statement_list()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(StaticBlock {
            node: self.end_node(start_index)?,
            body: statement_list,
        })
    }
}
