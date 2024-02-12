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

    // https://tc39.es/ecma262/#prod-BindingRestProperty
    fn parse_binding_rest_property(&mut self) -> Result<RestElement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?; // Eat '...' token.

        let argument = self.parse_binding_identifier()?;

        Ok(RestElement {
            node: self.end_node()?,
            argument: Pattern::Identifier(argument),
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

        let properties = vec![];

        let mut current_token_kind = self.cursor.current_token_kind();

        while current_token_kind != TokenKind::RightCurlyBrace {
            if current_token_kind == TokenKind::Ellipsis {
                self.parse_binding_rest_property()?;
            } else {
                self.parse_binding_property_list()?;
            }

            current_token_kind = self.cursor.current_token_kind();
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(ObjectPattern {
            node: self.end_node()?,
            properties,
        })
    }

    // https://tc39.es/ecma262/#prod-ArrayBindingPattern
    fn parse_array_binding_pattern(&mut self) -> Result<ArrayPattern, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftSquareBracket)?; // Eat '[' token.

        let elements = vec![];

        Ok(ArrayPattern {
            node: self.end_node()?,
            elements,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingPropertyList
    fn parse_binding_property_list(&mut self) -> Result<BindingPattern, ParserError> {
        self.parse_binding_property()?;

        todo!("parse_binding_property_list")
    }

    // https://tc39.es/ecma262/#prod-BindingProperty
    fn parse_binding_property(&mut self) -> Result<BindingPattern, ParserError> {
        todo!("parse_binding_property")
    }

    // https://tc39.es/ecma262/#prod-BindingElement
    pub(crate) fn parse_binding_element(&mut self) -> Result<Pattern, ParserError> {
        // TODO This should handle a lot more options.

        Ok(Pattern::Identifier(self.parse_binding_identifier()?))
    }

    // https://tc39.es/ecma262/#prod-BindingElement
    pub(crate) fn parse_binding_rest_element(&mut self) -> Result<Pattern, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?;

        if self
            .cursor
            .current_token_kind()
            .is_binding_identifier_keyword()
        {
            let binding_identifier = self.parse_binding_identifier()?;

            return Ok(Pattern::Rest(Box::new(RestElement {
                node: self.end_node()?,
                argument: Pattern::Identifier(binding_identifier),
            })));
        }

        let binding_pattern = self.parse_binding_pattern()?;

        match binding_pattern {
            BindingPattern::ArrayPattern(array_pattern) => Ok(Pattern::Array(array_pattern)),
            BindingPattern::ObjectPattern(object_pattern) => Ok(Pattern::Object(object_pattern)),
        }
    }
}
