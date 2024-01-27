#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use hippo_js_parser::{Config, Lexer, TokenValue};

#[napi(object)]
pub struct Token {
  pub token_type: String,
  pub value: String,
  pub start: u32,
  pub end: u32,
}

#[napi]
pub fn tokenize_sync(source_text: String) -> Vec<Token> {
  let mut tokens = vec![];

  let mut lexer = Lexer::new(&source_text, Config::default());

  let mut loop_count = 0;

  while loop_count < 10 {
    let token = lexer.next_token();

    let token_value = Token {
      token_type: token.kind.to_string(),
      value: match token.value {
        TokenValue::String(value) => value,
        _ => "".to_string(),
      },
      start: token.start as u32,
      end: token.end as u32,
    };

    tokens.push(token_value);

    loop_count += 1;
  }

  tokens
}
