macro_rules! assert_parser_eq {
    ($test_case: expr, $expected_result: expr) => {{
        use crate::parser::sort_json_keys;
        use hippo_js_parser::Parser;
        use pretty_assertions::assert_eq;

        let parsed = Parser::new(&$test_case).parse_module_json().unwrap();
        let parsed_json = serde_json::from_str(&parsed).unwrap();
        let parsed_sorted = sort_json_keys(parsed_json).unwrap();

        let expected_json = serde_json::from_str(&$expected_result).unwrap();
        let expected_sorted = sort_json_keys(expected_json).unwrap();

        assert_eq!(parsed_sorted, expected_sorted);
    }};
}

macro_rules! assert_parse_script_eq {
    ($input_str: expr, $expected_ast: expr) => {{
        use hippo_js_parser::Parser;
        use pretty_assertions::assert_eq;

        let mut parser = Parser::new($input_str);

        let ast_json = parser.parse_script_json().unwrap();

        assert_eq!(
            $expected_ast, ast_json,
            "Expected token {:#?}, but found {:#?}",
            $expected_ast, ast_json,
        );
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

pub(crate) use {assert_parse_module_to_throw, assert_parse_script_eq, assert_parser_eq};