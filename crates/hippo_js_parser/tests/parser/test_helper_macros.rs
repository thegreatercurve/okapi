macro_rules! assert_parser_script_eq {
    ($test_case: expr, $expected_result: expr) => {{
        use assert_json_diff::assert_json_include;
        use hippo_js_parser::Parser;
        use serde_json::Value;

        let parsed = Parser::new(&$test_case).parse_script_json().unwrap();
        let parsed_json = serde_json::from_str::<Value>(&parsed).unwrap();

        let expected_json = serde_json::from_str::<Value>(&$expected_result).unwrap();

        assert_json_include!(actual: parsed_json, expected: expected_json);
    }};
}

macro_rules! assert_parser_module_eq {
    ($test_case: expr, $expected_result: expr) => {{
        use assert_json_diff::assert_json_include;
        use hippo_js_parser::Parser;
        use serde_json::Value;

        let parsed = Parser::new(&$test_case).parse_module_json().unwrap();
        let parsed_json = serde_json::from_str::<Value>(&parsed).unwrap();

        let expected_json = serde_json::from_str::<Value>(&$expected_result).unwrap();

        assert_json_include!(actual: parsed_json, expected: expected_json);
    }};
}

macro_rules! assert_parse_module_to_throw {
    ($input_str: expr, $expected_error: expr) => {{
        use hippo_js_parser::Parser;

        use pretty_assertions::assert_eq;

        let mut parser = Parser::new($input_str);

        let ast_json = parser.parse_module().unwrap_err();

        assert_eq!(ast_json, $expected_error);
    }};
}

pub(crate) use {assert_parse_module_to_throw, assert_parser_module_eq, assert_parser_script_eq};
