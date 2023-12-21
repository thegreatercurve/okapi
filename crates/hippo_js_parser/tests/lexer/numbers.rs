use hippo_js_parser::{Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn numbers_integers() {
    assert_lexer_eq!(
        "1234567890",
        vec![Token::number_literal("1234567890".to_string(), 0, 10)]
    );

    assert_lexer_eq!(
        "1234567890",
        vec![Token::number_literal("1234567890".to_string(), 0, 10)]
    );
}

#[test]
fn numbers_floats() {
    assert_lexer_eq!(
        "12345.67890",
        vec![Token::number_literal("12345.67890".to_string(), 0, 13)]
    );

    assert_lexer_eq!(
        "0.1234567890",
        vec![Token::number_literal(".1234567890".to_string(), 0, 13)]
    );

    assert_lexer_eq!(
        ".1234567890",
        vec![Token::number_literal(".1234567890".to_string(), 0, 13)]
    );
}

#[test]
fn numbers_binary_integer_literals() {
    assert_lexer_eq!(
        "0b10101010",
        vec![Token::number_literal("170".to_string(), 0, 10)]
    );

    assert_lexer_eq!(
        "0B10101010",
        vec![Token::number_literal("170".to_string(), 0, 10)]
    );
}

// #[test]
// fn numbers_binary_integer_literals_invalid() {
//     assert_lexer_eq!(
//         "0b1012",
//         vec![Token::number_literal("170".to_string(), 0, 10)]
//     );
// }

#[test]
fn numbers_octal_integer_literals_legacy() {
    assert_lexer_eq!(
        "0o12345670",
        vec![Token::number_literal("2739128".to_string(), 0, 10)]
    );

    assert_lexer_eq!(
        "0O12345670",
        vec![Token::number_literal("2739128".to_string(), 0, 10)]
    );
}

// #[test]
// fn numbers_octal_integer_literals_legacy_invalid() {
//     assert_lexer_eq!(
//         "0o123456780",
//         vec![Token::number_literal("2739128".to_string(), 0, 10)]
//     );
// }

// #[test]
// fn numbers_octal_integer_literals_modern() {
//     assert_lexer_eq!(
//         "0012345670",
//         vec![Token::number_literal("2739128".to_string(), 0, 10)]
//     );

//     assert_lexer_eq!(
//         "0012345670",
//         vec![Token::number_literal("2739128".to_string(), 0, 10)]
//     );
// }

#[test]
fn numbers_hexadecimal_integer_literals() {
    assert_lexer_eq!(
        "0x1234567890abcdef",
        vec![Token::number_literal(
            "1311768467294899695".to_string(),
            0,
            18
        )]
    );

    assert_lexer_eq!(
        "0X1234567890abcdef",
        vec![Token::number_literal(
            "1311768467294899695".to_string(),
            0,
            18
        )]
    );
}

// #[test]
// fn numbers_hexadecimal_integer_literals_invalid() {
//     assert_lexer_eq!(
//         "0X1234567890abcdefg",
//         vec![Token::number_literal(
//             "1311768467294899695".to_string(),
//             0,
//             18
//         )]
//     );
// }

// #[test]
// fn numbers_numeric_separator() {
//     assert_lexer_eq!(
//         "123_456_789",
//         vec![Token::number_literal("123456789".to_string(), 0, 9)]
//     );
// }

// #[test]
// fn numbers_numeric_separator_invalid() {
//     assert_lexer_eq!("123__456_789", vec![Token::new(TokenKind::Illegal, 0, 4)]);
// }
