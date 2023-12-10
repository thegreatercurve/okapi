#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use hippo_js_parser::{Config, Lexer};

#[napi(object)]
pub struct Token {
  pub token_type: String,
  pub value: Option<String>,
  pub start: u32,
  pub end: u32,
}

#[napi]
pub fn tokenize_sync(source_text: String) -> Vec<Token> {
  let mut tokens = vec![];

  let mut scanner = Lexer::new(&source_text, Config::default());

  while !scanner.is_end_of_file() {
    let token = scanner.next_token();

    let token_value = Token {
      token_type: token.kind.to_string(),
      value: token.value,
      start: token.start as u32,
      end: token.end as u32,
    };

    tokens.push(token_value);
  }

  tokens
}
