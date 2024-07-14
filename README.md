# Okapi JS Parser

> [!NOTE]
> This is a parser is still a work-in-progress and should not be used in production.

A lightweight but production-grade JavaScript parser for Rust and WebAssembly.

The library fully supports ECMAScript 2024 and it outputs it's Abstract Syntax Tree (AST) as either normal [ESTree](https://github.com/estree/estree) compliant structs or as JSON using `serde_json`.

ESTree is a community-maintained and widely-used standard for ASTs, and is the same output as most other JavaScript parsers.

Below are listed some of the aims and achievements of this project:

- Lightweight and performant.
- Largely dependency-free apart from `serde_json`.
- Compliant with the [ECMAScript 2024 spec](https://tc39.es/ecma262/2024/).
- 100% pass rate against [Test262 parser tests](https://github.com/tc39/test262-parser-tests).
- Output equality tested against [Acorn](https://github.com/acornjs/acorn/tree/master).

### Usage

```rs
use okapi_js_parser::Parser;

let source_str: &str = ...

let mut parser = Parser::new(source_str);

match parser.parse_script() {
    Ok(program) => ...,
    Err(err) => ...,
}
```

### Tests

To run the local parser and lexer tests:

```shell
cargo test
```

To run the tests for [Test262](https://github.com/tc39/test262-parser-tests):

```shell
cargo run -p okapi_test262
```

### Playground

The local web playground to test the parser functionality can be found in `./playground`.

Instructions on how to set-up the playground can be found in the associated [`README.md`](./playground/README.md).
