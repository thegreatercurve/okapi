async function downloadDerivedCoreProperties(
  version: string | null | undefined = "UNIDATA"
): Promise<string> {
  const BASEURL = "https://unicode.org/Public";
  const FILENAME = "DerivedCoreProperties.txt";

  const url =
    version === "UNIDATA"
      ? `${BASEURL}/${version}/${FILENAME}`
      : `${BASEURL}/${version}/ucd/${FILENAME}`;

  try {
    console.info(`Downloading \`${url}\`...`);

    const response = await fetch(url);

    const text = await response.text();

    console.info(`Finished downloading \`DerivedCoreProperties.txt\``);

    return text;
  } catch (e) {
    throw e;
  }
}

function getDerivedCorePropertiesFileCodePoints(
  fileContents: string,
  derivedProperty: string
): Set<number> {
  let codePoints = new Set<number>();

  const rows = fileContents.split("\n");

  for (let row of rows) {
    // Continue if the line is empty or is a comment.
    if (row === "" || row.startsWith("#")) {
      continue;
    }

    let [codePointRange, property] = row.split("#")[0].split(";");

    codePointRange = codePointRange.trim();
    property = property.trim();

    if (property !== derivedProperty) {
      continue;
    }

    if (!codePointRange.includes("..")) {
      codePoints.add(parseInt(codePointRange, 16));
    } else {
      const [start, end] = codePointRange.split("..");

      for (let char = parseInt(start, 16); char <= parseInt(end, 16); char++) {
        codePoints.add(char);
      }
    }
  }

  return codePoints;
}

function processDerivedCorePropertiesFile(fileContents: string) {
  let iDStartCodePoints = getDerivedCorePropertiesFileCodePoints(
    fileContents,
    "ID_Start"
  );
  let iDContinueCodePoints = getDerivedCorePropertiesFileCodePoints(
    fileContents,
    "ID_Continue"
  );

  for (const codePoint of iDStartCodePoints) {
    console.log(String.fromCodePoint(205735), codePoint);
  }
}

function parseNamedArgs(args: string[]) {
  const namedArgs: Map<string, string> = new Map();

  for (const arg of args.slice(1)) {
    if (arg.startsWith("--") || arg.startsWith("-")) {
      console.log(arg);
      const [key, value] = arg.split("=");

      switch (key) {
        case "--version":
        case "-v":
          namedArgs.set("version", value);
      }
    }
  }

  return namedArgs;
}

async function main() {
  const [, ...args] = process.argv;

  const namedArgs = parseNamedArgs(args);

  const version = namedArgs.get("version");

  const derivedCoreProperties = await downloadDerivedCoreProperties(version);

  processDerivedCorePropertiesFile(derivedCoreProperties);
}

main();
