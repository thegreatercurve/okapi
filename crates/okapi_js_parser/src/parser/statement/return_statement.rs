use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.10 The return Statement
    // https://tc39.es/ecma262/#prod-ReturnStatement
    pub(crate) fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Return))?;

        let argument = if self.token_kind() == TokenKind::Semicolon {
            None
        } else {
            let expression = self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_expression,
            )?;

            Some(expression)
        };

        self.expect_optional_semicolon_and_advance();

        Ok(ReturnStatement {
            node: self.end_node(start_index)?,
            argument,
        })
    }
}
