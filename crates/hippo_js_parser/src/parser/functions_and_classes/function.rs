use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

enum GeneratorDeclarationOrExpression {
    Declaration(Declaration),
    Expression(Expression),
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 15.2 Function Definitions
    // https://tc39.es/ecma262/#prod-FunctionDeclaration
    pub(crate) fn parse_function_declaration(
        &mut self,
    ) -> Result<FunctionDeclaration, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        let identifier = if self.cursor.current_token_kind() == TokenKind::Identifier {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        // TODO Figure out a better way of handling these typings.
        let formal_parameters = self
            .parse_formal_parameters()?
            .into_iter()
            .map(|parameter| parameter)
            .collect();

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let body = self.parse_function_body()?;

        Ok(FunctionDeclaration {
            node: self.end_node()?,
            id: identifier,
            params: formal_parameters,
            body,
            generator: false,
            is_async: false,
            expression: false,
        })
    }

    pub(crate) fn parse_function_expression(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        if self.cursor.peek_token_kind() == TokenKind::Identifier {
            let _identifier = self.parse_binding_identifier()?;
        }

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        // TODO Figure out a better way of handling these typings.
        let formal_parameters = self
            .parse_formal_parameters()?
            .into_iter()
            .map(|parameter| parameter)
            .collect();

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let body = self.parse_function_body()?;

        Ok(Expression::Function(FunctionExpression {
            node: self.end_node()?,
            id: None,
            body,
            params: formal_parameters,
            expression: false,
            generator: false,
            is_async: false,
        }))
    }

    // https://tc39.es/ecma262/#prod-FunctionBody
    pub(crate) fn parse_function_body(&mut self) -> Result<BlockStatement, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let mut body = vec![];

        while self.cursor.current_token_kind() != TokenKind::RightCurlyBrace {
            body.push(self.parse_statement_list_item()?);
        }

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(BlockStatement {
            node: self.end_node()?,
            body,
        })
    }
}
