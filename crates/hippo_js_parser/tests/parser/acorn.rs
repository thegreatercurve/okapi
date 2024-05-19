use std::{io, path::PathBuf};

use crate::parser::sort_json_keys;
use hippo_js_parser::Parser;
use pretty_assertions::assert_eq;

use super::read_file;

pub(crate) fn read_fixture(path: &str) -> io::Result<String> {
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    full_path.push("tests/parser/fixtures");
    full_path.push(path);

    read_file(&full_path)
}

// #[test]
// fn acorn_equality_react() {
//     let fixture = read_fixture(&"react@18.2.0.development.js").unwrap();

//     let parsed = Parser::new(&fixture).parse_module_json().unwrap();
//     let parsed_json = serde_json::from_str(&parsed).unwrap();
//     let parsed_sorted = sort_json_keys(parsed_json).unwrap();

//     let acorn_parsed_fixture = read_fixture(&"acorn/react@18.2.0.development.json").unwrap();
//     let acorn_json = serde_json::from_str(&acorn_parsed_fixture).unwrap();
//     let acorn_sorted = sort_json_keys(acorn_json).unwrap();

//     assert_eq!(parsed_sorted, acorn_sorted);
// }

// #[test]
// fn acorn_equality_react_dom() {
//     let fixture = read_fixture(&"react-dom@18.2.0.development.js").unwrap();

//     let parsed = Parser::new(&fixture).parse_module_json().unwrap();
//     let parsed_json = serde_json::from_str(&parsed).unwrap();
//     let parsed_sorted = sort_json_keys(parsed_json).unwrap();

//     let acorn_parsed_fixture = read_fixture(&"acorn/react-dom@18.2.0.development.json").unwrap();
//     let acorn_json = serde_json::from_str(&acorn_parsed_fixture).unwrap();
//     let acorn_sorted = sort_json_keys(acorn_json).unwrap();

//     assert_eq!(parsed_sorted, acorn_sorted);
// }

// #[test]
// fn acorn_equality_angular() {
//     let fixture = read_fixture(&"angular@1.8.3.js").unwrap();

//     let parsed = Parser::new(&fixture).parse_module_json().unwrap();
//     let parsed_json = serde_json::from_str(&parsed).unwrap();
//     let parsed_sorted = sort_json_keys(parsed_json).unwrap();

//     let acorn_parsed_fixture = read_fixture(&"acorn/angular@1.8.3.json").unwrap();
//     let acorn_json = serde_json::from_str(&acorn_parsed_fixture).unwrap();
//     let acorn_sorted = sort_json_keys(acorn_json).unwrap();

//     assert_eq!(parsed_sorted, acorn_sorted);
// }

// #[test]
// fn acorn_equality_three() {
//     let fixture = read_fixture(&"three@0.163.0.js").unwrap();

//     let parsed = Parser::new(&fixture).parse_module_json().unwrap();
//     let parsed_json = serde_json::from_str(&parsed).unwrap();
//     let parsed_sorted = sort_json_keys(parsed_json).unwrap();

//     let acorn_parsed_fixture = read_fixture(&"acorn/three@0.163.0.json").unwrap();
//     let acorn_json = serde_json::from_str(&acorn_parsed_fixture).unwrap();
//     let acorn_sorted = sort_json_keys(acorn_json).unwrap();

//     assert_eq!(parsed_sorted, acorn_sorted);
// }