use crate::{Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // 14.2 Block
    // https://tc39.es/ecma262/#prod-BlockStatement
    pub(crate) fn parse_block_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let statement_list = self.parse_statement_list()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Statement::Block(BlockStatement {
            node: self.end_node()?,
            body: statement_list,
        }))
    }

    // https://tc39.es/ecma262/#prod-StatementList
    pub(crate) fn parse_statement_list(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut body = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightCurlyBrace {
            body.push(self.parse_statement()?);
        }

        Ok(body)
    }

    // https://tc39.es/ecma262/#prod-StatementListItem
    pub(crate) fn parse_statement_list_item(&mut self) -> Result<Statement, ParserError> {
        match self.cursor.current_token_kind() {
            current_token_kind if current_token_kind.is_declaration_start() => {
                Ok(Statement::Declaration(self.parse_declaration()?))
            }
            _ => Ok(self.parse_statement()?),
        }
    }
}
