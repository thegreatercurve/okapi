use hippo_unicode::{is_unicode_id_continue, is_unicode_id_start};

use crate::{errors::ParserError, KeywordKind, TokenType};

// 12.1 Unicode Format-Control Characters
// https://tc39.es/ecma262/#sec-unicode-format-control-characters
//

const ZWNJ: char = '\u{200C}'; // Used in IdentifierPart
const ZWJ: char = '\u{200D}'; // Used in IdentifierPart
const ZWNBSP: char = '\u{FEFF}'; // Used in WhiteSpace

// 12.2 White Space
// https://tc39.es/ecma262/#sec-white-space
const TAB: char = '\u{0009}';
const VT: char = '\u{000B}';
const FF: char = '\u{000C}';
const SP: char = '\u{0020}';
const NBSP: char = '\u{00A0}';

pub fn is_whitespace(ch: char) -> bool {
    match ch {
        TAB | VT | FF | SP | NBSP | ZWNBSP => true,
        _ => false,
    }
}

// 12.3 Line Terminators
// https://tc39.es/ecma262/#sec-line-terminators

const LF: char = '\u{000A}';
const CR: char = '\u{000D}';
const LS: char = '\u{2028}';
const PS: char = '\u{2029}';

pub fn is_line_terminator(ch: char) -> bool {
    match ch {
        LF | CR | LS | PS => true,
        _ => false,
    }
}

#[derive(Debug)]
pub struct Scanner<'a> {
    source_str: &'a str,
    read_index: usize,
    ch: char,
}

fn is_identifier_start(ch: char) -> bool {
    if ch.is_ascii() {
        ch == '$' || ch == '_' || ch.is_ascii_alphabetic()
    } else {
        is_unicode_id_start(ch)
    }
}

fn is_identifier_part(ch: char) -> bool {
    if ch.is_ascii() {
        ch == '$' || ch == '_' || ch.is_ascii_alphanumeric()
    } else {
        is_unicode_id_continue(ch) || ch == ZWNJ || ch == ZWJ
    }
}

fn is_punctuator_start(ch: char) -> bool {
    match ch {
        '{' | '(' | ')' | '[' | ']' | '.' | ';' | ',' | '<' | '>' | '=' | '!' | '+' | '-' | '*'
        | '%' | '&' | '|' | '^' | '~' | '?' | ':' | '/' | '}' => true,
        _ => false,
    }
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let lexer = Self {
            source_str: input,
            read_index: 0,
            ch: input.chars().next().unwrap(),
        };

        return lexer;
    }

    fn read_char(&mut self) {
        self.read_char_nth(1);
    }

    fn read_char_nth(&mut self, offset: usize) {
        self.ch = self.peek_char_nth(offset);

        self.read_index += offset;
    }

    fn peek_char(&mut self) -> char {
        self.peek_char_nth(1)
    }

    fn peek_char_nth(&mut self, offset: usize) -> char {
        self.source_str
            .chars()
            .nth(self.read_index + offset)
            .unwrap_or(0 as char)
    }

    pub fn is_end_of_file(&mut self) -> bool {
        self.read_index >= self.source_str.len()
    }

    fn skip_whitespace(&mut self) {
        while is_whitespace(self.ch) || is_line_terminator(self.ch) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> TokenType {
        self.skip_whitespace();

        if self.is_end_of_file() {
            return TokenType::EOF;
        }

        let token = self.advance();

        self.read_char();

        token
    }

    fn advance(&mut self) -> TokenType {
        let token = match self.ch {
            '#' => self.scan_private_identifier(),
            '1'..='9' => TokenType::NumberLiteral,
            '\'' | '"' => self.scan_string_literal().unwrap_or(TokenType::Illegal),
            _ if is_punctuator_start(self.ch) => self.scan_punctuator(),
            _ if is_identifier_start(self.ch) => self.scan_identifier_name_or_keyword(),
            _ => TokenType::Illegal,
        };

        token
    }

    // https://tc39.es/ecma262/#sec-punctuators
    // 12.8 Punctuators
    // ```text
    // Punctuator ::
    //   OptionalChainingPunctuator
    //   OtherPunctuator
    // OptionalChainingPunctuator ::
    //   ?. [lookahead ∉ DecimalDigit]
    // OtherPunctuator :: one of
    //   { ( ) [ ] . ... ; , < > <= >= == != === !== + - * % ** ++ -- << >> >>> & | ^ ! ~ && || ?? ? : = += -= *= %= **= <<= >>= >>>= &= |= ^= &&= ||= ??= =>
    // DivPunctuator ::
    //   /
    //   /=
    // RightBracePunctuator ::
    //   }
    // ```
    fn scan_punctuator(&mut self) -> TokenType {
        match self.ch {
            '{' => TokenType::LeftCurlyBrace,
            '}' => TokenType::RightCurlyBrace,
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            '[' => TokenType::LeftSquareBracket,
            ']' => TokenType::RightSquareBracket,
            '.' => {
                if self.peek_char() == '.' && self.peek_char_nth(1) == '.' {
                    self.read_char();
                    self.read_char();

                    TokenType::Ellipsis
                } else {
                    TokenType::Dot
                }
            }
            ';' => TokenType::Semicolon,
            ',' => TokenType::Comma,
            '<' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '<' && peek_char_1 == '=' {
                    self.read_char();
                    self.read_char();

                    TokenType::LeftShiftAssignment
                } else if peek_char == '<' {
                    self.read_char();

                    TokenType::LeftShift
                } else if peek_char == '=' {
                    self.read_char();

                    TokenType::LessThanOrEqual
                } else {
                    TokenType::LessThan
                }
            }
            '>' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);
                let peek_char_2 = self.peek_char_nth(3);

                if peek_char == '>' && peek_char_1 == '>' && peek_char_2 == '=' {
                    self.read_char();
                    self.read_char();
                    self.read_char();

                    TokenType::UnsignedRightShiftAssignment
                } else if peek_char == '>' && peek_char_1 == '>' {
                    self.read_char();
                    self.read_char();

                    TokenType::UnsignedRightShift
                } else if peek_char == '>' && peek_char_1 == '=' {
                    self.read_char();
                    self.read_char();

                    TokenType::RightShiftAssignment
                } else if peek_char == '>' {
                    self.read_char();

                    TokenType::RightShift
                } else if peek_char == '=' {
                    self.read_char();

                    TokenType::GreaterThanOrEqual
                } else {
                    TokenType::GreaterThan
                }
            }
            '=' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '=' && peek_char_1 == '=' {
                    self.read_char();
                    self.read_char();

                    TokenType::StrictEqual
                } else if peek_char == '=' {
                    self.read_char();

                    TokenType::Equal
                } else if peek_char == '>' {
                    self.read_char();

                    TokenType::ArrowFunction
                } else {
                    TokenType::Assignment
                }
            }
            '!' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '=' && peek_char_1 == '=' {
                    self.read_char();
                    self.read_char();

                    TokenType::StrictNotEqual
                } else if peek_char == '=' {
                    self.read_char();

                    TokenType::NotEqual
                } else {
                    TokenType::LogicalNot
                }
            }
            '+' => {
                let peek_char = self.peek_char();

                if peek_char == '=' {
                    self.read_char();

                    TokenType::PlusAssignment
                } else if peek_char == '+' {
                    self.read_char();

                    TokenType::Increment
                } else {
                    TokenType::Addition
                }
            }
            '-' => {
                let peek_char = self.peek_char();

                if peek_char == '=' {
                    self.read_char();

                    TokenType::MinusAssignment
                } else if peek_char == '-' {
                    self.read_char();

                    TokenType::Decrement
                } else {
                    TokenType::Subtraction
                }
            }
            '*' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '*' && peek_char_1 == '=' {
                    self.read_char();
                    self.read_char();

                    TokenType::ExponentiationAssignment
                } else if peek_char == '=' {
                    self.read_char();

                    TokenType::MultiplyAssignment
                } else if peek_char == '*' {
                    self.read_char();

                    TokenType::Exponentiation
                } else {
                    TokenType::Multiplication
                }
            }
            '/' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenType::DivisionAssignment
                } else {
                    TokenType::Division
                }
            }
            '%' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenType::ModulusAssignment
                } else {
                    TokenType::Modulus
                }
            }
            '&' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '&' && peek_char_1 == '=' {
                    self.read_char();
                    self.read_char();

                    TokenType::LogicalAndAssignment
                } else if peek_char == '=' {
                    self.read_char();

                    TokenType::BitwiseAndAssignment
                } else if peek_char == '&' {
                    self.read_char();

                    TokenType::LogicalAnd
                } else {
                    TokenType::BitwiseAnd
                }
            }
            '|' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '|' && peek_char_1 == '=' {
                    self.read_char();
                    self.read_char();

                    TokenType::LogicalOrAssignment
                } else if peek_char == '=' {
                    self.read_char();

                    TokenType::BitwiseOrAssignment
                } else if peek_char == '|' {
                    self.read_char();

                    TokenType::LogicalOr
                } else {
                    TokenType::BitwiseOr
                }
            }
            '^' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenType::BitwiseXorAssignment
                } else {
                    TokenType::BitwiseXor
                }
            }
            '~' => TokenType::BitwiseNot,
            '?' => {
                let peek_char = self.peek_char();
                let peek_char_1 = self.peek_char_nth(2);

                if peek_char == '?' && peek_char_1 == '=' {
                    self.read_char();
                    self.read_char();

                    TokenType::NullishCoalescingAssignment
                } else if peek_char == '?' {
                    self.read_char();

                    TokenType::NullishCoalescing
                } else if peek_char == '.' {
                    self.read_char();

                    TokenType::OptionalChaining
                } else {
                    TokenType::QuestionMark
                }
            }
            ':' => TokenType::Colon,
            _ => TokenType::Illegal,
        }
    }

    // https://tc39.es/ecma262/#sec-names-and-keywords
    // 12.7 Names and Keywords
    // ```text
    // PrivateIdentifier ::
    //   # IdentifierName
    //
    // IdentifierName ::
    //   IdentifierStart
    //   IdentifierName IdentifierPart
    //
    // IdentifierStart ::
    //   IdentifierStartChar
    //   \ UnicodeEscapeSequence
    //
    // IdentifierPart ::
    //   IdentifierPartChar
    //   \ UnicodeEscapeSequence
    //
    // IdentifierStartChar ::
    //   UnicodeIDStart
    //   $
    //   _
    //
    // IdentifierPartChar ::
    //   UnicodeIDContinue
    //   $
    //   <ZWNJ>
    //   <ZWJ>
    // ```
    fn scan_identifier_name_or_keyword(&mut self) -> TokenType {
        let start_index = self.read_index;

        self.identifier_start().unwrap();

        let keyword_or_identifer_name = &self.source_str[start_index..self.read_index];

        match self.match_reserved_keyword(keyword_or_identifer_name) {
            Some(keyword_token) => keyword_token,
            None => TokenType::Identifier(keyword_or_identifer_name.to_string()),
        }
    }

    fn identifier_start(&mut self) -> Result<(), ParserError> {
        match self.ch {
            '$' | '_' | _ if self.ch.is_ascii_alphabetic() => self.read_char(),
            '\\' => {
                if self.peek_char() != 'u' {
                    return Err(ParserError::IllegalCharacter);
                }

                // self.unicode_escape_sequence();
            }
            _ if is_unicode_id_start(self.ch) => self.read_char(),
            _ => {
                println!("Illegal character: {}", self.ch);
                return Err(ParserError::IllegalCharacter);
            }
        };

        self.identifier_part()
    }

    fn identifier_part(&mut self) -> Result<(), ParserError> {
        while is_identifier_part(self.ch) || self.ch == '\\' {
            if self.ch == '\\' {
                if self.peek_char() != 'u' {
                    return Err(ParserError::IllegalCharacter);
                }

                // self.unicode_escape_sequence();
            }

            self.read_char();
        }

        Ok(())
    }

    // https://tc39.es/ecma262/#sec-keywords-and-reserved-words
    // > Those that are contextually disallowed as identifiers, in strict mode code: let, static, implements, interface, package, private, protected, and public;
    // ```text
    // ReservedWord :: one of
    //   await break case catch class const continue debugger default delete do else enum export extends false finally for function if import in instanceof new null return super switch this throw true try typeof var void while with yield
    // ```
    fn match_reserved_keyword(&self, keyword_or_identifer: &str) -> Option<TokenType> {
        match keyword_or_identifer {
            "await" => Some(TokenType::Keyword(KeywordKind::Await)),
            "break" => Some(TokenType::Keyword(KeywordKind::Break)),
            "case" => Some(TokenType::Keyword(KeywordKind::Case)),
            "catch" => Some(TokenType::Keyword(KeywordKind::Catch)),
            "class" => Some(TokenType::Keyword(KeywordKind::Class)),
            "const" => Some(TokenType::Keyword(KeywordKind::Const)),
            "continue" => Some(TokenType::Keyword(KeywordKind::Continue)),
            "debugger" => Some(TokenType::Keyword(KeywordKind::Debugger)),
            "default" => Some(TokenType::Keyword(KeywordKind::Default)),
            "delete" => Some(TokenType::Keyword(KeywordKind::Delete)),
            "do" => Some(TokenType::Keyword(KeywordKind::Do)),
            "else" => Some(TokenType::Keyword(KeywordKind::Else)),
            "enum" => Some(TokenType::Keyword(KeywordKind::Enum)),
            "export" => Some(TokenType::Keyword(KeywordKind::Export)),
            "extends" => Some(TokenType::Keyword(KeywordKind::Extends)),
            "false" => Some(TokenType::Keyword(KeywordKind::False)),
            "finally" => Some(TokenType::Keyword(KeywordKind::Finally)),
            "for" => Some(TokenType::Keyword(KeywordKind::For)),
            "function" => Some(TokenType::Keyword(KeywordKind::Function)),
            "if" => Some(TokenType::Keyword(KeywordKind::If)),
            "import" => Some(TokenType::Keyword(KeywordKind::Import)),
            "in" => Some(TokenType::Keyword(KeywordKind::In)),
            "instanceof" => Some(TokenType::Keyword(KeywordKind::Instanceof)),
            "new" => Some(TokenType::Keyword(KeywordKind::New)),
            "null" => Some(TokenType::Keyword(KeywordKind::Null)),
            "return" => Some(TokenType::Keyword(KeywordKind::Return)),
            "super" => Some(TokenType::Keyword(KeywordKind::Super)),
            "switch" => Some(TokenType::Keyword(KeywordKind::Switch)),
            "this" => Some(TokenType::Keyword(KeywordKind::This)),
            "throw" => Some(TokenType::Keyword(KeywordKind::Throw)),
            "true" => Some(TokenType::Keyword(KeywordKind::True)),
            "try" => Some(TokenType::Keyword(KeywordKind::Try)),
            "typeof" => Some(TokenType::Keyword(KeywordKind::Typeof)),
            "var" => Some(TokenType::Keyword(KeywordKind::Var)),
            "void" => Some(TokenType::Keyword(KeywordKind::Void)),
            "while" => Some(TokenType::Keyword(KeywordKind::While)),
            "with" => Some(TokenType::Keyword(KeywordKind::With)),
            "yield" => Some(TokenType::Keyword(KeywordKind::Yield)),

            // Strict mode future reserved words
            "let" => Some(TokenType::Keyword(KeywordKind::Let)),
            "static" => Some(TokenType::Keyword(KeywordKind::Static)),
            "implements" => Some(TokenType::Keyword(KeywordKind::Implements)),
            "interface" => Some(TokenType::Keyword(KeywordKind::Interface)),
            "package" => Some(TokenType::Keyword(KeywordKind::Package)),
            "private" => Some(TokenType::Keyword(KeywordKind::Private)),
            "protected" => Some(TokenType::Keyword(KeywordKind::Protected)),
            "public" => Some(TokenType::Keyword(KeywordKind::Public)),
            _ => None,
        }
    }

    // https://tc39.es/ecma262/#prod-PrivateIdentifier
    // ```text
    // PrivateIdentifier ::
    //   # IdentifierName
    // ```
    fn scan_private_identifier(&mut self) -> TokenType {
        let start_index = self.read_index;

        self.read_char();

        self.identifier_start().unwrap();

        let identifer_name = &self.source_str[start_index..self.read_index];

        TokenType::Identifier(identifer_name.to_string())
    }

    // https://tc39.es/ecma262/#sec-literals-string-literals
    // ```text
    // StringLiteral ::
    //   " DoubleStringCharactersopt "
    //   ' SingleStringCharactersopt '
    // DoubleStringCharacters ::
    //   DoubleStringCharacter DoubleStringCharactersopt
    // SingleStringCharacters ::
    //   SingleStringCharacter SingleStringCharactersopt
    // DoubleStringCharacter ::
    //   SourceCharacter but not one of " or \ or LineTerminator
    //   <LS>
    //   <PS>
    //   \ EscapeSequence
    //   LineContinuation
    // SingleStringCharacter ::
    //   SourceCharacter but not one of ' or \ or LineTerminator
    //   <LS>
    //   <PS>
    //   \ EscapeSequence
    //   LineContinuation
    // ```
    fn scan_string_literal(&mut self) -> Result<TokenType, ParserError> {
        let start_quote_character = self.ch; // '\'' | '"'

        self.read_char();

        while self.ch != start_quote_character {
            if self.ch == CR || self.ch == LF {
                return Err(ParserError::UnterminatedStringLiteral);
            } else if self.ch == '\\' {
                self.scan_escape_sequence();
            }

            self.read_char();
        }

        return Ok(TokenType::StringLiteral);
    }

    fn scan_escape_sequence(&mut self) {
        match self.ch {
            '\'' | '"' | '\\' | 'b' | 'f' | 'n' | 'r' | 't' | 'v' => self.read_char(),
            'x' => {
                if self.hex_two_digits().is_ok() {
                    return;
                }
            }
            'u' => self.scan_unicode_escape_sequence(),
            CR => {
                if self.peek_char() == LF {
                    self.read_char();
                }
            }
            LF | LS | PS => {}
            _ => {
                println!("Illegal character: {}", self.ch);
            }
        }
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //   `u` Hex4Digits
    //   `u{` CodePoint `}`
    // ```
    fn scan_unicode_escape_sequence(&mut self) {
        if self.peek_char() == '{' {
            self.read_char();
        } else {
            self.hex_four_digits();
        }
    }

    // https://tc39.es/ecma262/#prod-HexEscapeSequence
    // ```text
    // HexEscapeSequence ::
    //   x HexDigit HexDigit
    // ```
    fn hex_two_digits(&mut self) -> Result<char, ParserError> {
        let start_index = self.read_index;

        while self.ch.is_ascii_hexdigit() {
            self.read_char();
        }

        let hex_digits = &self.source_str[start_index..self.read_index];

        self.code_point_to_char(hex_digits)
    }

    // https://tc39.es/ecma262/#prod-Hex4Digits
    // ```text
    // Hex4Digits ::
    //   HexDigit HexDigit HexDigit HexDigit
    // ```
    fn hex_four_digits(&mut self) -> Result<char, ParserError> {
        let start_index = self.read_index;

        while self.ch.is_ascii_hexdigit() {
            self.read_char();
        }

        let hex_digits = &self.source_str[start_index..self.read_index];

        self.code_point_to_char(hex_digits)
    }

    fn code_point_to_char(&self, hex_digits: &str) -> Result<char, ParserError> {
        let code_point = u32::from_str_radix(hex_digits, 16).unwrap();

        if !(0xD800..=0xDFFF).contains(&code_point) || hex_digits.len() < 4 {
            Err(ParserError::IllegalCharacter)
        } else {
            Ok(char::from_u32(code_point).unwrap())
        }
    }
}
