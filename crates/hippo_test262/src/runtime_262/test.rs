use std::{fs::File, io::BufReader, path::PathBuf};

use hippo_js_parser::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{errors::ErrorKind, test::Test};

pub(crate) struct RunTimeTest {
    path: PathBuf,
    meta_data: MetaData,
    source: String,
}

impl Test for RunTimeTest {
    fn new(path: PathBuf) -> Result<Option<Self>, ErrorKind> {
        let source = read_file(&path)?;
        let meta_data = parse_metadata(&source)?;

        Ok(Some(Test {
            source,
            meta_data,
            path,
        }))
    }

    fn description(&self) -> &String {
        &self.meta_data.description
    }

    fn ignore(&self) -> bool {
        self.meta_data
            .negative
            .clone()
            .is_some_and(|negative| negative.phase != Phase::Parse)
    }

    fn should_fail(&self) -> bool {
        self.meta_data
            .negative
            .clone()
            .is_some_and(|negative| negative.phase == Phase::Parse)
    }

    fn run(&self) -> Result<bool, ErrorKind> {
        let meta_data = &self.meta_data;

        if meta_data.flags.contains(&Flag::OnlyStrict) {
            self.parse_script(true)
        } else if meta_data.flags.contains(&Flag::NoStrict) {
            self.parse_script(false)
        } else if meta_data.flags.contains(&Flag::Module) {
            self.parse_module(true)
        } else {
            let strict = self.parse_script(true)?;
            let non_strict = self.parse_script(false)?;

            return Ok(strict && non_strict);
        }
    }

    fn parse_module(&self) -> Result<bool, ErrorKind> {
        let source = &self.source;

        let mut parser = Parser::new(&source);

        match parser.parse_module() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn parse_script(&self) -> Result<bool, ErrorKind> {
        let source = &self.source;

        let mut parser = Parser::new(&source);

        match parser.parse_script() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

fn parse_metadata(source: &str) -> Result<MetaData, ErrorKind> {
    let regex = Regex::new(r"/\*---\s*([\s\S]*?)\s*---\*/").map_err(ErrorKind::RegEx)?;

    let first_match = regex
        .captures(&source)
        .ok_or_else(|| ErrorKind::InvalidMetaData)?
        .get(1)
        .ok_or_else(|| ErrorKind::InvalidMetaData)?
        .as_str();

    let yaml = serde_yaml::from_str(first_match).map_err(ErrorKind::Yaml)?;

    Ok(yaml)
}

#[derive(Deserialize)]
struct MetaData {
    // Possible test ID.
    id: Option<String>,
    // Possible test ID.
    esid: Option<String>,
    // Possible test ID.
    es5id: Option<String>,
    // Possible test ID.
    es6id: Option<String>,
    // Long description of test contents.
    #[serde(default)]
    info: String,
    // Short description of test contents.
    #[serde(default)]
    description: String,
    // These tests are expected to generate an uncaught exception.
    #[serde(default)]
    negative: Option<Negative>,
    // One or more files whose content must be evaluated in the test realm's global scope prior to test execution. These files are located within the harness/ directory of the Test262 project.
    #[serde(default)]
    includes: Vec<String>,
    // The flags attribute is an optional value that specifies one or more of the flags (`onlyStrict``, `noStrict`, and `module` only relevant for parser tests).
    #[serde(default)]
    flags: Vec<Flag>,
    // The locale attribute allows tests to declare explicit information regarding locale specificity. Its value is an array of one or more valid language tags or subtags.
    #[serde(default)]
    locale: Vec<String>,
    // Required language features for the test.
    #[serde(default)]
    features: Vec<String>,
}

// These tests are expected to generate an uncaught exception.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Negative {
    phase: Phase,
    #[serde(rename = "type")]
    kind: Option<String>,
}

// The stage of the test interpretation process that the error is expected to be produced.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
enum Phase {
    Parse,
    Early,
    Resolution,
    Runtime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
enum Flag {
    OnlyStrict,
    NoStrict,
    Module,
    Raw,
    Async,
    Generated,
    #[serde(rename = "CanBlockIsFalse")]
    CanBlockIsFalse,
    #[serde(rename = "CanBlockIsTrue")]
    CanBlockIsTrue,
    #[serde(rename = "non-deterministic")]
    NonDeterministic,
}
