import acorn from "acorn";
import path from "path";
import { promises as fs } from "fs";

async function writeFile(
  filePath: string,
  content: string,
  newExtension: string
) {
  try {
    let newFilePath = filePath.replace(/\.js$/, newExtension);
    await fs.mkdir(path.dirname(newFilePath), { recursive: true });
    await fs.writeFile(newFilePath, content);
  } catch (err) {
    console.error(`Error writing file to disk: ${err}`);
  }
}

async function writeFixture(fileName: string) {
  const buffer = await fs.readFile(
    path.join(process.cwd(), "tests/parser/acorn/fixtures", fileName),
    "utf8"
  );

  const json = acorn.parse(buffer, {
    ecmaVersion: "latest",
    sourceType: "module",
  });

  if (!json) {
    throw new Error("Invalid Acorn output");
  }

  writeFile(
    path.join(process.cwd(), "tests/parser/fixtures/acorn", fileName),
    JSON.stringify(json, null, 2),
    ".json"
  );
}

writeFixture("angular@1.8.3.js");
writeFixture("react-dom@18.2.0.development.js");
writeFixture("react@18.2.0.development.js");
writeFixture("three@0.163.0.js");
