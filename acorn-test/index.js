const acorn = require("acorn");

let tokens = [...acorn.tokenizer("const x = 123", { ecmaVersion: 2010 })];

console.log(tokens);
