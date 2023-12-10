import test from "ava";

import { tokenizeSync } from "../index.js";

test("Can tokenize basic literals", (t) => {
  t.deepEqual(tokenizeSync("'123123'"), [
    {
      end: 8,
      start: 0,
      tokenType: "StringLiteral",
      value: "123123",
    },
  ]);
});
