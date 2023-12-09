macro_rules! assert_lexer_eq {
    ($input_str: expr, $tokens: expr) => {{
        use hippo_js_parser::Lexer;
        use std::collections::VecDeque;

        let mut tests = VecDeque::from($tokens);

        let mut scanner = Lexer::new($input_str);

        while !scanner.is_end_of_file() {
            let token = scanner.next_token();

            let expected_token = tests
                .pop_front()
                .unwrap_or_else(|| panic!("Unexpected end to queue"));

            assert_eq!(
                expected_token, token,
                "Expected token {:#?}, but found {:#?}",
                expected_token, token,
            );
        }
    }};
}

pub(crate) use assert_lexer_eq;
