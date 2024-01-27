use hippo_js_parser::{ParserError, Token, TokenKind, TokenValue};

use crate::lexer::{
    common::assert_lexer_eq,
    utils::{identifier, illegal, number_literal},
};

#[test]
fn numbers_zero() {
    assert_lexer_eq!("0", vec![number_literal("0", 0.0, 0, 1)]);
}

#[test]
fn numbers_integers() {
    assert_lexer_eq!(
        "1234567890",
        vec![number_literal("1234567890", 1234567890.0, 0, 10)]
    );
    assert_lexer_eq!(
        "1234567890",
        vec![number_literal("1234567890", 1234567890.0, 0, 10)]
    );
}

#[test]
fn numbers_floats() {
    assert_lexer_eq!(
        "1212345.6789",
        vec![number_literal("1212345.6789", 1212345.6789, 0, 12)]
    );
    assert_lexer_eq!(
        "0.0123456789",
        vec![number_literal("0.0123456789", 0.0123456789, 0, 12)]
    );
    assert_lexer_eq!(
        ".0123456789",
        vec![number_literal(".0123456789", 0.0123456789, 0, 11)]
    );
}

#[test]
fn numbers_binary_integer_literals() {
    assert_lexer_eq!(
        "0b10101010",
        vec![number_literal("0b10101010", 170.0, 0, 10)]
    );
    assert_lexer_eq!(
        "0B10101010",
        vec![number_literal("0B10101010", 170.0, 0, 10)]
    );
}

#[test]
fn numbers_binary_integer_literals_invalid() {
    assert_lexer_eq!(
        "0b1012111111",
        vec![
            number_literal("0b101", 5.0, 0, 5),
            number_literal("2111111", 2111111.0, 5, 12)
        ]
    );
}

#[test]
fn numbers_octal_integer_literals_modern() {
    assert_lexer_eq!(
        "0o12345670",
        vec![number_literal("0o12345670", 2739128.0, 0, 10)]
    );
    assert_lexer_eq!(
        "0O12345670",
        vec![number_literal("0O12345670", 2739128.0, 0, 10)]
    );
}

#[test]
fn numbers_octal_integer_literals_modern_invalid() {
    assert_lexer_eq!(
        "0o123456780",
        vec![
            number_literal("0o1234567", 342391.0, 0, 9),
            number_literal("80", 80.0, 9, 11)
        ]
    );
}

#[test]
fn numbers_hexadecimal_integer_literals() {
    assert_lexer_eq!(
        "0x1234567890abcdef",
        vec![number_literal(
            "0x1234567890abcdef",
            1311768467294899700.0,
            0,
            18
        )]
    );
    assert_lexer_eq!(
        "0X1234567890abcdef",
        vec![number_literal(
            "0X1234567890abcdef",
            1311768467294899700.0,
            0,
            18
        )]
    );
}

#[test]
fn numbers_hexadecimal_integer_literals_invalid() {
    assert_lexer_eq!(
        "0X1234567890abcdefgggg123",
        vec![
            number_literal("0X1234567890abcdef", 1311768467294899700.0, 0, 18),
            identifier("gggg123", 18, 25),
        ]
    );
}

#[test]
fn numbers_octal_integer_literals_legacy() {
    assert_lexer_eq!(
        "0012345670",
        vec![number_literal("0012345670", 2739128.0, 0, 10)]
    );

    assert_lexer_eq!(
        "0012345670",
        vec![number_literal("0012345670", 2739128.0, 0, 10)]
    );
}

#[test]
fn numbers_octal_integer_literals_legacy_invalid() {
    assert_lexer_eq!(
        "00123459670",
        vec![
            number_literal("0012345", 5349.0, 0, 7),
            number_literal("9670", 9670.0, 7, 11)
        ]
    );

    assert_lexer_eq!(
        "00123456970",
        vec![
            number_literal("00123456", 42798.0, 0, 8),
            number_literal("970", 970.0, 8, 11)
        ]
    );
}

#[test]
fn numbers_numeric_separator() {
    assert_lexer_eq!(
        "123_456_789",
        vec![number_literal("123_456_789", 123456789.0, 0, 11)]
    );
}

#[test]
fn numbers_numeric_separator_invalid() {
    assert_lexer_eq!(
        "123__456_789",
        vec![
            illegal(ParserError::InvalidNumericSeparatorAtSibling, 0, 4),
            identifier("_456_789", 4, 12)
        ]
    );
    assert_lexer_eq!(
        "123_456_789_",
        vec![illegal(ParserError::InvalidNumericSeparatorAtEnd, 0, 12),]
    );
}

#[test]
fn numbers_exponent() {
    // Positive exponent.
    assert_lexer_eq!(
        "123123123e+12",
        vec![Token {
            kind: TokenKind::NumberLiteral,
            start: 0,
            end: 13,
            value: TokenValue::Number {
                raw: "123123123e+12".to_string(),
                value: 123123123000000000000.0
            },
        }]
    );
    assert_lexer_eq!(
        "123.123123E+12",
        vec![Token {
            kind: TokenKind::NumberLiteral,
            start: 0,
            end: 14,
            value: TokenValue::Number {
                raw: "123.123123E+12".to_string(),
                value: 123123123000000.0
            },
        }]
    );
    assert_lexer_eq!(
        "123123123e12",
        vec![Token {
            kind: TokenKind::NumberLiteral,
            start: 0,
            end: 12,
            value: TokenValue::Number {
                raw: "123123123e12".to_string(),
                value: 123123123000000000000.0
            },
        }]
    );
    assert_lexer_eq!(
        "123.123123E12",
        vec![Token {
            kind: TokenKind::NumberLiteral,
            start: 0,
            end: 13,
            value: TokenValue::Number {
                raw: "123.123123E12".to_string(),
                value: 123123123000000.0
            },
        }]
    );

    // Negative exponent.
    assert_lexer_eq!(
        "123123123e-15",
        vec![Token {
            kind: TokenKind::NumberLiteral,
            start: 0,
            end: 13,
            value: TokenValue::Number {
                raw: "123123123e-15".to_string(),
                value: 0.000000123123123
            },
        }]
    );
    assert_lexer_eq!(
        "123.123123E-15",
        vec![Token {
            kind: TokenKind::NumberLiteral,
            start: 0,
            end: 14,
            value: TokenValue::Number {
                raw: "123.123123E-15".to_string(),
                value: 0.000000000000123123123
            },
        }]
    );
}

#[test]
fn numbers_big_int() {
    assert_lexer_eq!(
        "123n",
        vec![Token {
            kind: TokenKind::BigIntLiteral,
            start: 0,
            end: 4,
            value: TokenValue::BigInt("123n".to_string()),
        },]
    );
    assert_lexer_eq!(
        "0n",
        vec![Token {
            kind: TokenKind::BigIntLiteral,
            start: 0,
            end: 2,
            value: TokenValue::BigInt("0n".to_string()),
        }]
    );
}

#[test]
fn numbers_big_int_invalid() {
    assert_lexer_eq!(
        "123.123n",
        vec![
            Token {
                kind: TokenKind::NumberLiteral,
                start: 0,
                end: 7,
                value: TokenValue::Number {
                    raw: "123.123".to_string(),
                    value: 123.123
                },
            },
            identifier("n", 7, 8)
        ]
    );
}
