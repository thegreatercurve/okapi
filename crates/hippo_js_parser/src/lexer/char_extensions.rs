pub trait CharExtensions {
    fn is_start_of_identifier_or_keyword(&self) -> bool;
    fn is_within_identifier_or_keyword(&self) -> bool;
    fn is_string_literal(&self) -> bool;
    fn is_numeric(&self) -> bool;
    fn is_whitespace(&self) -> bool;
}

impl CharExtensions for char {
    fn is_start_of_identifier_or_keyword(&self) -> bool {
        match &self {
            'a'..='z' | 'A'..='Z' | '_' | '$' => true,
            _ => false,
        }
    }

    fn is_within_identifier_or_keyword(&self) -> bool {
        match &self {
            'a'..='z' | 'A'..='Z' | '_' | '$' | '0'..='9' => true,
            _ => false,
        }
    }

    fn is_string_literal(&self) -> bool {
        match &self {
            '\'' | '\"' => true,
            _ => false,
        }
    }

    fn is_numeric(&self) -> bool {
        match &self {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_whitespace(&self) -> bool {
        match &self {
            ' ' | '\t' | '\n' | '\r' => true,
            _ => false,
        }
    }
}
