mod block;
mod break_statement;
mod continue_statement;
mod debugger;
mod declarations_and_variable;
mod empty;
mod if_statement;
mod iteration;
mod return_statement;
mod switch;
mod throw;
mod try_statement;

use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl<'a> Parser<'a> {
    // https://tc39.es/ecma262/#prod-Statement
    pub(crate) fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.cursor.current_token_kind() {
            current_token_kind if current_token_kind.is_lexical_declaration_start() => Ok(
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
                KeywordKind::With => todo!("parse_with_statement"),
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

    // https://tc39.es/ecma262/#prod-Declaration
    fn parse_declaration(&mut self) -> Result<Declaration, ParserError> {
        match self.cursor.current_token_kind() {
            current_token_kind if current_token_kind.is_hoistable_declaration_start() => {
                self.parse_hoistable_declaration()
            }
            TokenKind::Keyword(KeywordKind::Class) => {
                Ok(Declaration::Class(self.parse_class_declaration()?))
            }
            current_token_kind if current_token_kind.is_lexical_declaration_start() => {
                Ok(Declaration::Variable(self.parse_lexical_declaration()?))
            }

            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-HoistableDeclaration
    fn parse_hoistable_declaration(&mut self) -> Result<Declaration, ParserError> {
        match self.cursor.current_token_kind() {
            TokenKind::Keyword(KeywordKind::Function) => match self.cursor.peek_token_kind() {
                TokenKind::Multiplication => {
                    Ok(Declaration::Function(self.parse_generator_declaration()?))
                }
                _ => Ok(Declaration::Function(self.parse_function_declaration()?)),
            },
            TokenKind::Keyword(KeywordKind::Async) => match self.cursor.peek_token_kind() {
                TokenKind::Keyword(KeywordKind::Function) => match self.cursor.peek_token_kind() {
                    TokenKind::Multiplication => Ok(Declaration::Function(
                        self.parse_async_generator_declaration()?,
                    )),
                    _ => Ok(Declaration::Function(
                        self.parse_async_function_declaration()?,
                    )),
                },
                _ => Err(self.unexpected_current_token_kind()),
            },
            _ => Err(self.unexpected_current_token_kind()),
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
}
