use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14.3 Declarations and the Variable Statement
// https://tc39.es/ecma262/#sec-declarations-and-the-variable-statement
impl Parser {
    // 14.3.1 Let and Const Declarations
    // https://tc39.es/ecma262/#prod-LexicalDeclaration
    pub(crate) fn parse_lexical_declaration(
        &mut self,
        include_optional_semicolon: bool,
    ) -> Result<VariableDeclaration, ParserError> {
        let start_index = self.start_node();

        let kind = match self.token_kind() {
            TokenKind::Keyword(KeywordKind::Let) => VariableKind::Let,
            TokenKind::Keyword(KeywordKind::Const) => VariableKind::Const,
            _ => return Err(self.unexpected_current_token_kind()),
        };

        match kind {
            VariableKind::Let => self.expect_and_advance(TokenKind::Keyword(KeywordKind::Let))?,
            VariableKind::Const => {
                self.expect_and_advance(TokenKind::Keyword(KeywordKind::Const))?
            }
            VariableKind::Var => return Err(self.unexpected_current_token_kind()),
        };

        let binding_list = self.parse_binding_list()?;

        if kind == VariableKind::Const {
            // TODO Check const declarations have a valid initializer.
        }

        if include_optional_semicolon {
            self.expect_optional_semicolon_and_advance();
        }

        Ok(VariableDeclaration {
            node: self.end_node(start_index)?,
            declarations: binding_list,
            kind,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingList
    pub(crate) fn parse_binding_list(&mut self) -> Result<Vec<VariableDeclarator>, ParserError> {
        let mut declarations = vec![self.parse_binding_identifier_or_binding_pattern()?];

        while self.token_kind() == TokenKind::Comma {
            self.expect_and_advance(TokenKind::Comma)?; // Eat ',' token.

            declarations.push(self.parse_binding_identifier_or_binding_pattern()?);
        }

        Ok(declarations)
    }

    // 14.3.2 Variable Statement
    // https://tc39.es/ecma262/#prod-VariableStatement
    pub(crate) fn parse_variable_statement(
        &mut self,
        include_optional_semicolon: bool,
    ) -> Result<VariableDeclaration, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Var))?;

        // `VariableDeclaration` is the same grammar as `LexicalDeclaration`.
        let binding_list = self.parse_binding_list()?;

        if include_optional_semicolon {
            self.expect_optional_semicolon_and_advance();
        }

        Ok(VariableDeclaration {
            node: self.end_node(start_index)?,
            declarations: binding_list,
            kind: VariableKind::Var,
        })
    }

    // 14.3.3 Destructuring Binding Patterns
    // https://tc39.es/ecma262/#prod-BindingIdentifier
    // https://tc39.es/ecma262/#prod-BindingPattern
    fn parse_binding_identifier_or_binding_pattern(
        &mut self,
    ) -> Result<VariableDeclarator, ParserError> {
        let start_index = self.start_node();

        let binding_identifier = match self.token_kind() {
            token_kind if token_kind.is_binding_identifier() => {
                Pattern::Identifier(self.parse_binding_identifier()?)
            }
            TokenKind::LeftCurlyBrace => Pattern::Object(self.parse_object_binding_pattern()?),
            TokenKind::LeftSquareBracket => Pattern::Array(self.parse_array_binding_pattern()?),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        let initializer = if self.token_kind() == TokenKind::Assignment {
            self.advance_any(); // Eat '=' token.

            let assignment_expression = self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_assignment_expression,
            )?;

            Some(assignment_expression)
        } else {
            None
        };

        Ok(VariableDeclarator {
            node: self.end_node(start_index)?,
            id: binding_identifier,
            init: initializer,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingPattern
    pub(crate) fn parse_binding_pattern(&mut self) -> Result<BindingPattern, ParserError> {
        match self.token_kind() {
            TokenKind::LeftCurlyBrace => {
                Ok(BindingPattern::Object(self.parse_object_binding_pattern()?))
            }
            TokenKind::LeftSquareBracket => {
                Ok(BindingPattern::Array(self.parse_array_binding_pattern()?))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ObjectBindingPattern
    pub(crate) fn parse_object_binding_pattern(&mut self) -> Result<ObjectPattern, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?; // Eat '{' token.

        let mut properties = self.parse_binding_property_list()?;

        if self.token_kind() == TokenKind::Ellipsis {
            properties.push(self.parse_binding_rest_property()?);
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?; // Eat '}' token.

        Ok(ObjectPattern {
            node: self.end_node(start_index)?,
            properties,
        })
    }

    // https://tc39.es/ecma262/#prod-ArrayBindingPattern
    pub(crate) fn parse_array_binding_pattern(&mut self) -> Result<ArrayPattern, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftSquareBracket)?; // Eat '[' token.

        let mut elements = self.parse_binding_element_list()?;

        if self.token_kind() == TokenKind::Ellipsis {
            elements.push(Some(self.parse_binding_rest_element()?));
        }

        self.expect_and_advance(TokenKind::RightSquareBracket)?; // Eat ']' token.

        Ok(ArrayPattern {
            node: self.end_node(start_index)?,
            elements,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingRestProperty
    fn parse_binding_rest_property(&mut self) -> Result<ObjectPatternProperty, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?; // Eat '...' token.

        let identifier = self.parse_binding_identifier()?;

        Ok(ObjectPatternProperty::Rest(RestElement {
            node: self.end_node(start_index)?,
            argument: Box::new(Pattern::Identifier(identifier)),
        }))
    }

    // https://tc39.es/ecma262/#prod-BindingPropertyList
    fn parse_binding_property_list(&mut self) -> Result<Vec<ObjectPatternProperty>, ParserError> {
        let mut properties = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            if self.token_kind() == TokenKind::Ellipsis {
                break;
            }

            properties.push(self.parse_binding_property()?);

            if self.token_kind() == TokenKind::Comma {
                self.advance_any(); // Eat ',' token.
            } else {
                break;
            }
        }

        Ok(properties)
    }

    // https://tc39.es/ecma262/#prod-BindingElementList
    fn parse_binding_element_list(
        &mut self,
    ) -> Result<Vec<Option<ArrayPatternElement>>, ParserError> {
        let mut elements = vec![];

        while self.token_kind() != TokenKind::RightSquareBracket {
            if self.token_kind() == TokenKind::Ellipsis {
                break;
            }

            if self.token_kind() == TokenKind::Comma {
                self.advance_any(); // Eat ellision token.

                elements.push(None);

                continue;
            }

            elements.push(Some(self.parse_binding_element()?));

            if self.token_kind() == TokenKind::Comma {
                self.advance_any(); // Eat ',' token.
            } else {
                break;
            }
        }

        Ok(elements)
    }

    // https://tc39.es/ecma262/#prod-BindingProperty
    fn parse_binding_property(&mut self) -> Result<ObjectPatternProperty, ParserError> {
        let mut is_computed = false;

        let start_index = self.start_node(); // Start property node.

        let left_hand_expression = match self.token_kind() {
            TokenKind::LeftSquareBracket => {
                is_computed = true;

                self.parse_property_name()?
            }
            token_kind if token_kind.is_property_name() => self.parse_property_name()?,
            _ => Expression::Identifier(self.parse_binding_identifier()?),
        };

        match self.token_kind() {
            TokenKind::Colon => {
                self.expect_and_advance(TokenKind::Colon)?; // Eat ':' token.

                let assignment_element = self.parse_binding_element()?;

                Ok(ObjectPatternProperty::Property(Property {
                    node: self.end_node(start_index)?,
                    method: false,
                    shorthand: false,
                    computed: is_computed,
                    key: left_hand_expression,
                    kind: PropertyKind::Init,
                    value: PropertyValue::Pattern(Pattern::try_from(assignment_element)?),
                }))
            }
            TokenKind::Assignment => {
                self.advance_any(); // Eat '=' token.

                let assignment_expression = self.with_params(
                    self.params.clone().add_allow_in(false),
                    Self::parse_assignment_expression,
                )?;

                let assignment_pattern = AssignmentPattern {
                    node: self.end_node(start_index)?,
                    left: Box::new(Pattern::try_from(left_hand_expression.clone())?),
                    right: assignment_expression,
                };

                Ok(ObjectPatternProperty::Property(Property {
                    node: self.end_node(start_index)?,
                    method: false,
                    shorthand: true,
                    computed: is_computed,
                    key: left_hand_expression,
                    kind: PropertyKind::Init,
                    value: PropertyValue::Pattern(Pattern::Assignment(assignment_pattern)),
                }))
            }
            _ => Ok(ObjectPatternProperty::Property(Property {
                node: self.end_node(start_index)?,
                method: false,
                shorthand: true,
                computed: is_computed,
                key: left_hand_expression.clone(),
                kind: PropertyKind::Init,
                value: PropertyValue::Pattern(Pattern::try_from(left_hand_expression)?),
            })),
        }
    }

    // https://tc39.es/ecma262/#prod-BindingElement
    pub(crate) fn parse_binding_element(&mut self) -> Result<ArrayPatternElement, ParserError> {
        let start_index = self.start_node();

        let left_hand_idenfitier_or_binding_pattern = match self.token_kind() {
            token_kind if token_kind.is_binding_identifier() => {
                ArrayPatternElement::Identifier(self.parse_binding_identifier()?)
            }
            TokenKind::LeftSquareBracket => {
                ArrayPatternElement::Array(self.parse_array_binding_pattern()?)
            }
            TokenKind::LeftCurlyBrace => {
                ArrayPatternElement::Object(self.parse_object_binding_pattern()?)
            }
            _ => return Err(self.unexpected_current_token_kind()),
        };

        if self.token_kind() == TokenKind::Assignment {
            self.advance_any(); // Eat '=' token.
        } else {
            return Ok(left_hand_idenfitier_or_binding_pattern);
        }

        let assignment_expression = self.with_params(
            self.params.clone().add_allow_in(false),
            Self::parse_assignment_expression,
        )?;

        let left_hand_pattern = Pattern::try_from(left_hand_idenfitier_or_binding_pattern)?;

        Ok(ArrayPatternElement::Assignment(AssignmentPattern {
            node: self.end_node(start_index)?,
            left: Box::new(left_hand_pattern),
            right: assignment_expression,
        }))
    }

    // https://tc39.es/ecma262/#prod-BindingElement
    pub(crate) fn parse_binding_rest_element(
        &mut self,
    ) -> Result<ArrayPatternElement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?;

        let binding_identifier_or_pattern = match self.token_kind() {
            token_kind if token_kind.is_binding_identifier() => {
                Pattern::Identifier(self.parse_binding_identifier()?)
            }
            TokenKind::LeftSquareBracket => Pattern::Array(self.parse_array_binding_pattern()?),
            TokenKind::LeftCurlyBrace => Pattern::Object(self.parse_object_binding_pattern()?),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(ArrayPatternElement::RestElement(RestElement {
            node: self.end_node(start_index)?,
            argument: Box::new(binding_identifier_or_pattern),
        }))
    }
}
