use std::collections::VecDeque;

use hippo_js_parser::{KeywordKind, Scanner, TokenType};

#[test]
fn test_simple_variable_assignment() {
    const INPUT: &str = "
    const foo = 123;
    let baz = 123;
    var baz = 123;
";

    let mut tests = VecDeque::from(vec![
        TokenType::Keyword(KeywordKind::Const),
        TokenType::Identifier("foo".to_string()),
        TokenType::Assign,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Keyword(KeywordKind::Let),
        TokenType::Identifier("baz".to_string()),
        TokenType::Assign,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::Keyword(KeywordKind::Var),
        TokenType::Identifier("baz".to_string()),
        TokenType::Assign,
        TokenType::Number,
        TokenType::SemiColon,
        TokenType::EOF,
    ]);

    let mut scanner = Scanner::new(INPUT);

    while !scanner.is_end_of_file() {
        let token = scanner.next_token();

        let expected_token = tests.pop_front().unwrap();

        println!("{:?} {:?}", expected_token, token);

        assert_eq!(token, expected_token);
    }
}
