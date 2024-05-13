use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, fs};

use hippo_js_parser::Parser;
use pretty_assertions::assert_eq;
use serde_json::Value;
use std::collections::BTreeMap;

fn convert_ast_to_sorted_json(ast_str: &str) -> String {
    let value: Value = serde_json::from_str(&ast_str).unwrap();
    let bt_tree_map = value.as_object().unwrap().clone();
    let sorted = bt_tree_map.into_iter().collect::<BTreeMap<_, _>>();
    serde_json::to_string(&sorted).unwrap()
}

#[test]
fn large_file() {
    let pwd = env::current_dir().unwrap();

    let mut react_path = PathBuf::from(&pwd);
    react_path.push("./tests/parser/ok/fixtures/react-dom@18.2.0.development.js");

    let contents = fs::read_to_string(react_path).unwrap();
    let mut parser = Parser::new(&contents);

    parser.parse_module_json().unwrap();
}
