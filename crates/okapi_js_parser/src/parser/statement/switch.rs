use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.14 The switch Statement
    // https://tc39.es/ecma262/#prod-SwitchStatement
    pub(crate) fn parse_switch_statement(&mut self) -> Result<SwitchStatement, ParserError> {
        let start_index = self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Switch))?;

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let expression = self.with_params(
            self.params.clone().add_allow_in(false),
            Self::parse_expression,
        )?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let case_block = self.parse_case_block()?;

        Ok(SwitchStatement {
            node: self.end_node(start_index)?,
            discriminant: expression,
            cases: case_block,
        })
    }

    // https://tc39.es/ecma262/#prod-CaseBlock
    fn parse_case_block(&mut self) -> Result<Vec<SwitchCase>, ParserError> {
        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let mut cases = vec![];

        while self.token_kind() != TokenKind::RightCurlyBrace {
            cases.push(self.parse_case_or_default_clause()?);
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(cases)
    }

    // https://tc39.es/ecma262/#prod-CaseClause
    // https://tc39.es/ecma262/#prod-DefaultClause
    fn parse_case_or_default_clause(&mut self) -> Result<SwitchCase, ParserError> {
        let start_index = self.start_node();

        let case_clause_expression = match self.token_kind() {
            TokenKind::Keyword(KeywordKind::Case) => {
                self.advance_any(); // Eat 'case' token.

                let expression = self.with_params(
                    self.params.clone().add_allow_in(false),
                    Self::parse_expression,
                )?;

                Some(expression)
            }
            TokenKind::Keyword(KeywordKind::Default) => {
                self.advance_any(); // Eat 'default' token.

                None
            }
            _ => None,
        };

        self.expect_and_advance(TokenKind::Colon)?;

        let mut statement_list = vec![];

        while self.token_kind() != TokenKind::Keyword(KeywordKind::Case)
            && self.token_kind() != TokenKind::Keyword(KeywordKind::Default)
            && self.token_kind() != TokenKind::RightCurlyBrace
        {
            statement_list.push(self.parse_statement()?);
        }

        Ok(SwitchCase {
            node: self.end_node(start_index)?,
            test: case_clause_expression,
            consequent: statement_list,
        })
    }
}
