use std::collections::VecDeque;

use crate::{KeywordKind, Scanner, TokenType};

macro_rules! assert_lexer_eq {
    ($input_str: expr, $tokens: expr) => {{
        let mut tests = VecDeque::from($tokens);

        let mut scanner = Scanner::new($input_str);

        while !scanner.is_end_of_file() {
            let token = scanner.next_token();

            let expected_token = tests
                .pop_front()
                .unwrap_or_else(|| panic!("Unexpected end to queue"));

            println!("{:?} {:?}", token, expected_token);

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
            TokenType::NumberLiteral,
            TokenType::Semicolon,
        ]
    );

    assert_lexer_eq!(
        "let baz = 1;",
        vec![
            TokenType::Keyword(KeywordKind::Let),
            TokenType::Identifier("baz".to_string()),
            TokenType::Assignment,
            TokenType::NumberLiteral,
            TokenType::Semicolon,
        ]
    );

    assert_lexer_eq!(
        "var baz = 1;",
        vec![
            TokenType::Keyword(KeywordKind::Var),
            TokenType::Identifier("baz".to_string()),
            TokenType::Assignment,
            TokenType::NumberLiteral,
            TokenType::Semicolon,
        ]
    );

    // assert_lexer_eq!(
    //     r#"var \\u{0042}\\u{0041}z = 1;"#,
    //     vec![
    //         TokenType::Keyword(KeywordKind::Var),
    //         TokenType::Identifier("baz".to_string()),
    //         TokenType::Assign,
    //         TokenType::NumberLiteral,
    //         TokenType::Semicolon,
    //     ]
    // );

    assert_lexer_eq!(
        r#"class Foo { #bar = 1; }"#,
        vec![
            TokenType::Keyword(KeywordKind::Class),
            TokenType::Identifier("Foo".to_string()),
            TokenType::LeftCurlyBrace,
            TokenType::Identifier("#bar".to_string()),
            TokenType::Assignment,
            TokenType::NumberLiteral,
            TokenType::Semicolon,
            TokenType::RightCurlyBrace,
            TokenType::Semicolon,
        ]
    );

    // TODO Handle surrogate unciode pairs
}

#[test]
fn punctuators() {
    assert_lexer_eq!(
        r#"?. { ( ) [ ] . ... ; , < > <= >= == != === !== + - * % ** ++ -- << >> >>> & | ^ ! ~ && || ?? ? : = += -= *= %= **= <<= >>= >>>= &= |= ^= &&= ||= ??= => / /= }"#,
        vec![
            TokenType::OptionalChaining,
            TokenType::LeftCurlyBrace,
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::LeftSquareBracket,
            TokenType::RightSquareBracket,
            TokenType::Dot,
            TokenType::Ellipsis,
            TokenType::Semicolon,
            TokenType::Comma,
            TokenType::LessThan,
            TokenType::GreaterThan,
            TokenType::LessThanOrEqual,
            TokenType::GreaterThanOrEqual,
            TokenType::Equal,
            TokenType::NotEqual,
            TokenType::StrictEqual,
            TokenType::StrictNotEqual,
            TokenType::Addition,
            TokenType::Subtraction,
            TokenType::Multiplication,
            TokenType::Modulus,
            TokenType::Exponentiation,
            TokenType::Increment,
            TokenType::Decrement,
            TokenType::LeftShift,
            TokenType::RightShift,
            TokenType::UnsignedRightShift,
            TokenType::BitwiseAnd,
            TokenType::BitwiseOr,
            TokenType::BitwiseXor,
            TokenType::LogicalNot,
            TokenType::BitwiseNot,
            TokenType::LogicalAnd,
            TokenType::LogicalOr,
            TokenType::NullishCoalescing,
            TokenType::QuestionMark,
            TokenType::Colon,
            TokenType::Assignment,
            TokenType::PlusAssignment,
            TokenType::MinusAssignment,
            TokenType::MultiplyAssignment,
            TokenType::ModulusAssignment,
            TokenType::ExponentiationAssignment,
            TokenType::LeftShiftAssignment,
            TokenType::RightShiftAssignment,
            TokenType::UnsignedRightShiftAssignment,
            TokenType::BitwiseAndAssignment,
            TokenType::BitwiseOrAssignment,
            TokenType::BitwiseXorAssignment,
            TokenType::LogicalAndAssignment,
            TokenType::LogicalOrAssignment,
            TokenType::NullishCoalescingAssignment,
            TokenType::ArrowFunction,
            TokenType::Division,
            TokenType::DivisionAssignment,
            TokenType::RightCurlyBrace,
        ]
    );
}
