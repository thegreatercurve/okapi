const acorn = require("acorn");

let tokens = [
  ...acorn.tokenizer("`hello\u0020${1}world\u{D83D}\u{DE04}\u{1F607}`", {
    ecmaVersion: 2010,
  }),
];

console.log(tokens);
