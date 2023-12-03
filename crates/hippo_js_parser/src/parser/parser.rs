use crate::{Lexer, TokenType};
use hippo_estree::Program;

struct Context {
    strict_mode: bool,
}

pub struct Parser<'a> {
    context: Context,
    current_token: TokenType,
    next_token: TokenType,
    input: &'a str,
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);

        let current_token = lexer.next_token();
        let next_token = lexer.next_token();

        Self {
            context: Context { strict_mode: false },
            current_token,
            next_token,
            input,
            lexer,
        }
    }

    pub fn parse(&mut self) -> Program {
        Program { body: vec![] }
    }

    fn bump(&mut self) {
        self.current_token = self.lexer.next_token();

        self.next_token = self.lexer.next_token();
    }

    fn parse_expression(&mut self) {
        match self.current_token {
            TokenType::NumberLiteral => {}
            _ => {}
        }
    }
}
