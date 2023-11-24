use std::{char, str::Chars, string::ParseError};

use hippo_unicode::{is_unicode_id_continue, is_unicode_id_start};

use crate::{errors::ParserError, KeywordKind, TokenType};

use super::char_utils::{is_line_terminator, is_whitespace};

const ZWNJ: char = '\u{200C}'; // Used in IdentifierPart
const ZWJ: char = '\u{200D}'; // Used in IdentifierPart

#[derive(Debug)]
pub struct Scanner<'a> {
    input: Chars<'a>,
    input_length: usize,
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
        '{' | '}' | '(' | ')' | '[' | ']' | '.' | ';' | ',' | '<' | '>' | '=' | '!' | '+' | '-'
        | '*' | '%' | '&' | '|' | '^' | '~' | '?' | ':' => true,
        _ => false,
    }
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut input_chars = input.chars();

        let first_char = input_chars.next().unwrap();

        let lexer = Self {
            input: input_chars,
            input_length: input.len(),
            read_index: 0,
            ch: first_char,
        };

        return lexer;
    }

    fn read_char(&mut self) {
        self.ch = self.peek_char();
        self.read_index += 1;
    }

    fn peek_char(&mut self) -> char {
        self.peek_char_nth(1)
    }

    fn peek_char_nth(&mut self, offset: usize) -> char {
        self.input
            .nth(self.read_index + offset)
            .unwrap_or(0 as char)
    }

    pub fn is_end_of_file(&mut self) -> bool {
        self.read_index >= self.input_length
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
        match self.ch {
            '#' => self.scan_private_identifier(),
            '1'..='9' => TokenType::Number,
            _ if is_punctuator_start(self.ch) => self.scan_punctuator(),
            _ if is_identifier_start(self.ch) => self.scan_identifier_name_or_keyword(),
            _ => TokenType::Illegal,
        }
    }

    // https://tc39.es/ecma262/#sec-punctuators
    // 12.8 Punctuators
    // ```text
    // Punctuator ::
    //  OptionalChainingPunctuator
    //  OtherPunctuator
    // OptionalChainingPunctuator ::
    //  ?. [lookahead ∉ DecimalDigit]
    // OtherPunctuator :: one of
    //  { ( ) [ ] . ... ; , < > <= >= == != === !== + - * % ** ++ -- << >> >>> & | ^ ! ~ && || ?? ? : = += -= *= %= **= <<= >>= >>>= &= |= ^= &&= ||= ??= =>
    // DivPunctuator ::
    //  /
    //  /=
    // RightBracePunctuator ::
    //  }
    // ```
    fn scan_punctuator(&mut self) -> TokenType {
        match self.ch {
            '{' => TokenType::LeftCurlyBrace,
            '}' => TokenType::RightCurlyBrace,
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            '[' => TokenType::LeftSquareBracket,
            ']' => TokenType::RightSquareBracket,
            '.' => TokenType::Dot,
            ';' => TokenType::Semicolon,
            ',' => TokenType::Comma,
            '<' => TokenType::LessThan,
            '>' => TokenType::GreaterThan,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();

                    TokenType::Assign
                } else {
                    TokenType::Assign
                }
            }
            '!' => TokenType::Bang,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '%' => TokenType::Percent,
            '&' => TokenType::Ampersand,
            '|' => TokenType::Pipe,
            '^' => TokenType::Caret,
            '~' => TokenType::Tilde,
            '?' => TokenType::QuestionMark,
            ':' => TokenType::Colon,
            _ => break,
        }
    }

    // https://tc39.es/ecma262/#sec-names-and-keywords
    // 12.7 Names and Keywords
    // ```text
    // PrivateIdentifier ::
    //  # IdentifierName
    //
    // IdentifierName ::
    //  IdentifierStart
    //  IdentifierName IdentifierPart
    //
    // IdentifierStart ::
    //  IdentifierStartChar
    //  \ UnicodeEscapeSequence
    //
    // IdentifierPart ::
    //  IdentifierPartChar
    //  \ UnicodeEscapeSequence
    //
    // IdentifierStartChar ::
    //  UnicodeIDStart
    //  $
    //  _
    //
    // IdentifierPartChar ::
    //  UnicodeIDContinue
    //  $
    //  <ZWNJ>
    //  <ZWJ>
    // ```
    fn scan_identifier_name_or_keyword(&mut self) -> TokenType {
        let start_index = self.read_index;

        self.identifier_start().unwrap();

        let keyword_or_identifer_name = &self.input.as_str()[start_index..self.read_index];

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

        self.read_char();

        self.identifier_part()
    }

    fn identifier_part(&mut self) -> Result<(), ParserError> {
        let asdf = while is_identifier_part(self.ch) || self.ch == '\\' {
            if self.ch == '\\' {
                if self.peek_char() != 'u' {
                    return Err(ParserError::IllegalCharacter);
                }

                // self.unicode_escape_sequence();
            }

            self.read_char();
        };

        Ok(asdf)
    }

    // https://tc39.es/ecma262/#sec-keywords-and-reserved-words
    // > Those that are contextually disallowed as identifiers, in strict mode code: let, static, implements, interface, package, private, protected, and public;
    // ```text
    // ReservedWord :: one of
    //  await break case catch class const continue debugger default delete do else enum export extends false finally for function if import in instanceof new null return super switch this throw true try typeof var void while with yield
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
    //  # IdentifierName
    // ```
    fn scan_private_identifier(&mut self) -> TokenType {
        let start_index = self.read_index;

        self.read_char();

        self.identifier_start().unwrap();

        let identifer_name = &self.input.as_str()[start_index..self.read_index];

        TokenType::Identifier(identifer_name.to_string())
    }

    // https://tc39.es/ecma262/#prod-UnicodeEscapeSequence
    // ```text
    // UnicodeEscapeSequence ::
    //  `u` Hex4Digits
    //  `u{` CodePoint `}`
    // ```
    fn unicode_escape_sequence(&mut self) {
        if self.peek_char() == '{' {
            self.read_char();
        } else {
            self.hex_four_digits();
        }
    }

    // https://tc39.es/ecma262/#prod-Hex4Digits
    // ```text
    // Hex4Digits ::
    //  HexDigit HexDigit HexDigit HexDigit
    // ```
    fn hex_four_digits(&mut self) -> Result<char, ParserError> {
        let start_index = self.read_index;

        while self.ch.is_ascii_hexdigit() {
            self.read_char();
        }

        let hex_digits = &self.input.as_str()[start_index..self.read_index];

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
