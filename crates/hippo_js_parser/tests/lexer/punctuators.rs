use hippo_js_parser::TokenKind;

use crate::lexer::{common::assert_lexer_eq, utils::punctuator};

#[test]
fn punctuators_123() {
    assert_lexer_eq!(
        r"/ /=",
        vec![
            punctuator(TokenKind::Division, 0, 1),
            punctuator(TokenKind::DivisionAssignment, 2, 4),
        ]
    );
}

#[test]
fn punctuators() {
    assert_lexer_eq!(
        r"?. { ( ) [ ] . ... ; , < > <= >= == != === !== + - * % ** ++ -- << >> >>> & | ^ ! ~ && || ?? ? : = += -= *= %= **= <<= >>= >>>= &= |= ^= &&= ||= ??= => / /= }",
        vec![
            punctuator(TokenKind::OptionalChaining, 0, 2),
            punctuator(TokenKind::LeftCurlyBrace, 3, 4),
            punctuator(TokenKind::LeftParenthesis, 5, 6),
            punctuator(TokenKind::RightParenthesis, 7, 8),
            punctuator(TokenKind::LeftSquareBracket, 9, 10),
            punctuator(TokenKind::RightSquareBracket, 11, 12),
            punctuator(TokenKind::Dot, 13, 14),
            punctuator(TokenKind::Ellipsis, 15, 18),
            punctuator(TokenKind::Semicolon, 19, 20),
            punctuator(TokenKind::Comma, 21, 22),
            punctuator(TokenKind::LessThan, 23, 24),
            punctuator(TokenKind::GreaterThan, 25, 26),
            punctuator(TokenKind::LessThanOrEqual, 27, 29),
            punctuator(TokenKind::GreaterThanOrEqual, 30, 32),
            punctuator(TokenKind::Equal, 33, 35),
            punctuator(TokenKind::NotEqual, 36, 38),
            punctuator(TokenKind::StrictEqual, 39, 42),
            punctuator(TokenKind::StrictNotEqual, 43, 46),
            punctuator(TokenKind::Addition, 47, 48),
            punctuator(TokenKind::Subtraction, 49, 50),
            punctuator(TokenKind::Multiplication, 51, 52),
            punctuator(TokenKind::Modulus, 53, 54),
            punctuator(TokenKind::Exponentiation, 55, 57),
            punctuator(TokenKind::Increment, 58, 60),
            punctuator(TokenKind::Decrement, 61, 63),
            punctuator(TokenKind::LeftShift, 64, 66),
            punctuator(TokenKind::RightShift, 67, 69),
            punctuator(TokenKind::UnsignedRightShift, 70, 73),
            punctuator(TokenKind::BitwiseAnd, 74, 75),
            punctuator(TokenKind::BitwiseOr, 76, 77),
            punctuator(TokenKind::BitwiseXor, 78, 79),
            punctuator(TokenKind::LogicalNot, 80, 81),
            punctuator(TokenKind::BitwiseNot, 82, 83),
            punctuator(TokenKind::LogicalAnd, 84, 86),
            punctuator(TokenKind::LogicalOr, 87, 89),
            punctuator(TokenKind::NullishCoalescing, 90, 92),
            punctuator(TokenKind::QuestionMark, 93, 94),
            punctuator(TokenKind::Colon, 95, 96),
            punctuator(TokenKind::Assignment, 97, 98),
            punctuator(TokenKind::PlusAssignment, 99, 101),
            punctuator(TokenKind::MinusAssignment, 102, 104),
            punctuator(TokenKind::MultiplyAssignment, 105, 107),
            punctuator(TokenKind::ModulusAssignment, 108, 110),
            punctuator(TokenKind::ExponentiationAssignment, 111, 114),
            punctuator(TokenKind::LeftShiftAssignment, 115, 118),
            punctuator(TokenKind::RightShiftAssignment, 119, 122),
            punctuator(TokenKind::UnsignedRightShiftAssignment, 123, 127),
            punctuator(TokenKind::BitwiseAndAssignment, 128, 130),
            punctuator(TokenKind::BitwiseOrAssignment, 131, 133),
            punctuator(TokenKind::BitwiseXorAssignment, 134, 136),
            punctuator(TokenKind::LogicalAndAssignment, 137, 140),
            punctuator(TokenKind::LogicalOrAssignment, 141, 144),
            punctuator(TokenKind::NullishCoalescingAssignment, 145, 148),
            punctuator(TokenKind::ArrowFunction, 149, 151),
            punctuator(TokenKind::Division, 152, 153),
            punctuator(TokenKind::DivisionAssignment, 154, 156),
            punctuator(TokenKind::RightCurlyBrace, 157, 158),
        ]
    );
}
