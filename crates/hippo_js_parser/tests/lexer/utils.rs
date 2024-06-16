use hippo_js_parser::{ParserError, Token, TokenKind, TokenValue};

pub fn string_literal(
    value: &str,
    raw: &str,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
) -> Token {
    Token {
        kind: TokenKind::StringLiteral,
        start,
        end,
        line,
        column,
        value: TokenValue::String {
            raw: raw.to_string(),
            value: value.to_string(),
        },
        line_terminator: false,
    }
}

pub fn identifier(value: &str, start: usize, end: usize, line: usize, column: usize) -> Token {
    Token {
        kind: TokenKind::Identifier,
        start,
        end,
        line,
        column,
        value: TokenValue::String {
            raw: value.to_string(),
            value: value.to_string(),
        },
        line_terminator: false,
    }
}

pub fn punctuator(kind: TokenKind, start: usize, end: usize, line: usize, column: usize) -> Token {
    Token {
        kind,
        start,
        end,
        line,
        column,
        value: TokenValue::Null,
        line_terminator: false,
    }
}

pub fn number_literal(
    raw_value: &str,
    parsed_value: f64,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
) -> Token {
    Token {
        kind: TokenKind::NumberLiteral,
        start,
        end,
        line,
        column,
        value: TokenValue::Number {
            raw: raw_value.to_string(),
            value: parsed_value,
        },
        line_terminator: false,
    }
}

pub fn template_literal_no_substitution(
    value: &str,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
) -> Token {
    Token {
        kind: TokenKind::TemplateNoSubstitution,
        start,
        end,
        line,
        column,
        value: TokenValue::Template {
            raw: value.to_string(),
            cooked: value.to_string(),
        },
        line_terminator: false,
    }
}

pub fn template_literal_head(
    value: &str,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
) -> Token {
    Token {
        kind: TokenKind::TemplateHead,
        start,
        end,
        line,
        column,
        value: TokenValue::Template {
            raw: value.to_string(),
            cooked: value.to_string(),
        },
        line_terminator: false,
    }
}

pub fn template_literal_middle(
    value: &str,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
) -> Token {
    Token {
        kind: TokenKind::TemplateMiddle,
        start,
        end,
        line,
        column,
        value: TokenValue::Template {
            raw: value.to_string(),
            cooked: value.to_string(),
        },
        line_terminator: false,
    }
}

pub fn template_literal_tail(
    value: &str,
    start: usize,
    end: usize,
    line: usize,
    column: usize,
) -> Token {
    Token {
        kind: TokenKind::TemplateTail,
        start,
        end,
        line,
        column,
        value: TokenValue::Template {
            raw: value.to_string(),
            cooked: value.to_string(),
        },
        line_terminator: false,
    }
}

pub fn illegal(value: ParserError, start: usize, end: usize, line: usize, column: usize) -> Token {
    Token {
        kind: TokenKind::Illegal,
        start,
        end,
        line,
        column,
        value: TokenValue::String {
            raw: value.to_string(),
            value: value.to_string(),
        },
        line_terminator: false,
    }
}
