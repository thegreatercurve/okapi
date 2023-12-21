use hippo_js_parser::ParserError;

use crate::lexer::{
    common::assert_lexer_eq,
    utils::{identifier, illegal, number_literal},
};

#[test]
fn numbers_integers() {
    assert_lexer_eq!("1234567890", vec![number_literal("1234567890", 0, 10)]);
    assert_lexer_eq!("1234567890", vec![number_literal("1234567890", 0, 10)]);
}

#[test]
fn numbers_floats() {
    assert_lexer_eq!("1212345.6789", vec![number_literal("1212345.6789", 0, 12)]);
    assert_lexer_eq!("0.0123456789", vec![number_literal("0.0123456789", 0, 12)]);
    assert_lexer_eq!(".0123456789", vec![number_literal("0.0123456789", 0, 11)]);

    // Weird JavasScript rounding idiosyncrasies.
    assert_lexer_eq!(
        ".12345678900011222",
        vec![number_literal("0.12345678900011221", 0, 18)]
    );
    assert_lexer_eq!(
        "1212345.678900",
        vec![number_literal("1212345.6789", 0, 14)]
    );
}

#[test]
fn numbers_binary_integer_literals() {
    assert_lexer_eq!("0b10101010", vec![number_literal("170", 0, 10)]);

    assert_lexer_eq!("0B10101010", vec![number_literal("170", 0, 10)]);
}

#[test]
fn numbers_binary_integer_literals_invalid() {
    assert_lexer_eq!(
        "0b1012111111",
        vec![number_literal("5", 0, 5), number_literal("2111111", 5, 12)]
    );
}

#[test]
fn numbers_octal_integer_literals_modern() {
    assert_lexer_eq!("0o12345670", vec![number_literal("2739128", 0, 10)]);

    assert_lexer_eq!("0O12345670", vec![number_literal("2739128", 0, 10)]);
}

#[test]
fn numbers_octal_integer_literals_modern_invalid() {
    assert_lexer_eq!(
        "0o123456780",
        vec![number_literal("342391", 0, 9), number_literal("80", 9, 11)]
    );
}

#[test]
fn numbers_hexadecimal_integer_literals() {
    assert_lexer_eq!(
        "0x1234567890abcdef",
        vec![number_literal("1311768467294899700", 0, 18)]
    );
    assert_lexer_eq!(
        "0X1234567890abcdef",
        vec![number_literal("1311768467294899700", 0, 18)]
    );
}

#[test]
fn numbers_hexadecimal_integer_literals_invalid() {
    assert_lexer_eq!(
        "0X1234567890abcdefgggg123",
        vec![
            number_literal("1311768467294899700", 0, 18),
            identifier("gggg123", 18, 25),
        ]
    );
}

#[test]
fn numbers_octal_integer_literals_legacy() {
    assert_lexer_eq!("0012345670", vec![number_literal("2739128", 0, 10)]);

    assert_lexer_eq!("0012345670", vec![number_literal("2739128", 0, 10)]);
}

#[test]
fn numbers_octal_integer_literals_legacy_invalid() {
    assert_lexer_eq!(
        "00123459670",
        vec![number_literal("5349", 0, 7), number_literal("9670", 7, 11)]
    );

    assert_lexer_eq!(
        "00123456970",
        vec![number_literal("42798", 0, 8), number_literal("970", 8, 11)]
    );
}

#[test]
fn numbers_numeric_separator() {
    assert_lexer_eq!("123_456_789", vec![number_literal("123456789", 0, 11)]);
}

#[test]
fn numbers_numeric_separator_invalid() {
    assert_lexer_eq!(
        "123__456_789",
        vec![illegal(
            ParserError::InvalidNumericSeparatorAtSibling,
            0,
            12
        )]
    );
    assert_lexer_eq!(
        "123_456_789_",
        vec![illegal(ParserError::InvalidNumericSeparatorAtEnd, 0, 12)]
    );
}

#[test]
fn numbers_zero() {
    assert_lexer_eq!("0", vec![number_literal("0", 0, 1)]);
}
