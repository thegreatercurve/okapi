use std::{io, path::PathBuf};

use assert_json_diff::assert_json_include;
use okapi_js_parser::Parser;
use serde_json::Value;

mod file;

pub(crate) fn read_fixture(path: &str) -> io::Result<String> {
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    full_path.push("tests/parser/acorn/fixtures");
    full_path.push(path);

    file::read_file(&full_path)
}

#[test]
fn acorn_equality_react() {
    let fixture = read_fixture("react@18.2.0.development.js").unwrap();

    let parsed = Parser::new(&fixture).parse_module_json().unwrap();
    let parsed_json = serde_json::from_str::<Value>(&parsed).unwrap();

    let acorn_parsed_fixture = read_fixture("acorn/react@18.2.0.development.json").unwrap();
    let acorn_json = serde_json::from_str::<Value>(&acorn_parsed_fixture).unwrap();

    assert_json_include!(
        actual: parsed_json,
        expected: acorn_json
    );
}

#[test]
fn acorn_equality_react_dom() {
    let fixture = read_fixture("react-dom@18.2.0.development.js").unwrap();

    let parsed = Parser::new(&fixture).parse_module_json().unwrap();
    let parsed_json = serde_json::from_str::<Value>(&parsed).unwrap();

    let acorn_parsed_fixture = read_fixture("acorn/react-dom@18.2.0.development.json").unwrap();
    let acorn_json = serde_json::from_str::<Value>(&acorn_parsed_fixture).unwrap();

    assert_json_include!( actual: parsed_json, expected: acorn_json);
}

#[test]
fn acorn_equality_angular() {
    let fixture = read_fixture("angular@1.8.3.js").unwrap();

    let parsed = Parser::new(&fixture).parse_module_json().unwrap();
    let parsed_json = serde_json::from_str::<Value>(&parsed).unwrap();

    let acorn_parsed_fixture = read_fixture("acorn/angular@1.8.3.json").unwrap();
    let acorn_json = serde_json::from_str::<Value>(&acorn_parsed_fixture).unwrap();

    assert_json_include!( actual: parsed_json, expected: acorn_json);
}

#[test]
fn acorn_equality_three() {
    let fixture = read_fixture("three@0.163.0.js").unwrap();

    let parsed = Parser::new(&fixture).parse_module_json().unwrap();
    let parsed_json = serde_json::from_str::<Value>(&parsed).unwrap();

    let acorn_parsed_fixture = read_fixture("acorn/three@0.163.0.json").unwrap();
    let acorn_json = serde_json::from_str::<Value>(&acorn_parsed_fixture).unwrap();

    assert_json_include!( actual: parsed_json, expected: acorn_json);
}
