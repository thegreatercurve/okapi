const fs = require("fs");
const readline = require("readline");
const path = require("path");
const acorn = require("acorn");

const FULL_PATH =
  "/Users/johnflockton/Development/hippo/crates/hippo_js_parser/tests/parser/fixtures";

function fileExists(filePath) {
  try {
    fs.accessSync(filePath, fs.constants.F_OK);
    return true;
  } catch (err) {
    return false;
  }
}

function main() {
  fs.readdir(FULL_PATH, (err, files) => {
    if (err) {
      console.error(`Error reading directory: ${dirPath}`);
      console.error(err);
    } else {
      files.forEach((file) => {
        let filePath = path.join(FULL_PATH, file);

        fs.readFile(filePath, "utf8", (err, data) => {
          if (err) {
            console.error(`Error reading file: ${filePath}`);
            console.error(err);
          } else {
            const lines = data.split("\n").filter((line) => line !== "");

            filePath = filePath.slice(0, -3) + ".rs";

            const processLines = () => {
              let output = "";

              for (const line of lines) {
                let parsedJSON;

                try {
                  parsedJSON = JSON.stringify(
                    acorn.parse(line.trim(), { ecmaVersion: 2010 })
                  );
                } catch (e) {}

                output += `
                    assert_parse_module_eq!(
                        r#"${line.trim()}"#,
                        r#"${parsedJSON}"#
                    );`;
              }

              return output;
            };

            const content = `
use crate::parser::common::assert_parse_module_eq;

#[test]
fn ${file.slice(0, -3)}() {
    ${processLines()}
}
`;

            console.log(content);

            fs.writeFile(filePath, content, "utf8", (err) => {
              if (err) {
                console.error(`Error writing to file: ${filePath}`);
                console.error(err);
              } else {
                console.log(`Content has been written to: ${filePath}`);
              }
            });
          }
        });
      });
    }
  });
}

main();
