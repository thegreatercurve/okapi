use hippo_js_parser::{ParserError, Token, TokenKind, TokenValue};

pub fn string_literal(value: &str, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::StringLiteral,
        start,
        end,
        value: TokenValue::String(value.to_string()),
    }
}

pub fn identifier(value: &str, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::Identifier,
        start,
        end,
        value: TokenValue::String(value.to_string()),
    }
}

pub fn punctuator(kind: TokenKind, start: usize, end: usize) -> Token {
    Token {
        kind,
        start,
        end,
        value: TokenValue::Null,
    }
}

pub fn number_literal(raw_value: &str, parsed_value: f64, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::NumberLiteral,
        start,
        end,
        value: TokenValue::Number {
            raw: raw_value.to_string(),
            value: parsed_value,
        },
    }
}

pub fn regular_expression_literal(value: &str, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::RegularExpressionLiteral,
        start,
        end,
        value: TokenValue::String(value.to_string()),
    }
}

pub fn illegal(value: ParserError, start: usize, end: usize) -> Token {
    Token {
        kind: TokenKind::Illegal,
        start,
        end,
        value: TokenValue::String(value.to_string()),
    }
}
