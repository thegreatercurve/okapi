use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14.3 Declarations and the Variable Statement
// https://tc39.es/ecma262/#sec-declarations-and-the-variable-statement
impl<'a> Parser<'a> {
    // 14.3.1 Let and Const Declarations
    // https://tc39.es/ecma262/#prod-LexicalDeclaration
    pub(crate) fn parse_lexical_declaration(&mut self) -> Result<VariableDeclaration, ParserError> {
        self.start_node();

        let kind = match self.cursor.current_token_kind() {
            TokenKind::Keyword(KeywordKind::Let) => VariableKind::Let,
            TokenKind::Keyword(KeywordKind::Const) => VariableKind::Const,
            TokenKind::Keyword(KeywordKind::Var) => VariableKind::Var,
            _ => return Err(self.unexpected_current_token_kind()),
        };

        if kind == VariableKind::Const {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::Const))?;
        } else if kind == VariableKind::Let {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::Let))?;
        } else if kind == VariableKind::Var {
            self.expect_and_advance(TokenKind::Keyword(KeywordKind::Var))?;
        }

        let declarations = self.parse_binding_list()?;

        if kind == VariableKind::Const {
            // TODO Check const declarations have a valid identifier.
        }

        self.expect_optional_semicolon_and_advance();

        Ok(VariableDeclaration {
            node: self.end_node()?,
            declarations,
            kind,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingList
    fn parse_binding_list(&mut self) -> Result<Vec<VariableDeclarator>, ParserError> {
        let mut declarations = vec![self.parse_binding_identifier_or_binding_pattern()?];

        while self.cursor.current_token_kind() == TokenKind::Comma {
            self.expect_and_advance(TokenKind::Comma)?; // Eat ',' token.

            declarations.push(self.parse_binding_identifier_or_binding_pattern()?);
        }

        Ok(declarations)
    }

    // https://tc39.es/ecma262/#prod-BindingIdentifier
    // https://tc39.es/ecma262/#prod-BindingPattern
    fn parse_binding_identifier_or_binding_pattern(
        &mut self,
    ) -> Result<VariableDeclarator, ParserError> {
        self.start_node();

        let current_token_kind = self.cursor.current_token_kind();

        let binding_identifier = match current_token_kind {
            TokenKind::Identifier => Pattern::Identifier(self.parse_binding_identifier()?),
            TokenKind::LeftCurlyBrace => Pattern::Object(self.parse_object_binding_pattern()?),
            TokenKind::LeftSquareBracket => Pattern::Array(self.parse_array_binding_pattern()?),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        let initializer = if self.cursor.current_token_kind() == TokenKind::Assignment {
            self.expect_and_advance(TokenKind::Assignment)?; // Eat '=' token.

            Some(self.parse_expression()?)
        } else {
            None
        };

        Ok(VariableDeclarator {
            node: self.end_node()?,
            id: binding_identifier,
            init: initializer,
        })
    }

    // 14.3.3 Destructuring Binding Patterns
    // https://tc39.es/ecma262/#prod-BindingPattern
    pub(crate) fn parse_binding_pattern(&mut self) -> Result<BindingPattern, ParserError> {
        let current_token_kind = self.cursor.current_token_kind();

        match current_token_kind {
            TokenKind::LeftCurlyBrace => Ok(BindingPattern::ObjectPattern(
                self.parse_object_binding_pattern()?,
            )),
            TokenKind::LeftSquareBracket => Ok(BindingPattern::ArrayPattern(
                self.parse_array_binding_pattern()?,
            )),
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ObjectBindingPattern
    fn parse_object_binding_pattern(&mut self) -> Result<ObjectPattern, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?; // Eat '{' token.

        let mut properties = self.parse_binding_property_list()?;

        if self.cursor.current_token_kind() == TokenKind::Ellipsis {
            properties.push(self.parse_binding_rest_property()?);
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?; // Eat '}' token.

        Ok(ObjectPattern {
            node: self.end_node()?,
            properties,
        })
    }

    // https://tc39.es/ecma262/#prod-ArrayBindingPattern
    fn parse_array_binding_pattern(&mut self) -> Result<ArrayPattern, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftSquareBracket)?; // Eat '[' token.

        let mut elements = self.parse_binding_element_list()?;

        if self.cursor.current_token_kind() == TokenKind::Ellipsis {
            elements.push(Some(self.parse_binding_rest_element()?));
        }

        self.expect_and_advance(TokenKind::RightSquareBracket)?; // Eat ']' token.

        Ok(ArrayPattern {
            node: self.end_node()?,
            elements,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingRestProperty
    fn parse_binding_rest_property(&mut self) -> Result<ObjectPatternProperty, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?; // Eat '...' token.

        let argument = self.parse_binding_identifier()?;

        Ok(ObjectPatternProperty::Rest(RestElement {
            node: self.end_node()?,
            argument: Pattern::Identifier(argument),
        }))
    }

    // https://tc39.es/ecma262/#prod-BindingPropertyList
    fn parse_binding_property_list(&mut self) -> Result<Vec<ObjectPatternProperty>, ParserError> {
        let mut properties = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightCurlyBrace {
            if self.cursor.current_token_kind() == TokenKind::Ellipsis {
                break;
            }

            properties.push(self.parse_binding_property()?);

            if self.cursor.current_token_kind() == TokenKind::Comma {
                self.cursor.advance(); // Eat ',' token.
            }
        }

        Ok(properties)
    }

    // https://tc39.es/ecma262/#prod-BindingElementList
    fn parse_binding_element_list(
        &mut self,
    ) -> Result<Vec<Option<ArrayPatternElement>>, ParserError> {
        let mut elements = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightSquareBracket {
            if self.cursor.current_token_kind() == TokenKind::Ellipsis {
                break;
            }

            if self.cursor.current_token_kind() == TokenKind::Comma {
                self.cursor.advance(); // Eat ellision token.

                elements.push(None);

                continue;
            }

            elements.push(Some(self.parse_binding_element()?));

            if self.cursor.current_token_kind() == TokenKind::Comma {
                self.cursor.advance(); // Eat ',' token.
            }
        }

        Ok(elements)
    }

    // https://tc39.es/ecma262/#prod-BindingProperty
    fn parse_binding_property(&mut self) -> Result<ObjectPatternProperty, ParserError> {
        let binding_property = match self.cursor.current_token_kind() {
            TokenKind::Identifier => {
                match self.cursor.peek_token_kind() {
                    TokenKind::Colon => {
                        self.start_node();

                        let property_name = self.parse_property_name()?;

                        self.expect_and_advance(TokenKind::Colon)?; // Eat ':' token.

                        let binding_element = self.parse_binding_element()?;

                        ObjectPatternProperty::Property(Property {
                            node: self.end_node()?,
                            method: false,
                            shorthand: false,
                            computed: false,
                            key: property_name,
                            kind: PropertyKind::Init,
                            value: PropertyValue::Pattern(binding_element.to_pattern()),
                        })
                    }
                    TokenKind::Assignment => {
                        self.start_node(); // Start node for property.

                        self.start_node(); // Start node for assignment pattern.

                        let binding_identifier = self.parse_binding_identifier()?;

                        self.cursor.advance(); // Eat '=' token.

                        let assignment_expression = self.parse_assignment_expression()?;

                        let assignment_pattern = AssignmentPattern {
                            node: self.end_node()?,
                            left: Pattern::Identifier(binding_identifier.clone()),
                            right: assignment_expression,
                        };

                        ObjectPatternProperty::Property(Property {
                            node: self.end_node()?,
                            method: false,
                            shorthand: true,
                            computed: false,
                            key: Expression::Identifier(binding_identifier.clone()),
                            kind: PropertyKind::Init,
                            value: PropertyValue::Pattern(Pattern::Assignment(Box::new(
                                assignment_pattern,
                            ))),
                        })
                    }
                    _ => {
                        self.start_node();

                        let binding_identifier = self.parse_binding_identifier()?;

                        ObjectPatternProperty::Property(Property {
                            node: self.end_node()?,
                            method: false,
                            shorthand: true,
                            computed: false,
                            key: Expression::Identifier(binding_identifier.clone()),
                            kind: PropertyKind::Init,
                            value: PropertyValue::Pattern(Pattern::Identifier(binding_identifier)),
                        })
                    }
                }
            }
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(binding_property)
    }

    // https://tc39.es/ecma262/#prod-BindingElement
    pub(crate) fn parse_binding_element(&mut self) -> Result<ArrayPatternElement, ParserError> {
        let binding_element = match self.cursor.current_token_kind() {
            TokenKind::Identifier => {
                self.start_node();

                let binding_identifier = self.parse_binding_identifier()?;

                if self.cursor.current_token_kind() == TokenKind::Assignment {
                    self.cursor.advance(); // Eat '=' token.

                    let assignment_expression = self.parse_assignment_expression()?;

                    ArrayPatternElement::Assignment(AssignmentPattern {
                        node: self.end_node()?,
                        left: Pattern::Identifier(binding_identifier),
                        right: assignment_expression,
                    })
                } else {
                    self.end_node()?; // End node for binding_identifier.

                    ArrayPatternElement::Identifier(binding_identifier)
                }
            }
            TokenKind::LeftSquareBracket => {
                let array_binding_pattern = self.parse_array_binding_pattern()?;

                ArrayPatternElement::Array(array_binding_pattern)
            }
            TokenKind::LeftCurlyBrace => {
                let object_binding_pattern = self.parse_object_binding_pattern()?;

                ArrayPatternElement::Object(object_binding_pattern)
            }
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(binding_element)
    }

    // https://tc39.es/ecma262/#prod-BindingElement
    pub(crate) fn parse_binding_rest_element(
        &mut self,
    ) -> Result<ArrayPatternElement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?;

        let binding_identifier_or_pattern = match self.cursor.current_token_kind() {
            TokenKind::Identifier => Pattern::Identifier(self.parse_binding_identifier()?),
            TokenKind::LeftSquareBracket => Pattern::Array(self.parse_array_binding_pattern()?),
            TokenKind::LeftCurlyBrace => Pattern::Object(self.parse_object_binding_pattern()?),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(ArrayPatternElement::Rest(RestElement {
            node: self.end_node()?,
            argument: binding_identifier_or_pattern,
        }))
    }
}
