pub fn is_start_of_identifier_or_keyword(ch: char) -> bool {
    match ch {
        'a'..='z' | 'A'..='Z' | '_' | '$' => true,
        _ => false,
    }
}

pub fn is_within_identifier_or_keyword(ch: char) -> bool {
    match ch {
        'a'..='z' | 'A'..='Z' | '_' | '$' | '0'..='9' => true,
        _ => false,
    }
}

pub fn is_string_literal(ch: char) -> bool {
    match ch {
        '\'' | '\"' => true,
        _ => false,
    }
}

pub fn is_numeric(ch: char) -> bool {
    match ch {
        '0'..='9' => true,
        _ => false,
    }
}

pub fn is_whitespace(ch: char) -> bool {
    match ch {
        ' ' | '\t' | '\n' | '\r' => true,
        _ => false,
    }
}
