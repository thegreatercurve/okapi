use crate::{ast::*, TokenKind};
use crate::{Parser, ParserError};

// 14 ECMAScript Language: Statements and Declarations
// https://tc39.es/ecma262/#prod-Statement
impl Parser {
    // 14.13 Labelled Statements
    // https://tc39.es/ecma262/#prod-LabelledStatement
    pub(crate) fn parse_labeled_statement(&mut self) -> Result<LabeledStatement, ParserError> {
        let start_index = self.start_node();

        let label_identifier = self.parse_label_identifier()?;

        self.expect_and_advance(TokenKind::Colon)?;

        let statement = self.parse_statement()?;

        Ok(LabeledStatement {
            node: self.end_node(start_index)?,
            label: label_identifier,
            body: Box::new(statement),
        })
    }
}
