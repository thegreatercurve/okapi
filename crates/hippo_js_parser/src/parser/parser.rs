use crate::{tokens::Token, KeywordKind, Lexer, ParserError, TokenKind};
use hippo_estree::{
    DebuggerStatement, Declaration, Identifier, Node, Program, ProgramBody, ProgramSourceTypes,
    Statement, VariableDeclaration, VariableDeclarator, VariableKind,
};

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
    current_token: Token,
    next_token: Token,
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
            current_token,
            next_token,
            source_str: input,
            lexer,
        }
    }

    pub fn parse(&mut self) -> Program {
        let program_body = self.parse_statement().unwrap();

        Program {
            body: vec![ProgramBody::Statement(program_body)],
            source_type: ProgramSourceTypes::Module,
            node: Node::new(0, self.lexer.len()),
        }
    }

    fn current_token_kind(&self) -> TokenKind {
        self.current_token.kind.clone()
    }

    fn current_token_value(&self) -> String {
        self.current_token.value.clone().unwrap_or_default()
    }

    fn unexpected_current_token_kind(&self) -> ParserError {
        ParserError::UnexpectedToken(self.current_token_kind())
    }

    fn advance(&mut self) {
        self.current_token = self.next_token.clone();
        self.next_token = self.lexer.next_token();
    }

    fn start_node(&mut self) -> Node {
        let token = &self.current_token;

        Node::new(token.start, 0)
    }

    fn finish_node(&mut self, node: &Node) -> Node {
        Node::new(node.loc.start, self.current_token.end)
    }

    // Statements

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.current_token_kind() {
            TokenKind::Keyword(keyword) => match keyword {
                KeywordKind::Debugger => self.parse_debugger_statement(),
                KeywordKind::Let | KeywordKind::Const | KeywordKind::Var => {
                    self.parse_lexical_declaration()
                }
                _ => return Err(ParserError::UnexpectedToken(self.current_token_kind())),
            },
            _ => return Err(ParserError::UnexpectedToken(self.current_token_kind())),
        }
    }

    // https://tc39.es/ecma262/#sec-let-and-const-declarations
    fn parse_lexical_declaration(&mut self) -> Result<Statement, ParserError> {
        let start_node = self.start_node();

        let kind = match self.current_token_kind() {
            TokenKind::Keyword(KeywordKind::Let) => VariableKind::Let,
            TokenKind::Keyword(KeywordKind::Const) => VariableKind::Const,
            TokenKind::Keyword(KeywordKind::Var) => VariableKind::Var,
            _ => return Err(ParserError::UnexpectedToken(self.current_token_kind())),
        };

        self.advance(); // Eat `let` or `const` token.

        let declarations = self.parse_binding_list()?;

        if kind == VariableKind::Const {
            todo!("Check const declarations have a valid identifier");
        }

        Ok(Statement::Declaration(Declaration::Variable(
            VariableDeclaration {
                node: self.finish_node(&start_node),
                declarations,
                kind,
            },
        )))
    }

    // https://tc39.es/ecma262/#prod-BindingList
    fn parse_binding_list(&mut self) -> Result<Vec<VariableDeclarator>, ParserError> {
        let mut declarations = vec![self.parse_lexical_binding()?];

        self.advance(); // Eat first identifier token.

        while self.current_token_kind() == TokenKind::Comma {
            self.advance(); // Eat `,` token.

            declarations.push(self.parse_lexical_binding()?);

            self.advance(); // Eat identifier token.
        }

        self.eat_semicolon();

        Ok(declarations)
    }

    // https://tc39.es/ecma262/#prod-LexicalBinding
    fn parse_lexical_binding(&mut self) -> Result<VariableDeclarator, ParserError> {
        let current_token_kind = self.current_token_kind();

        let identifier = match current_token_kind {
            TokenKind::Identifier => self.parse_binding_identifier(),
            _ => Err(self.unexpected_current_token_kind()),
        }?;

        Ok(VariableDeclarator {
            node: self.finish_node(&identifier.node),
            id: identifier,
            init: None,
        })
    }

    // https://tc39.es/ecma262/#prod-BindingIdentifier
    fn parse_binding_identifier(&mut self) -> Result<Identifier, ParserError> {
        let node = self.start_node();

        let name = match self.current_token_kind() {
            TokenKind::Identifier => self.current_token_value(),
            TokenKind::Keyword(KeywordKind::Await) => todo!(),
            TokenKind::Keyword(KeywordKind::Yield) => todo!(),
            _ => return Err(self.unexpected_current_token_kind()),
        };

        Ok(Identifier {
            node: self.finish_node(&node),
            name: name,
        })
    }

    // 14.16 The `debugger` Statement
    // https://tc39.github.io/ecma262/#sec-debugger-statement
    fn parse_debugger_statement(&mut self) -> Result<Statement, ParserError> {
        let node = self.start_node();

        self.advance();

        Ok(Statement::Debugger(DebuggerStatement {
            node: self.finish_node(&node),
        }))
    }

    fn eat_semicolon(&mut self) -> bool {
        if self.current_token_kind() == TokenKind::Semicolon {
            self.advance();

            return true;
        }

        false
    }
}
