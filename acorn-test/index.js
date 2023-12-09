const acorn = require("acorn");

let tokens = [
  ...acorn.tokenizer("class Foo { #bar = 1; };", { ecmaVersion: 2010 }),
];

console.log(tokens);
