const acorn = require("acorn");

let tokens = [...acorn.tokenizer("const foo = 1;", { ecmaVersion: 2010 })];

console.log(tokens);
