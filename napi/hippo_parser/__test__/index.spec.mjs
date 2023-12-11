import test from "ava";

import { tokenizeSync } from "../index.js";

test("Can tokenize basic literals", (t) => {
  console.log(tokenizeSync('"hello \\x4A\\x61vaScript"'));
  console.log(tokenizeSync('"hello world \\u{1F607}"'));
  console.log(tokenizeSync('"hello\\u0020world"'));
  console.log(tokenizeSync("'hello\\u0020world\\u{D83D}\\u{DE04}\\u{1F607}'"));
  console.log(tokenizeSync("'hello \\x4A\\x61vaScript'"));
  console.log(tokenizeSync("'abcdefghijklmnopqrstuvwxyz🙂1234567891010🎉'"));
});
