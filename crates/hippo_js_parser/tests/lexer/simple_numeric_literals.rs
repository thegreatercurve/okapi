use std::collections::VecDeque;

use hippo_js_parser::{lexer::Lexer, tokens::TokenType};

#[test]
fn test_simple_variable_assignment() {
    const INPUT: &str = "
    1;
    12;
    123456;
    123_456_789;
    .123;
    0.123;
    10.123;
    10.123e4;
";

    let mut tests = VecDeque::from(vec![
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Number,
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
