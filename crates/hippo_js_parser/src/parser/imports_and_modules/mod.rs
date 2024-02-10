mod module;
mod script;

use crate::{Parser, ParserError};
use hippo_estree::*;

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl<'a> Parser<'a> {
    // 16.1 Scripts
    // https://tc39.es/ecma262/#prod-Script
    pub(crate) fn parse_script(&mut self) -> Result<Program, ParserError> {
        // TODO Parse parser statement of declaration.
        self.start_node();

        let program_body = self.parse_statement().unwrap();

        Ok(Program {
            body: vec![StatementListItem::Statement(program_body)],
            source_type: ProgramSourceTypes::Script,
            node: Node::new(0, self.cursor.lexer.len()),
        })
    }

    // 16.2 Modules
    // https://tc39.es/ecma262/#prod-Module
    pub(crate) fn parse_module(&mut self) -> Result<Program, ParserError> {
        // TODO Parse parser statement of declaration.
        let program_body = self.parse_statement()?;

        let _body = self.parse_module_body();

        Ok(Program {
            body: vec![StatementListItem::Statement(program_body)],
            source_type: ProgramSourceTypes::Module,
            node: Node::new(0, self.cursor.lexer.len()),
        })
    }
}
