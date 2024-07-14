#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use okapi_js_parser::Parser;

#[napi(object)]
pub struct ParseResult {
    pub program: Option<String>,
    pub error: Option<String>,
}

#[napi]
pub fn parse(source: String) -> ParseResult {
    let mut parser = Parser::new(&source);

    match parser.parse_module_json() {
        Ok(program) => ParseResult {
            program: Some(program),
            error: None,
        },
        Err(err) => ParseResult {
            program: None,
            error: Some(err.to_string()),
        },
    }
}
