mod block;
mod empty;
mod if_statement;
mod iteration;

use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // https://tc39.es/ecma262/#prod-Statement
    pub(crate) fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cursor.current_token_kind() {
            current_token_kind if current_token_kind.is_lexical_declaration_keyword() => Ok(
                Statement::Declaration(Declaration::Variable(self.parse_lexical_declaration()?)),
            ),
            TokenKind::Keyword(keyword) => match keyword {
                KeywordKind::If => self.parse_if_statement(),
                KeywordKind::For => self.parse_for_or_for_of_in_statement(),
                KeywordKind::Do => self.parse_do_while_statement(),
                KeywordKind::While => self.parse_while_statement(),
                KeywordKind::Continue => self.parse_continue_statement(),
                KeywordKind::Break => self.parse_break_statement(),
                KeywordKind::Return => self.parse_return_statement(),
                KeywordKind::With => todo!(),
                KeywordKind::Throw => self.parse_throw_statement(),
                KeywordKind::Try => self.parse_try_statement(),
                KeywordKind::Switch => self.parse_switch_statement(),
                KeywordKind::Debugger => self.parse_debugger_statement(),
                _ => self.parse_expression_statement_or_labelled_statement(),
            },
            TokenKind::LeftCurlyBrace => self.parse_block_statement(),
            TokenKind::Semicolon => self.parse_empty_statement(),
            _ => self.parse_expression_statement_or_labelled_statement(),
        }
    }

    // https://tc39.es/ecma262/#prod-ExpressionStatement
    // https://tc39.es/ecma262/#prod-LabelledStatement
    fn parse_expression_statement_or_labelled_statement(
        &mut self,
    ) -> Result<Statement, ParserError> {
        self.start_node();

        let expression = self.parse_expression()?;

        if self.cursor.current_token_kind() == TokenKind::Colon {
            self.end_node()?;

            todo!("parse_labelled_statement")
        } else {
            self.expect_optional_semicolon_and_advance();

            Ok(Statement::Expression(ExpressionStatement {
                node: self.end_node()?,
                expression: expression,
                directive: None,
            }))
        }
    }

    // https://tc39.es/ecma262/#prod-ContinueStatement
    fn parse_continue_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Continue))?;

        let label = if self.cursor.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_label_identifier()?)
        };

        Ok(Statement::Continue(ContinueStatement {
            node: self.end_node()?,
            label,
        }))
    }

    // https://tc39.es/ecma262/#prod-BreakStatement
    fn parse_break_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Break))?;

        let label = if self.cursor.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_label_identifier()?)
        };

        Ok(Statement::Break(BreakStatement {
            node: self.end_node()?,
            label,
        }))
    }

    // https://tc39.es/ecma262/#prod-ReturnStatement
    fn parse_return_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Return))?;

        let argument = if self.cursor.current_token_kind() == TokenKind::Semicolon {
            None
        } else {
            Some(self.parse_expression()?)
        };

        Ok(Statement::Return(ReturnStatement {
            node: self.end_node()?,
            argument,
        }))
    }

    // https://tc39.es/ecma262/#prod-ThrowStatement
    fn parse_throw_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Throw))?;

        let argument = self.parse_expression()?;

        Ok(Statement::Throw(ThrowStatement {
            node: self.end_node()?,
            argument,
        }))
    }

    // https://tc39.es/ecma262/#prod-TryStatement
    fn parse_try_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    // https://tc39.es/ecma262/#prod-SwitchStatement
    fn parse_switch_statement(&mut self) -> Result<Statement, ParserError> {
        todo!()
    }

    // https://tc39.github.io/ecma262/#sec-debugger-statement
    fn parse_debugger_statement(&mut self) -> Result<Statement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Debugger))?;

        self.expect_and_advance(TokenKind::Semicolon)?;

        Ok(Statement::Debugger(DebuggerStatement {
            node: self.end_node()?,
        }))
    }
}
