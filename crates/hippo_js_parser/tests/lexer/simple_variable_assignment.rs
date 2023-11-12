use std::collections::VecDeque;

use hippo_js_parser::{lexer::Lexer, tokens::TokenType};

#[test]
fn test_simple_variable_assignment() {
    const INPUT: &str = "
    const foo = 'bar';
    let baz = 1234567;
    var baz = 123;
";

    let mut tests = VecDeque::from(vec![
        TokenType::Constant,
        TokenType::Name,
        TokenType::Assign,
        TokenType::String,
        TokenType::SemiColon,
        TokenType::Let,
        TokenType::Name,
        TokenType::Assign,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Variable,
        TokenType::Name,
        TokenType::Assign,
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
