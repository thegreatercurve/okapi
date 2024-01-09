macro_rules! assert_parser_eq {
    ($input_str: expr, $expected_ast: expr) => {{
        use hippo_js_parser::Parser;
        use pretty_assertions::assert_eq;

        let mut parser = Parser::new($input_str);

        let ast_json = parser.parse_json().unwrap();

        assert_eq!(
            $expected_ast, ast_json,
            "Expected token {:#?}, but found {:#?}",
            $expected_ast, ast_json,
        );
    }};
}

pub(crate) use assert_parser_eq;
