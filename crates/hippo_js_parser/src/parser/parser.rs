use crate::{tokens::Token, KeywordKind, Lexer, TokenKind};
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

    fn current_token_type(&self) -> TokenKind {
        self.current_token.kind.clone()
    }

    fn bump(&mut self) {
        self.current_token = self.lexer.next_token();

        self.next_token = self.lexer.next_token();
    }

    fn start_node(&mut self) -> Node {
        let token = &self.current_token;

        Node::new(token.start, 0)
    }

    fn finish_node(&mut self, node: &Node) -> Node {
        Node::new(node.loc.start, self.current_token.end)
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token_type() {
            TokenKind::Keyword(keyword) => match keyword {
                KeywordKind::Debugger => Some(self.parse_debugger_statement()),
                KeywordKind::Let | KeywordKind::Const => Some(self.parse_lexical_declaration()),
                _ => None,
            },
            _ => None,
        }
    }

    // https://tc39.es/ecma262/#sec-let-and-const-declarations
    fn parse_lexical_declaration(&mut self) -> Statement {
        println!("current token type: {:?}", self.current_token_type());

        let start_node = self.start_node();

        self.bump();

        println!("current token type: {:?}", self.current_token_type());

        Statement::Declaration(Declaration::Variable(VariableDeclaration {
            node: self.finish_node(&start_node),
            declarations: self.parse_binding_list(),
            kind: VariableKind::Let,
        }))
    }

    fn parse_binding_list(&mut self) -> Vec<VariableDeclarator> {
        let mut declarations = Vec::new();

        while self.current_token_type() != TokenKind::EOF {
            match self.current_token.kind {
                TokenKind::Identifier => {
                    let start_node = self.start_node();

                    self.bump();

                    let declaration = self.finish_node(&start_node);

                    if let Some(identifier) = &self.current_token.value {
                        declarations.push(VariableDeclarator {
                            node: declaration,
                            id: Identifier {
                                node: declaration,
                                name: identifier.clone(),
                            },
                            init: None,
                        });
                    }
                }
                _ => break,
            }
        }

        declarations
    }

    fn parse_debugger_statement(&mut self) -> Statement {
        let node = self.start_node();

        self.bump();

        Statement::Debugger(DebuggerStatement {
            node: self.finish_node(&node),
        })
    }
}
