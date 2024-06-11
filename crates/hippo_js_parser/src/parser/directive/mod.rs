use crate::{
    ast::{
        DirectiveStatement, Expression, ExpressionStatement, Literal, LiteralValue, Statement,
        StatementListItem,
    },
    Parser, ParserError, TokenKind, TokenValue,
};

// 11 ECMAScript Language: Source Text
// https://tc39.es/ecma262/#sec-ecmascript-language-source-code
impl Parser {
    // 11.2.1 Directive Prologues and the Use Strict Directive
    // https://tc39.es/ecma262/#sec-directive-prologues-and-the-use-strict-directive
    pub(crate) fn parse_directive_prologue(
        &mut self,
    ) -> Result<Vec<StatementListItem>, ParserError> {
        let mut directives = vec![];

        while self.token_kind() == TokenKind::StringLiteral {
            let directive = self.parse_directive()?;

            directives.push(directive);
        }

        Ok(directives)
    }

    fn parse_directive(&mut self) -> Result<StatementListItem, ParserError> {
        let start_index = self.start_node();

        let expression = self.parse_expression()?;

        self.expect_optional_semicolon_and_advance();

        if let Expression::Literal(Literal {
            value: LiteralValue::String(value),
            ..
        }) = expression.clone()
        {
            // TODO Check if token has does not contain an escape sequence.
            if self.has_previous_token_line_terminator() || self.token_kind() == TokenKind::EOF {
                if value == "use strict" {
                    self.context.strict_mode = true;
                }

                return Ok(StatementListItem::Statement(Statement::Directive(
                    DirectiveStatement {
                        node: self.end_node(start_index)?,
                        expression,
                        directive: value,
                    },
                )));
            }
        };

        Ok(StatementListItem::Statement(Statement::Expression(
            ExpressionStatement {
                node: self.end_node(start_index)?,
                expression: expression.clone(),
            },
        )))
    }
}
