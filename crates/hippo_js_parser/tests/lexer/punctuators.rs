use hippo_js_parser::{Token, TokenKind};

use crate::lexer::common::assert_lexer_eq;

#[test]
fn punctuators_123() {
    assert_lexer_eq!(
        r"/ /=",
        vec![
            Token::new(TokenKind::Division, 0, 1, None),
            Token::new(TokenKind::DivisionAssignment, 2, 4, None),
        ]
    );
}

#[test]
fn punctuators() {
    assert_lexer_eq!(
        r"?. { ( ) [ ] . ... ; , < > <= >= == != === !== + - * % ** ++ -- << >> >>> & | ^ ! ~ && || ?? ? : = += -= *= %= **= <<= >>= >>>= &= |= ^= &&= ||= ??= => / /= }",
        vec![
            Token::new(TokenKind::OptionalChaining, 0, 2, None),
            Token::new(TokenKind::LeftCurlyBrace, 3, 4, None),
            Token::new(TokenKind::LeftParenthesis, 5, 6, None),
            Token::new(TokenKind::RightParenthesis, 7, 8, None),
            Token::new(TokenKind::LeftSquareBracket, 9, 10, None),
            Token::new(TokenKind::RightSquareBracket, 11, 12, None),
            Token::new(TokenKind::Dot, 13, 14, None),
            Token::new(TokenKind::Ellipsis, 15, 18, None),
            Token::new(TokenKind::Semicolon, 19, 20, None),
            Token::new(TokenKind::Comma, 21, 22, None),
            Token::new(TokenKind::LessThan, 23, 24, None),
            Token::new(TokenKind::GreaterThan, 25, 26, None),
            Token::new(TokenKind::LessThanOrEqual, 27, 29, None),
            Token::new(TokenKind::GreaterThanOrEqual, 30, 32, None),
            Token::new(TokenKind::Equal, 33, 35, None),
            Token::new(TokenKind::NotEqual, 36, 38, None),
            Token::new(TokenKind::StrictEqual, 39, 42, None),
            Token::new(TokenKind::StrictNotEqual, 43, 46, None),
            Token::new(TokenKind::Addition, 47, 48, None),
            Token::new(TokenKind::Subtraction, 49, 50, None),
            Token::new(TokenKind::Multiplication, 51, 52, None),
            Token::new(TokenKind::Modulus, 53, 54, None),
            Token::new(TokenKind::Exponentiation, 55, 57, None),
            Token::new(TokenKind::Increment, 58, 60, None),
            Token::new(TokenKind::Decrement, 61, 63, None),
            Token::new(TokenKind::LeftShift, 64, 66, None),
            Token::new(TokenKind::RightShift, 67, 69, None),
            Token::new(TokenKind::UnsignedRightShift, 70, 73, None),
            Token::new(TokenKind::BitwiseAnd, 74, 75, None),
            Token::new(TokenKind::BitwiseOr, 76, 77, None),
            Token::new(TokenKind::BitwiseXor, 78, 79, None),
            Token::new(TokenKind::LogicalNot, 80, 81, None),
            Token::new(TokenKind::BitwiseNot, 82, 83, None),
            Token::new(TokenKind::LogicalAnd, 84, 86, None),
            Token::new(TokenKind::LogicalOr, 87, 89, None),
            Token::new(TokenKind::NullishCoalescing, 90, 92, None),
            Token::new(TokenKind::QuestionMark, 93, 94, None),
            Token::new(TokenKind::Colon, 95, 96, None),
            Token::new(TokenKind::Assignment, 97, 98, None),
            Token::new(TokenKind::PlusAssignment, 99, 101, None),
            Token::new(TokenKind::MinusAssignment, 102, 104, None),
            Token::new(TokenKind::MultiplyAssignment, 105, 107, None),
            Token::new(TokenKind::ModulusAssignment, 108, 110, None),
            Token::new(TokenKind::ExponentiationAssignment, 111, 114, None),
            Token::new(TokenKind::LeftShiftAssignment, 115, 118, None),
            Token::new(TokenKind::RightShiftAssignment, 119, 122, None),
            Token::new(TokenKind::UnsignedRightShiftAssignment, 123, 127, None),
            Token::new(TokenKind::BitwiseAndAssignment, 128, 130, None),
            Token::new(TokenKind::BitwiseOrAssignment, 131, 133, None),
            Token::new(TokenKind::BitwiseXorAssignment, 134, 136, None),
            Token::new(TokenKind::LogicalAndAssignment, 137, 140, None),
            Token::new(TokenKind::LogicalOrAssignment, 141, 144, None),
            Token::new(TokenKind::NullishCoalescingAssignment, 145, 148, None),
            Token::new(TokenKind::ArrowFunction, 149, 151, None),
            Token::new(TokenKind::Division, 152, 153, None),
            Token::new(TokenKind::DivisionAssignment, 154, 156, None),
            Token::new(TokenKind::RightCurlyBrace, 157, 158, None),
        ]
    );
}
