use std::collections::VecDeque;

use crate::{KeywordKind, Lexer, TokenKind};

macro_rules! assert_lexer_eq {
    ($input_str: expr, $tokens: expr) => {{
        let mut tests = VecDeque::from($tokens);

        let mut scanner = Lexer::new($input_str);

        while !scanner.is_end_of_file() {
            let token = scanner.next_token();

            let expected_token = tests
                .pop_front()
                .unwrap_or_else(|| panic!("Unexpected end to queue"));

            assert_eq!(
                expected_token, token.kind,
                "Expected token {:?}, but found {:?}",
                expected_token, token.kind,
            );
        }
    }};
}

#[test]
fn keywords_and_identifiers() {
    assert_lexer_eq!(
        "const foo = 1;",
        vec![
            TokenKind::Keyword(KeywordKind::Const),
            TokenKind::Identifier("foo".to_string()),
            TokenKind::Assignment,
            TokenKind::NumberLiteral,
            TokenKind::Semicolon,
        ]
    );

    assert_lexer_eq!(
        "let baz = 1;",
        vec![
            TokenKind::Keyword(KeywordKind::Let),
            TokenKind::Identifier("baz".to_string()),
            TokenKind::Assignment,
            TokenKind::NumberLiteral,
            TokenKind::Semicolon,
        ]
    );

    assert_lexer_eq!(
        "var baz = 1;",
        vec![
            TokenKind::Keyword(KeywordKind::Var),
            TokenKind::Identifier("baz".to_string()),
            TokenKind::Assignment,
            TokenKind::NumberLiteral,
            TokenKind::Semicolon,
        ]
    );

    // assert_lexer_eq!(
    //     r#"var \u{0042}\u{0041}z = 1;"#,
    //     vec![
    //         TokenType::Keyword(KeywordKind::Var),
    //         TokenType::Identifier("baz".to_string()),
    //         TokenType::Assignment,
    //         TokenType::NumberLiteral,
    //         TokenType::Semicolon,
    //     ]
    // );

    assert_lexer_eq!(
        r#"class Foo { #bar = 1; }"#,
        vec![
            TokenKind::Keyword(KeywordKind::Class),
            TokenKind::Identifier("Foo".to_string()),
            TokenKind::LeftCurlyBrace,
            TokenKind::Identifier("#bar".to_string()),
            TokenKind::Assignment,
            TokenKind::NumberLiteral,
            TokenKind::Semicolon,
            TokenKind::RightCurlyBrace,
            TokenKind::Semicolon,
        ]
    );

    // TODO Handle surrogate unciode pairs
}

#[test]
fn punctuators() {
    assert_lexer_eq!(
        r#"?. { ( ) [ ] . ... ; , < > <= >= == != === !== + - * % ** ++ -- << >> >>> & | ^ ! ~ && || ?? ? : = += -= *= %= **= <<= >>= >>>= &= |= ^= &&= ||= ??= => / /= }"#,
        vec![
            TokenKind::OptionalChaining,
            TokenKind::LeftCurlyBrace,
            TokenKind::LeftParenthesis,
            TokenKind::RightParenthesis,
            TokenKind::LeftSquareBracket,
            TokenKind::RightSquareBracket,
            TokenKind::Dot,
            TokenKind::Ellipsis,
            TokenKind::Semicolon,
            TokenKind::Comma,
            TokenKind::LessThan,
            TokenKind::GreaterThan,
            TokenKind::LessThanOrEqual,
            TokenKind::GreaterThanOrEqual,
            TokenKind::Equal,
            TokenKind::NotEqual,
            TokenKind::StrictEqual,
            TokenKind::StrictNotEqual,
            TokenKind::Addition,
            TokenKind::Subtraction,
            TokenKind::Multiplication,
            TokenKind::Modulus,
            TokenKind::Exponentiation,
            TokenKind::Increment,
            TokenKind::Decrement,
            TokenKind::LeftShift,
            TokenKind::RightShift,
            TokenKind::UnsignedRightShift,
            TokenKind::BitwiseAnd,
            TokenKind::BitwiseOr,
            TokenKind::BitwiseXor,
            TokenKind::LogicalNot,
            TokenKind::BitwiseNot,
            TokenKind::LogicalAnd,
            TokenKind::LogicalOr,
            TokenKind::NullishCoalescing,
            TokenKind::QuestionMark,
            TokenKind::Colon,
            TokenKind::Assignment,
            TokenKind::PlusAssignment,
            TokenKind::MinusAssignment,
            TokenKind::MultiplyAssignment,
            TokenKind::ModulusAssignment,
            TokenKind::ExponentiationAssignment,
            TokenKind::LeftShiftAssignment,
            TokenKind::RightShiftAssignment,
            TokenKind::UnsignedRightShiftAssignment,
            TokenKind::BitwiseAndAssignment,
            TokenKind::BitwiseOrAssignment,
            TokenKind::BitwiseXorAssignment,
            TokenKind::LogicalAndAssignment,
            TokenKind::LogicalOrAssignment,
            TokenKind::NullishCoalescingAssignment,
            TokenKind::ArrowFunction,
            TokenKind::Division,
            TokenKind::DivisionAssignment,
            TokenKind::RightCurlyBrace,
        ]
    );
}

#[test]
fn string_literals() {
    assert_lexer_eq!(r#""hello world""#, vec![TokenKind::StringLiteral]);
    assert_lexer_eq!("\"hello world\"", vec![TokenKind::StringLiteral]);

    assert_lexer_eq!(r#""hello\n\tworld""#, vec![TokenKind::StringLiteral]);
    assert_lexer_eq!("\"hello\\b\\tworld\"", vec![TokenKind::StringLiteral]);

    // Hexadecimal escape sequence
    assert_lexer_eq!("\"hello \x4A\x61vaScript\"", vec![TokenKind::StringLiteral]);
    assert_lexer_eq!(
        r#""hello \x4A\x61vaScript""#,
        vec![TokenKind::StringLiteral]
    );

    // Unicode escape sequence
    assert_lexer_eq!("\"hello\\u0020world\"", vec![TokenKind::StringLiteral]);
    assert_lexer_eq!(r#""hello\u0020world""#, vec![TokenKind::StringLiteral]);

    // Unicode escape sequence with surrogate pairs
    assert_lexer_eq!(
        "\"hello\\u0020world\\uD83D\\uDE00\"",
        vec![TokenKind::StringLiteral]
    );
    assert_lexer_eq!(
        r#""hello\u0020world\uD83D\uDE00""#,
        vec![TokenKind::StringLiteral]
    );

    // Code point escape sequence with surrogate pairs
    assert_lexer_eq!("\"hello world \\u{1F607}\"", vec![TokenKind::StringLiteral]);
    assert_lexer_eq!(r#""hello world \u{1F607}""#, vec![TokenKind::StringLiteral]);

    // Unescaped string with complex graphemes.
    assert_lexer_eq!(
        r#""abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉""#,
        vec![TokenKind::StringLiteral]
    );
    assert_lexer_eq!(
        "\"abcdefghijklmnopqrstuvwxyz🙂12345678910'\'10🎉\"",
        vec![TokenKind::StringLiteral]
    );
}
