use crate::lexer::utils::regular_expression_literal;

macro_rules! assert_lexer_eq {
    ($input_str: expr, $tokens: expr) => {{
        use hippo_js_parser::{Config, GoalSymbol, Lexer};
        use std::collections::VecDeque;

        let mut tests = VecDeque::from($tokens);

        let mut lexer = Lexer::new($input_str, Config::default());

        // Expressly set the goal symbol to RegExp.
        lexer.goal_symbol = GoalSymbol::RegExp;

        while !lexer.is_end_of_file() {
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
    }};
}

#[test]
fn regular_expression_simple() {
    assert_lexer_eq!("/123/", vec![regular_expression_literal("/123/", 0, 5)]);
    assert_lexer_eq!("/123/gu", vec![regular_expression_literal("/123/gu", 0, 7)]);
}
