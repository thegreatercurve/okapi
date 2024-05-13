import hippo from "hippo";
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
    path.join(process.cwd(), "tests/parser/fixtures", fileName),
    "utf8"
  );

  const json = hippo.parse(buffer);

  let result;

  if (json.program) {
    result = JSON.parse(json.program);
  } else if (json.error) {
    throw new Error(json.error);
  } else {
    throw new Error("Invalid Hippo output");
  }

  writeFile(
    path.join(process.cwd(), "tests/parser/fixtures/hippo", fileName),
    JSON.stringify(result, null, 2),
    ".json"
  );
}

// writeFixture("angular@1.8.3.js");
// writeFixture("react-dom@18.2.0.development.js");
// writeFixture("react@18.2.0.development.js");
writeFixture("three@0.163.0.js");
