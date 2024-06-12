macro_rules! assert_parser_script_eq {
    ($test_case: expr, $expected_result: expr) => {{
        use crate::parser::sort_json_keys;
        use hippo_js_parser::{Config, Parser};
        use pretty_assertions::assert_eq;

        let parsed = Parser::new(&$test_case, Config::default())
            .parse_script_json()
            .unwrap();
        let parsed_json = serde_json::from_str(&parsed).unwrap();
        let parsed_sorted = sort_json_keys(parsed_json).unwrap();

        let expected_json = serde_json::from_str(&$expected_result).unwrap();
        let expected_sorted = sort_json_keys(expected_json).unwrap();

        assert_eq!(parsed_sorted, expected_sorted);
    }};
}

macro_rules! assert_parser_module_eq {
    ($input_str: expr, $expected_ast: expr) => {{
        use hippo_js_parser::{Config, Parser};

        use pretty_assertions::assert_eq;

        let mut parser = Parser::new($input_str, Config::default());

        let ast_json = parser.parse_module_json().unwrap();

        assert_eq!(
            $expected_ast, ast_json,
            "Expected token {:#?}, but found {:#?}",
            $expected_ast, ast_json,
        );
    }};
}

macro_rules! assert_parse_module_to_throw {
    ($input_str: expr, $expected_error: expr) => {{
        use hippo_js_parser::{Config, Parser};

        use pretty_assertions::assert_eq;

        let mut parser = Parser::new($input_str, Config::default());

        let ast_json = parser.parse_module().unwrap_err();

        assert_eq!(ast_json, $expected_error);
    }};
}

pub(crate) use {assert_parse_module_to_throw, assert_parser_module_eq, assert_parser_script_eq};
