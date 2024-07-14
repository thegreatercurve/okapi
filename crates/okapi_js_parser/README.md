# Okapi Test262

This crate contains the code for both the lexer and parser.

These are both tested against a suite of local tests, taken predominantly from [the Biome parser](https://github.com/biomejs/biome/tree/main/crates/biome_js_parser), and tested against the output of the [Acorn](https://github.com/acornjs/acorn/tree/master) JS parser.

### Tests

To run the local parser and lexer tests:

```shell
cargo test
```

To update the additional Acorn equilty tests against some major libraries (e.g. React, ReactDOM, Angular, and Three.js):

```shell
npm install

npm run generate
```
