mod block;
mod break_statement;
mod continue_statement;
mod debugger;
mod declarations_and_variable;
mod empty;
mod if_statement;
mod iteration;
mod labelled;
mod return_statement;
mod switch;
mod throw;
mod try_statement;
mod with_statement;

use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

use self::iteration::ForStatementKind;

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // https://tc39.es/ecma262/#prod-Statement
    pub(crate) fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.token_kind() {
            TokenKind::Keyword(keyword) => match keyword {
                KeywordKind::Let | KeywordKind::Const => Ok(Statement::Declaration(
                    Declaration::Variable(self.parse_lexical_declaration(true)?),
                )),
                KeywordKind::Var => Ok(Statement::Declaration(Declaration::Variable(
                    self.parse_variable_statement(true)?,
                ))),
                KeywordKind::If => Ok(Statement::If(self.parse_if_statement()?)),
                KeywordKind::For => match self.parse_for_or_for_of_in_statement()? {
                    ForStatementKind::Classic(for_statement) => Ok(Statement::For(for_statement)),
                    ForStatementKind::In(for_in_statement) => {
                        Ok(Statement::ForIn(for_in_statement))
                    }
                    ForStatementKind::Of(for_of_statement) => {
                        Ok(Statement::ForOf(for_of_statement))
                    }
                },
                KeywordKind::Do => Ok(Statement::DoWhile(self.parse_do_while_statement()?)),
                KeywordKind::While => Ok(Statement::While(self.parse_while_statement()?)),
                KeywordKind::Continue => Ok(Statement::Continue(self.parse_continue_statement()?)),
                KeywordKind::Break => Ok(Statement::Break(self.parse_break_statement()?)),
                KeywordKind::Return => Ok(Statement::Return(self.parse_return_statement()?)),
                KeywordKind::With => Ok(Statement::With(self.parse_with_statement()?)),
                KeywordKind::Throw => Ok(Statement::Throw(self.parse_throw_statement()?)),
                KeywordKind::Try => Ok(Statement::Try(self.parse_try_statement()?)),
                KeywordKind::Switch => Ok(Statement::Switch(self.parse_switch_statement()?)),
                KeywordKind::Debugger => Ok(Statement::Debugger(self.parse_debugger_statement()?)),
                _ => self.parse_expression_statement_or_labelled_statement(),
            },
            TokenKind::LeftCurlyBrace => Ok(Statement::Block(self.parse_block_statement()?)),
            TokenKind::Semicolon => self.parse_empty_statement(),
            _ => self.parse_expression_statement_or_labelled_statement(),
        }
    }

    // https://tc39.es/ecma262/#prod-Declaration
    fn parse_declaration(&mut self) -> Result<Declaration, ParserError> {
        match self.token_kind() {
            token_kind if token_kind.is_hoistable_declaration_start() => {
                Ok(Declaration::Function(self.parse_hoistable_declaration()?))
            }
            TokenKind::Keyword(KeywordKind::Class) => {
                Ok(Declaration::Class(self.parse_class_declaration()?))
            }
            token_kind if token_kind.is_lexical_declaration_start() => {
                let lexical_declaration = self
                    .with_params(self.params.clone().add_allow_in(false), |slf| {
                        slf.parse_lexical_declaration(true)
                    })?;

                Ok(Declaration::Variable(lexical_declaration))
            }
            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-HoistableDeclaration
    pub(crate) fn parse_hoistable_declaration(
        &mut self,
    ) -> Result<FunctionDeclaration, ParserError> {
        // An ExpressionStatement cannot start with '{', 'function', 'async [no LineTerminator here] function', 'class', or 'let ['
        match (self.token_kind(), self.peek_token_kind()) {
            (TokenKind::Keyword(KeywordKind::Function), TokenKind::Multiplication) => {
                self.parse_generator_declaration()
            }
            (TokenKind::Keyword(KeywordKind::Function), _) => self.parse_function_declaration(),
            (TokenKind::Keyword(KeywordKind::Async), TokenKind::Keyword(KeywordKind::Function)) => {
                match self.peek_nth_kind(2) {
                    TokenKind::Multiplication => self.parse_async_generator_declaration(),
                    _ => self.parse_async_function_declaration(),
                }
            }

            _ => Err(self.unexpected_current_token_kind()),
        }
    }

    // https://tc39.es/ecma262/#prod-ExpressionStatement
    // https://tc39.es/ecma262/#prod-LabelledStatement
    fn parse_expression_statement_or_labelled_statement(
        &mut self,
    ) -> Result<Statement, ParserError> {
        // An ExpressionStatement cannot start with '{', 'function', 'async [no LineTerminator here] function', 'class', or 'let ['
        match (self.token_kind(), self.peek_token_kind()) {
            (TokenKind::LeftCurlyBrace, _) => {
                return Ok(Statement::Block(self.parse_block_statement()?))
            }
            (TokenKind::Keyword(KeywordKind::Function), _) => {
                return Ok(Statement::Declaration(Declaration::Function(
                    self.parse_function_declaration()?,
                )))
            }
            (TokenKind::Keyword(KeywordKind::Async), TokenKind::Keyword(KeywordKind::Function)) => {
                return match self.peek_nth_kind(2) {
                    TokenKind::Multiplication => Ok(Statement::Declaration(Declaration::Function(
                        self.parse_async_generator_declaration()?,
                    ))),
                    _ => Ok(Statement::Declaration(Declaration::Function(
                        self.parse_async_function_declaration()?,
                    ))),
                }
            }
            (TokenKind::Keyword(KeywordKind::Class), _) => {
                return Ok(Statement::Declaration(Declaration::Class(
                    self.parse_class_declaration()?,
                )))
            }
            (TokenKind::Keyword(KeywordKind::Let), TokenKind::LeftSquareBracket) => {
                let lexical_declaration = self.parse_lexical_declaration(true)?;

                return Ok(Statement::Declaration(Declaration::Variable(
                    lexical_declaration,
                )));
            }
            _ => {}
        };

        if self.peek_token_kind() == TokenKind::Colon {
            Ok(Statement::Labeled(self.parse_labeled_statement()?))
        } else {
            let start_index = self.start_node();

            let expression = self.with_params(
                self.params.clone().add_allow_in(false),
                Self::parse_expression,
            )?;

            self.expect_optional_semicolon_and_advance();

            Ok(Statement::Expression(ExpressionStatement {
                node: self.end_node(start_index)?,
                expression: expression.clone(),
            }))
        }
    }
}
