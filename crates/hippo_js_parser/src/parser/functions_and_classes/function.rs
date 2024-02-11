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
    // https://tc39.es/ecma262/#prod-FunctionExpression
    pub(crate) fn parse_function_declaration(
        &mut self,
    ) -> Result<FunctionDeclaration, ParserError> {
        todo!("parse_function_declaration")
    }

    pub(crate) fn parse_function_expression(&mut self) -> Result<Expression, ParserError> {
        self.start_node();

        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        if self.cursor.peek_token_kind() == TokenKind::Identifier {
            let _identifier = self.parse_binding_identifier()?;
        }

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let _formal_paramaters = self.parse_formal_parameters()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        self.expect_and_advance(TokenKind::LeftCurlyBrace)?;

        let body = self.parse_function_body()?;

        self.expect_and_advance(TokenKind::RightCurlyBrace)?;

        Ok(Expression::Function(FunctionExpression {
            node: self.end_node()?,
            body: Box::new(body),
        }))
    }
}
