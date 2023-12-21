use hippo_js_parser::{Token, TokenKind};

pub(crate) fn string_literal(value: String, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::StringLiteral,
        start,
        end,
        value: Some(value),
    }
}

pub(crate) fn identifier(value: String, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::Identifier,
        start,
        end,
        value: Some(value),
    }
}

pub(crate) fn number_literal(value: String, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::NumberLiteral,
        start,
        end,
        value: Some(value),
    }
}
