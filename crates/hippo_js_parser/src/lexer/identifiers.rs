use hippo_unicode::is_unicode_id_start;

use crate::{KeywordKind, Lexer, ParserError, Token, TokenKind, TokenValue};

use super::utils::is_identifier_part;

// 12.7 Names and Keywords
// https://tc39.es/ecma262/#sec-names-and-keywords
impl<'a> Lexer<'a> {
    // https://tc39.es/ecma262/#sec-names-and-keywords
    pub(crate) fn scan_identifier_name_or_keyword(&mut self) -> Token {
        let start_index = self.read_index;

        let identifier = self.read_identifier_start();

        if identifier.is_err() {
            return Token::new(
                TokenKind::Illegal,
                start_index,
                self.read_index,
                TokenValue::String(identifier.unwrap_err().to_string()),
            );
        };

        let keyword_or_identifer_name = &self.source_str[start_index..self.read_index];

        match self.match_reserved_keyword(keyword_or_identifer_name) {
            Some(keyword_token) => Token::new(
                keyword_token,
                start_index,
                self.read_index,
                TokenValue::Null,
            ),
            None => Token::new(
                TokenKind::Identifier,
                start_index,
                self.read_index,
                TokenValue::String(keyword_or_identifer_name.to_string()),
            ),
        }
    }

    fn read_identifier_start(&mut self) -> Result<(), ParserError> {
        let current_char = self.current_char();

        match current_char {
            '$' | '_' => self.read_char(),
            _ if current_char.is_ascii_alphabetic() => self.read_char(),
            '\\' => {
                if self.peek_char() != 'u' {
                    return Err(ParserError::InvalidIdentifierCharacter);
                }

                let unicode_escape_sequence_u32 = self.read_unicode_escape_sequence();

                if unicode_escape_sequence_u32.is_err() {
                    return Err(unicode_escape_sequence_u32.unwrap_err());
                }
            }
            _ if is_unicode_id_start(current_char) => self.read_char(),
            _ => {
                return Err(ParserError::InvalidIdentifierCharacter);
            }
        };

        self.read_identifier_part()
    }

    fn read_identifier_part(&mut self) -> Result<(), ParserError> {
        while is_identifier_part(self.current_char()) || self.current_char() == '\\' {
            if self.current_char() == '\\' {
                if self.peek_char() != 'u' {
                    return Err(ParserError::InvalidIdentifierCharacter);
                }

                let unicode_escape_sequence_u32 = self.read_unicode_escape_sequence();

                if unicode_escape_sequence_u32.is_err() {
                    return Err(unicode_escape_sequence_u32.unwrap_err());
                }
            }

            self.read_char();
        }

        Ok(())
    }

    // https://tc39.es/ecma262/#prod-ReservedWord
    // > Those that are contextually disallowed as identifiers, in strict mode code: let, static, implements, interface, package, private, protected, and public;
    fn match_reserved_keyword(&self, keyword_or_identifer: &str) -> Option<TokenKind> {
        match keyword_or_identifer {
            "await" => Some(TokenKind::Keyword(KeywordKind::Await)),
            "break" => Some(TokenKind::Keyword(KeywordKind::Break)),
            "case" => Some(TokenKind::Keyword(KeywordKind::Case)),
            "catch" => Some(TokenKind::Keyword(KeywordKind::Catch)),
            "class" => Some(TokenKind::Keyword(KeywordKind::Class)),
            "const" => Some(TokenKind::Keyword(KeywordKind::Const)),
            "continue" => Some(TokenKind::Keyword(KeywordKind::Continue)),
            "debugger" => Some(TokenKind::Keyword(KeywordKind::Debugger)),
            "default" => Some(TokenKind::Keyword(KeywordKind::Default)),
            "delete" => Some(TokenKind::Keyword(KeywordKind::Delete)),
            "do" => Some(TokenKind::Keyword(KeywordKind::Do)),
            "else" => Some(TokenKind::Keyword(KeywordKind::Else)),
            "enum" => Some(TokenKind::Keyword(KeywordKind::Enum)),
            "export" => Some(TokenKind::Keyword(KeywordKind::Export)),
            "extends" => Some(TokenKind::Keyword(KeywordKind::Extends)),
            "false" => Some(TokenKind::Keyword(KeywordKind::False)),
            "finally" => Some(TokenKind::Keyword(KeywordKind::Finally)),
            "for" => Some(TokenKind::Keyword(KeywordKind::For)),
            "function" => Some(TokenKind::Keyword(KeywordKind::Function)),
            "if" => Some(TokenKind::Keyword(KeywordKind::If)),
            "import" => Some(TokenKind::Keyword(KeywordKind::Import)),
            "in" => Some(TokenKind::Keyword(KeywordKind::In)),
            "instanceof" => Some(TokenKind::Keyword(KeywordKind::Instanceof)),
            "new" => Some(TokenKind::Keyword(KeywordKind::New)),
            "null" => Some(TokenKind::Keyword(KeywordKind::Null)),
            "return" => Some(TokenKind::Keyword(KeywordKind::Return)),
            "super" => Some(TokenKind::Keyword(KeywordKind::Super)),
            "switch" => Some(TokenKind::Keyword(KeywordKind::Switch)),
            "this" => Some(TokenKind::Keyword(KeywordKind::This)),
            "throw" => Some(TokenKind::Keyword(KeywordKind::Throw)),
            "true" => Some(TokenKind::Keyword(KeywordKind::True)),
            "try" => Some(TokenKind::Keyword(KeywordKind::Try)),
            "typeof" => Some(TokenKind::Keyword(KeywordKind::Typeof)),
            "var" => Some(TokenKind::Keyword(KeywordKind::Var)),
            "void" => Some(TokenKind::Keyword(KeywordKind::Void)),
            "while" => Some(TokenKind::Keyword(KeywordKind::While)),
            "with" => Some(TokenKind::Keyword(KeywordKind::With)),
            "yield" => Some(TokenKind::Keyword(KeywordKind::Yield)),

            // Strict mode future reserved words.
            "let" => Some(TokenKind::Keyword(KeywordKind::Let)),
            "static" => Some(TokenKind::Keyword(KeywordKind::Static)),
            "implements" => Some(TokenKind::Keyword(KeywordKind::Implements)),
            "interface" => Some(TokenKind::Keyword(KeywordKind::Interface)),
            "package" => Some(TokenKind::Keyword(KeywordKind::Package)),
            "private" => Some(TokenKind::Keyword(KeywordKind::Private)),
            "protected" => Some(TokenKind::Keyword(KeywordKind::Protected)),
            "public" => Some(TokenKind::Keyword(KeywordKind::Public)),

            // Appear as keywords within certain syntactic productions, at places where Identifier is not allowed.
            "as" => Some(TokenKind::Keyword(KeywordKind::As)),
            "async" => Some(TokenKind::Keyword(KeywordKind::Async)),
            "from" => Some(TokenKind::Keyword(KeywordKind::From)),
            "get" => Some(TokenKind::Keyword(KeywordKind::Get)),
            "of" => Some(TokenKind::Keyword(KeywordKind::Of)),
            "set" => Some(TokenKind::Keyword(KeywordKind::Set)),
            "target" => Some(TokenKind::Keyword(KeywordKind::Target)),
            _ => None,
        }
    }

    // https://tc39.es/ecma262/#prod-PrivateIdentifier
    pub(crate) fn scan_private_identifier(&mut self) -> Token {
        let start_index = self.read_index;

        self.read_char(); // Eat the '#' char.

        let identifier = self.read_identifier_start();

        let identifer_name = &self.source_str[start_index..self.read_index];

        match identifier {
            Ok(_) => Token::new(
                TokenKind::PrivateIdentifier,
                start_index,
                self.read_index,
                TokenValue::String(identifer_name.to_string()),
            ),
            Err(error) => Token::new(
                TokenKind::Illegal,
                start_index,
                self.read_index,
                TokenValue::String(error.to_string()),
            ),
        }
    }
}
