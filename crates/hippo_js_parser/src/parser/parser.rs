use crate::{Cursor, KeywordKind, Lexer, ParserError, TokenKind, TokenValue};
use hippo_estree::*;

#[derive(Clone, Debug)]
pub struct Config {
    pub strict_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { strict_mode: true }
    }
}

#[derive(Clone)]
pub struct Parser<'a> {
    config: Config,
    pub cursor: Cursor<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let config = Config::default();

        let mut lexer = Lexer::new(input, config.clone());

        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Self {
            config,
            cursor: Cursor::new(input, lexer, current_token, next_token),
        }
    }

    pub(crate) fn unexpected_current_token_kind(&self) -> ParserError {
        ParserError::UnexpectedToken(self.cursor.current_token_kind())
    }

    pub fn parse_script_json(&mut self) -> Result<String, serde_json::Error> {
        let program = self.parse_script();

        serde_json::to_string(&program.unwrap())
    }

    pub fn parse_module_json(&mut self) -> Result<String, serde_json::Error> {
        let program = self.parse_module();

        serde_json::to_string(&program.unwrap())
    }

    pub(crate) fn start_node(&mut self) {
        self.cursor
            .token_stack
            .push(self.cursor.current_token.clone())
    }

    pub fn end_node(&mut self) -> Result<Node, ParserError> {
        if self.cursor.token_stack.is_empty() {
            return Err(ParserError::UnexpectedEmptyNode);
        }

        let start = self.cursor.token_stack.pop().unwrap().start;
        let end: usize = self.cursor.previous_token.end;

        Ok(Node::new(start, end))
    }

    pub(crate) fn expect_and_advance(&mut self, token_kind: TokenKind) -> Result<(), ParserError> {
        if self.cursor.current_token_kind() == token_kind {
            self.cursor.advance();

            return Ok(());
        }

        Err(self.unexpected_current_token_kind())
    }

    pub(crate) fn expect_one_of_and_advance(
        &mut self,
        token_kinds: Vec<TokenKind>,
    ) -> Result<(), ParserError> {
        for token_kind in token_kinds {
            if self.cursor.current_token_kind() == token_kind {
                self.cursor.advance();

                return Ok(());
            }
        }

        Err(self.unexpected_current_token_kind())
    }

    pub(crate) fn expect_optional_semicolon_and_advance(&mut self) {
        if self.cursor.current_token_kind() == TokenKind::Semicolon {
            self.cursor.advance();
        }
    }

    // https://tc39.es/ecma262/#sec-let-and-const-declarations
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
            self.expect_and_advance(TokenKind::Comma)?; // Eat `,` token.

            declarations.push(self.parse_binding_identifier_or_binding_pattern()?);
        }

        Ok(declarations)
    }

    // https://tc39.es/ecma262/#prod-LexicalBinding
    fn parse_binding_identifier_or_binding_pattern(
        &mut self,
    ) -> Result<VariableDeclarator, ParserError> {
        self.start_node();

        let current_token_kind = self.cursor.current_token_kind();

        let binding_identifier = match current_token_kind {
            TokenKind::Identifier => {
                VariableDeclaratorBindingKind::Identifier(self.parse_binding_identifier()?)
            }
            TokenKind::LeftCurlyBrace => {
                VariableDeclaratorBindingKind::ObjectPattern(self.parse_object_binding_pattern()?)
            }
            TokenKind::LeftSquareBracket => {
                VariableDeclaratorBindingKind::ArrayPattern(self.parse_array_binding_pattern()?)
            }
            _ => return Err(self.unexpected_current_token_kind()),
        };

        let initializer = if self.cursor.current_token_kind() == TokenKind::Assignment {
            self.expect_and_advance(TokenKind::Assignment)?; // Eat `=` token.

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

    // https://tc39.es/ecma262/#prod-BindingIdentifier
    pub(crate) fn parse_binding_identifier(&mut self) -> Result<Identifier, ParserError> {
        self.start_node();

        let token_value = match self.cursor.current_token_kind() {
            TokenKind::Identifier => self.cursor.current_token_value(),
            TokenKind::Keyword(KeywordKind::Await) => todo!("parse_binding_identifier await"),
            TokenKind::Keyword(KeywordKind::Yield) => todo!("parse_binding_identifier yield"),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        let identifier_name = match token_value {
            TokenValue::String(name) => name,
            _ => return Err(ParserError::UnexpectedTokenValue),
        };

        self.expect_one_of_and_advance(vec![
            TokenKind::Identifier,
            TokenKind::Keyword(KeywordKind::Await),
            TokenKind::Keyword(KeywordKind::Yield),
        ])?; // Eat identifier token.

        Ok(Identifier {
            node: self.end_node()?,
            name: identifier_name,
        })
    }

    // https://tc39.es/ecma262/#prod-ObjectBindingPattern
    fn parse_object_binding_pattern(&mut self) -> Result<ObjectPattern, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?; // Eat `{` token.

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

        self.expect_and_advance(TokenKind::LeftSquareBracket)?; // Eat `[` token.

        let elements = vec![];

        Ok(ArrayPattern {
            node: self.end_node()?,
            elements,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingRestProperty
    fn parse_binding_rest_property(&mut self) -> Result<RestElement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Ellipsis)?; // Eat `...` token.

        //
        let argument = self.parse_binding_identifier()?;

        Ok(RestElement {
            node: self.end_node()?,
            argument: RestElementArgument::Identifier(argument),
        })
    }

    // https://tc39.es/ecma262/#prod-BindingPropertyList
    fn parse_binding_property_list(&mut self) -> Result<BindingPattern, ParserError> {
        self.parse_binding_property()?;

        todo!()
    }

    // https://tc39.es/ecma262/#prod-BindingProperty
    fn parse_binding_property(&mut self) -> Result<BindingPattern, ParserError> {
        todo!()
    }

    // https://tc39.es/ecma262/#prod-Initializer
    fn parse_initializer(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}
