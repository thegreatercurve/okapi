use std::collections::VecDeque;

use hippo_js_parser::{lexer::Lexer, tokens::TokenType};

#[test]
fn test_simple_variable_assignment() {
    const INPUT: &str = "
    \"hello world\";
    'hello world';
    `hello world`;
";

    let mut tests = VecDeque::from(vec![
        TokenType::String,
        TokenType::SemiColon,
        TokenType::String,
        TokenType::SemiColon,
        TokenType::String,
        TokenType::SemiColon,
    ]);

    let mut lexer = Lexer::new(INPUT);

    loop {
        let token = lexer.next_token();

        if token == TokenType::EOF {
            break;
        }

        let expected_token = tests.pop_front().unwrap();

        assert_eq!(token, expected_token);
    }
}
