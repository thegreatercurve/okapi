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
    pub(crate) fn parse_generator_expression(&mut self) -> Result<Expression, ParserError> {
        self.expect_and_advance(TokenKind::Keyword(KeywordKind::Function))?;

        self.expect_and_advance(TokenKind::Multiplication)?;

        let _binding_identifier = self.parse_binding_identifier()?;

        //  TODO Handle function expression generators.

        self.expect_and_advance(TokenKind::LeftParenthesis)?;

        let _parameterss = self.parse_formal_parameters()?;

        self.expect_and_advance(TokenKind::RightParenthesis)?;

        let _body = self.parse_generator_body()?;

        todo!("parse_generator_expression")

        // TODO: Implement the following code once the types are better.
        // Ok(GeneratorDeclarationOrExpression::Declaration(
        //     Declaration::Function(FunctionDeclaration {
        //         node: self.end_node()?,
        //         id: Some(identifier),
        //         params: parameters,
        //         body,
        //         generator: true,
        //         asynchronous: false,
        //         expression: false,
        //     }),
        // ))
    }

    // https://tc39.es/ecma262/#prod-GeneratorMethod
    fn parse_generator_method(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_generator_method")
    }

    // https://tc39.es/ecma262/#prod-GeneratorBody
    fn parse_generator_body(&mut self) -> Result<BlockStatement, ParserError> {
        todo!("parse_generator_declaration")
    }

    // https://tc39.es/ecma262/#prod-YieldExpression
    pub(crate) fn parse_yield_expression(&mut self) -> Result<Expression, ParserError> {
        todo!("parse_yield_expression")
    }
}
