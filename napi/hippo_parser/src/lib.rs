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

  let mut loop_count = 0;

  while loop_count < 10 {
    print!("loop_count: {}", loop_count);

    let token = scanner.next_token();

    let token_value = Token {
      token_type: token.kind.to_string(),
      value: token.value,
      start: token.start as u32,
      end: token.end as u32,
    };

    tokens.push(token_value);

    loop_count += 1;
  }

  tokens
}
