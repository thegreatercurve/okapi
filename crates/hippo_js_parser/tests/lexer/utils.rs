use hippo_js_parser::{ParserError, Token, TokenKind};

pub fn string_literal(value: &str, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::StringLiteral,
        start,
        end,
        value: Some(value.to_string()),
    }
}

pub fn identifier(value: &str, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::Identifier,
        start,
        end,
        value: Some(value.to_string()),
    }
}

pub fn number_literal(value: &str, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::NumberLiteral,
        start,
        end,
        value: Some(value.to_string()),
    }
}

pub fn regular_expression_literal(value: &str, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::RegularExpressionLiteral,
        start,
        end,
        value: Some(value.to_string()),
    }
}

pub fn illegal(value: ParserError, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::Illegal,
        start,
        end,
        value: Some(value.to_string()),
    }
}
