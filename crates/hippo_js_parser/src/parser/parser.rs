use crate::{tokens::Token, KeywordKind, Lexer, ParserError, TokenKind};
use hippo_estree::*;

fn is_lexical_declaration(token: &TokenKind) -> bool {
    match token {
        TokenKind::Keyword(KeywordKind::Const) | TokenKind::Keyword(KeywordKind::Let) => true,
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub strict_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { strict_mode: true }
    }
}

pub struct Parser<'a> {
    config: Config,
    pub(crate) current_token: Token,
    next_token: Token,
    pub(crate) previous_token: Token,
    source_str: &'a str,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let config = Config::default();

        let mut lexer = Lexer::new(input, config.clone());

        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Self {
            config,
            previous_token: current_token.clone(),
            next_token,
            current_token,
            source_str: input,
            lexer,
        }
    }

    pub fn parse_script(&mut self) -> Program {
        // TODO Parse parser statement of declaration.
        let program_body = self.parse_statement().unwrap();

        Program {
            body: vec![ProgramBody::Statement(program_body)],
            source_type: ProgramSourceTypes::Script,
            node: Node::new(0, self.lexer.len()),
        }
    }

    pub fn parse_script_json(&mut self) -> Result<String, serde_json::Error> {
        let program = self.parse_script();

        serde_json::to_string(&program)
    }

    pub fn parse_module(&mut self) -> Program {
        // TODO Parse parser statement of declaration.
        let program_body = self.parse_statement().unwrap();

        Program {
            body: vec![ProgramBody::Statement(program_body)],
            source_type: ProgramSourceTypes::Module,
            node: Node::new(0, self.lexer.len()),
        }
    }

    pub fn parse_module_json(&mut self) -> Result<String, serde_json::Error> {
        let program = self.parse_module();

        serde_json::to_string(&program)
    }

    pub(crate) fn current_token_kind(&self) -> TokenKind {
        self.current_token.kind.clone()
    }

    pub(crate) fn current_token_value(&self) -> String {
        self.current_token.value.clone().unwrap_or_default()
    }

    pub(crate) fn peek_token_kind(&self) -> TokenKind {
        self.next_token.kind.clone()
    }

    pub(crate) fn peek_token_value(&self) -> String {
        self.next_token.value.clone().unwrap_or_default()
    }

    pub(crate) fn unexpected_current_token_kind(&self) -> ParserError {
        ParserError::UnexpectedToken(self.current_token_kind())
    }

    pub(crate) fn advance(&mut self) {
        self.previous_token = self.current_token.clone();
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    pub(crate) fn expect_and_advance(&mut self, token_kind: TokenKind) -> Result<(), ParserError> {
        if self.current_token_kind() == token_kind {
            self.advance();

            return Ok(());
        }

        Err(self.unexpected_current_token_kind())
    }

    pub(crate) fn expect_one_of_and_advance(
        &mut self,
        token_kinds: Vec<TokenKind>,
    ) -> Result<(), ParserError> {
        for token_kind in token_kinds {
            if self.current_token_kind() == token_kind {
                self.advance();

                return Ok(());
            }
        }

        Err(self.unexpected_current_token_kind())
    }

    pub(crate) fn start_token(&mut self) -> Token {
        self.current_token.clone()
    }

    pub(crate) fn create_node(&self, start_token: &Token, end_token: &Token) -> Node {
        Node::new(start_token.start, end_token.end)
    }

    // https://tc39.es/ecma262/#sec-let-and-const-declarations
    pub(crate) fn parse_lexical_declaration(&mut self) -> Result<Statement, ParserError> {
        let start_token = self.start_token();

        let kind = match self.current_token_kind() {
            TokenKind::Keyword(KeywordKind::Let) => VariableKind::Let,
            TokenKind::Keyword(KeywordKind::Const) => VariableKind::Const,
            TokenKind::Keyword(KeywordKind::Var) => VariableKind::Var,
            _ => return Err(ParserError::UnexpectedToken(self.current_token_kind())),
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

        Ok(Statement::Declaration(Declaration::Variable(
            VariableDeclaration {
                node: self.create_node(&start_token, &self.current_token),
                declarations,
                kind,
            },
        )))
    }

    // https://tc39.es/ecma262/#prod-BindingList
    fn parse_binding_list(&mut self) -> Result<Vec<VariableDeclarator>, ParserError> {
        let mut declarations = vec![self.parse_binding_identifier_or_binding_pattern()?];

        self.expect_and_advance(TokenKind::Identifier)?; // Eat first identifier token.

        while self.current_token_kind() == TokenKind::Comma {
            self.expect_and_advance(TokenKind::Comma)?; // Eat `,` token.

            declarations.push(self.parse_binding_identifier_or_binding_pattern()?);

            self.expect_and_advance(TokenKind::Identifier)?; // Eat identifier token.
        }

        self.expect_and_advance(TokenKind::Semicolon)?;

        Ok(declarations)
    }

    // https://tc39.es/ecma262/#prod-LexicalBinding
    fn parse_binding_identifier_or_binding_pattern(
        &mut self,
    ) -> Result<VariableDeclarator, ParserError> {
        let current_token_kind = self.current_token_kind();

        let binding_identifier = match current_token_kind {
            TokenKind::Identifier => self.parse_binding_identifier(),
            TokenKind::LeftCurlyBrace => self.parse_object_binding_pattern(),
            TokenKind::LeftSquareBracket => self.parse_array_binding_pattern(),
            _ => Err(self.unexpected_current_token_kind()),
        }?;

        let node = match &binding_identifier {
            BindingKind::Identifier(identifier) => identifier.node,
            BindingKind::ObjectPattern(object_pattern) => object_pattern.node,
            BindingKind::ArrayPattern(array_pattern) => array_pattern.node,
        };

        Ok(VariableDeclarator {
            node,
            id: binding_identifier,
            init: None,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingIdentifier
    fn parse_binding_identifier(&mut self) -> Result<BindingKind, ParserError> {
        let start_token = self.start_token();

        let name = match self.current_token_kind() {
            TokenKind::Identifier => self.current_token_value(),
            TokenKind::Keyword(KeywordKind::Await) => todo!(),
            TokenKind::Keyword(KeywordKind::Yield) => todo!(),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(BindingKind::Identifier(Identifier {
            node: self.create_node(&start_token, &self.current_token),
            name: name,
        }))
    }

    // https://tc39.es/ecma262/#prod-ObjectBindingPattern
    fn parse_object_binding_pattern(&mut self) -> Result<BindingKind, ParserError> {
        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?; // Eat `{` token.

        let properties = vec![];

        let mut current_token_kind = self.current_token_kind();

        while current_token_kind != TokenKind::RightCurlyBrace {
            if current_token_kind == TokenKind::Ellipsis {
                self.parse_binding_rest_property()?;
            } else {
                self.parse_binding_property_list()?;
            }

            current_token_kind = self.current_token_kind();
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(BindingKind::ObjectPattern(ObjectPattern {
            node: self.create_node(&start_token, &self.current_token),
            properties,
        }))
    }

    // https://tc39.es/ecma262/#prod-ArrayBindingPattern
    fn parse_array_binding_pattern(&mut self) -> Result<BindingKind, ParserError> {
        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::LeftSquareBracket)?; // Eat `[` token.

        let elements = vec![];

        Ok(BindingKind::ArrayPattern(ArrayPattern {
            node: self.create_node(&start_token, &self.current_token),
            elements,
        }))
    }

    // https://tc39.es/ecma262/#prod-BindingRestProperty
    fn parse_binding_rest_property(&mut self) -> Result<RestElement, ParserError> {
        let start_token = self.start_token();

        self.expect_and_advance(TokenKind::Ellipsis)?; // Eat `...` token.

        let argument = self.parse_binding_identifier()?;

        Ok(RestElement {
            node: self.create_node(&start_token, &self.current_token),
            argument,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingPropertyList
    fn parse_binding_property_list(&mut self) -> Result<BindingKind, ParserError> {
        self.parse_binding_property()?;

        todo!()
    }

    // https://tc39.es/ecma262/#prod-BindingProperty
    fn parse_binding_property(&mut self) -> Result<BindingKind, ParserError> {
        todo!()
    }

    // https://tc39.es/ecma262/#prod-PropertyName
    fn parse_property_name(&mut self) -> Result<(), ParserError> {
        todo!()
    }

    // https://tc39.es/ecma262/#prod-Initializer
    fn parse_initializer(&mut self) -> Result<(), ParserError> {
        todo!()
    }
}
