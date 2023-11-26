use std::collections::VecDeque;

use hippo_js_parser::{KeywordKind, Scanner, TokenType};

macro_rules! assert_lexer_eq {
    ($input_str: expr, $tokens: expr) => {{
        let mut tests = VecDeque::from($tokens);

        let mut scanner = Scanner::new($input_str);

        while !scanner.is_end_of_file() {
            let token = scanner.next_token();

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
fn keywords_and_identifiers() {
    assert_lexer_eq!(
        "const foo = 1;",
        vec![
            TokenType::Keyword(KeywordKind::Const),
            TokenType::Identifier("foo".to_string()),
            TokenType::Assignment,
            TokenType::Number,
            TokenType::Semicolon,
        ]
    );

    assert_lexer_eq!(
        "let baz = 1;",
        vec![
            TokenType::Keyword(KeywordKind::Let),
            TokenType::Identifier("baz".to_string()),
            TokenType::Assignment,
            TokenType::Number,
            TokenType::Semicolon,
        ]
    );

    assert_lexer_eq!(
        "var baz = 1;",
        vec![
            TokenType::Keyword(KeywordKind::Var),
            TokenType::Identifier("baz".to_string()),
            TokenType::Assignment,
            TokenType::Number,
            TokenType::Semicolon,
        ]
    );

    // assert_lexer_eq!(
    //     r#"var \\u{0042}\\u{0041}z = 1;"#,
    //     vec![
    //         TokenType::Keyword(KeywordKind::Var),
    //         TokenType::Identifier("baz".to_string()),
    //         TokenType::Assign,
    //         TokenType::Number,
    //         TokenType::Semicolon,
    //     ]
    // );

    assert_lexer_eq!(
        r#"class Foo { #bar = 1; }"#,
        vec![
            TokenType::Keyword(KeywordKind::Class),
            TokenType::Identifier("Foo".to_string()),
            // TokenType::LeftBracket,
            TokenType::Identifier("#bar".to_string()),
            TokenType::Assignment,
            TokenType::Number,
            TokenType::Semicolon,
            // TokenType::RightBracket,
            TokenType::Semicolon,
        ]
    );

    // TODO Handle surrogate unciode pairs
}
