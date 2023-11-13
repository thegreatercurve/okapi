const esprima = require("esprima");
const program = "const answer = 42";

console.log(esprima.tokenize(program));

// esprima.parseScript(program);
