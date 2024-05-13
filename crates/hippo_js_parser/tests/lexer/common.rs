macro_rules! assert_lexer_eq {
    ($input_str: expr, $tokens: expr) => {{
        use hippo_js_parser::{Config, Lexer};
        use pretty_assertions::assert_eq;
        use std::collections::VecDeque;

        let mut tests = VecDeque::from($tokens);

        let mut lexer = Lexer::new($input_str, Config::default());

        while !lexer.is_end_of_file() && tests.len() > 0 {
            let token = lexer.next_token();

            let expected_token = tests
                .pop_front()
                .unwrap_or_else(|| panic!("Unexpected end to queue"));

            assert_eq!(
                expected_token, token,
                "Expected token {:?}, but found {:?}",
                expected_token, token,
            );
        }

        if tests.len() > 0 {
            panic!("Unexpected tokens remainaing: {:?}", tests);
        }
    }};
}

pub(crate) use assert_lexer_eq;
