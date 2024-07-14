use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.2 Block
    // https://tc39.es/ecma262/#prod-BlockStatement
    pub(crate) fn parse_block_statement(&mut self) -> Result<BlockStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let statement_list = self.parse_statement_list()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(BlockStatement {
            node: self.end_node(start_index)?,
            body: statement_list,
        })
    }

    // https://tc39.es/ecma262/#prod-StatementList
    pub(crate) fn parse_statement_list(&mut self) -> Result<Vec<StatementListItem>, ParserError> {
        let mut body = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            body.push(StatementListItem::Statement(self.parse_statement()?));
        }

        Ok(body)
    }

    // https://tc39.es/ecma262/#prod-StatementListItem
    pub(crate) fn parse_statement_list_item(&mut self) -> Result<StatementListItem, ParserError> {
        match self.token_kind() {
            TokenKind::Keyword(KeywordKind::Let) => {
                if self.peek_token_kind().is_lexical_declaration_start() {
                    Ok(StatementListItem::Declaration(self.parse_declaration()?))
                } else {
                    Ok(StatementListItem::Statement(self.parse_statement()?))
                }
            }
            token_kind if token_kind.is_declaration_start() => {
                Ok(StatementListItem::Declaration(self.parse_declaration()?))
            }
            _ => Ok(StatementListItem::Statement(self.parse_statement()?)),
        }
    }
}
