use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

enum GeneratorDeclarationOrExpression {
    Declaration(Declaration),
    Expression(Expression),
}

// 13 ECMAScript Language: Expressions
// https://tc39.es/ecma262/#sec-ecmascript-language-expressions
impl<'a> Parser<'a> {
    // 15.5 Generator Function Definitions
    // https://tc39.es/ecma262/#prod-GeneratorExpression
    pub(crate) fn parse_generator_expression(
        &mut self,
    ) -> Result<GeneratorDeclarationOrExpression, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        self.expect_and_advance(TokenKind::Multiplication)?;

        let binding_identifier = self.parse_binding_identifier()?;

        // Handle function expression generators.
        let identifier = match binding_identifier {
            VariableDeclaratorBindingKind::Identifier(identifier) => identifier,
            _ => return Err(self.unexpected_current_token_kind()),
        };

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let parameters = self.parse_formal_parameters()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let body = self.parse_generator_body()?;

        Ok(GeneratorDeclarationOrExpression::Declaration(
            Declaration::Function(FunctionDeclaration {
                node: self.end_node()?,
                id: todo!(),
                params: todo!(),
                body: todo!(),
                generate: todo!(),
                asynchronous: todo!(),
                expression: todo!(),
            }),
        ))
    }

    // https://tc39.es/ecma262/#prod-GeneratorMethod
    fn parse_generator_method(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_generator_method")
    }

    // https://tc39.es/ecma262/#prod-GeneratorBody
    fn parse_generator_body(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_generator_declaration")
    }

    // https://tc39.es/ecma262/#prod-YieldExpression
    pub(crate) fn parse_yield_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_yield_expression")
    }
}
