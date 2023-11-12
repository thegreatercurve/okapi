const acorn = require("acorn");

let tokens = [...acorn.tokenizer("123_123.12123;", { ecmaVersion: 2010 })];

console.log(tokens);
