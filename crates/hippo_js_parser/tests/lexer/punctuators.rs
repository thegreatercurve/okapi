use hippo_js_parser::{Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn punctuators() {
    assert_lexer_eq!(
        r#"?. { ( ) [ ] . ... ; , < > <= >= == != === !== + - * % ** ++ -- << >> >>> & | ^ ! ~ && || ?? ? : = += -= *= %= **= <<= >>= >>>= &= |= ^= &&= ||= ??= => / /= }"#,
        vec![
            Token::new(TokenKind::OptionalChaining, 0, 2),
            Token::new(TokenKind::LeftCurlyBrace, 3, 4),
            Token::new(TokenKind::LeftParenthesis, 5, 6),
            Token::new(TokenKind::RightParenthesis, 7, 8),
            Token::new(TokenKind::LeftSquareBracket, 9, 10),
            Token::new(TokenKind::RightSquareBracket, 11, 12),
            Token::new(TokenKind::Dot, 13, 14),
            Token::new(TokenKind::Ellipsis, 15, 18),
            Token::new(TokenKind::Semicolon, 19, 20),
            Token::new(TokenKind::Comma, 21, 22),
            Token::new(TokenKind::LessThan, 23, 24),
            Token::new(TokenKind::GreaterThan, 25, 26),
            Token::new(TokenKind::LessThanOrEqual, 27, 29),
            Token::new(TokenKind::GreaterThanOrEqual, 30, 32),
            Token::new(TokenKind::Equal, 33, 35),
            Token::new(TokenKind::NotEqual, 36, 38),
            Token::new(TokenKind::StrictEqual, 39, 42),
            Token::new(TokenKind::StrictNotEqual, 43, 46),
            Token::new(TokenKind::Addition, 47, 48),
            Token::new(TokenKind::Subtraction, 49, 50),
            Token::new(TokenKind::Multiplication, 51, 52),
            Token::new(TokenKind::Modulus, 53, 54),
            Token::new(TokenKind::Exponentiation, 55, 57),
            Token::new(TokenKind::Increment, 58, 60),
            Token::new(TokenKind::Decrement, 61, 63),
            Token::new(TokenKind::LeftShift, 64, 66),
            Token::new(TokenKind::RightShift, 67, 69),
            Token::new(TokenKind::UnsignedRightShift, 70, 73),
            Token::new(TokenKind::BitwiseAnd, 74, 75),
            Token::new(TokenKind::BitwiseOr, 76, 77),
            Token::new(TokenKind::BitwiseXor, 78, 79),
            Token::new(TokenKind::LogicalNot, 80, 81),
            Token::new(TokenKind::BitwiseNot, 82, 83),
            Token::new(TokenKind::LogicalAnd, 84, 86),
            Token::new(TokenKind::LogicalOr, 87, 89),
            Token::new(TokenKind::NullishCoalescing, 90, 92),
            Token::new(TokenKind::QuestionMark, 93, 94),
            Token::new(TokenKind::Colon, 95, 96),
            Token::new(TokenKind::Assignment, 97, 98),
            Token::new(TokenKind::PlusAssignment, 99, 101),
            Token::new(TokenKind::MinusAssignment, 102, 104),
            Token::new(TokenKind::MultiplyAssignment, 105, 107),
            Token::new(TokenKind::ModulusAssignment, 108, 110),
            Token::new(TokenKind::ExponentiationAssignment, 111, 114),
            Token::new(TokenKind::LeftShiftAssignment, 115, 118),
            Token::new(TokenKind::RightShiftAssignment, 119, 122),
            Token::new(TokenKind::UnsignedRightShiftAssignment, 123, 127),
            Token::new(TokenKind::BitwiseAndAssignment, 128, 130),
            Token::new(TokenKind::BitwiseOrAssignment, 131, 133),
            Token::new(TokenKind::BitwiseXorAssignment, 134, 136),
            Token::new(TokenKind::LogicalAndAssignment, 137, 140),
            Token::new(TokenKind::LogicalOrAssignment, 141, 144),
            Token::new(TokenKind::NullishCoalescingAssignment, 145, 148),
            Token::new(TokenKind::ArrowFunction, 149, 151),
            Token::new(TokenKind::Division, 152, 153),
            Token::new(TokenKind::DivisionAssignment, 154, 156),
            Token::new(TokenKind::RightCurlyBrace, 157, 158),
        ]
    );
}
