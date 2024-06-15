mod exports;
mod imports;
mod module;
mod script;

use crate::ast::*;
use crate::{Parser, ParserError};

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl Parser {
    // 16.1 Scripts
    // https://tc39.es/ecma262/#prod-Script
    pub fn parse_script(&mut self) -> Result<Program, ParserError> {
        let program_body = self.parse_script_body()?;

        Ok(Program {
            body: program_body,
            source_type: ProgramSource::Script,
            node: Node::new(0, self.cursor.lexer.len()),
        })
    }

    // 16.2 Modules
    // https://tc39.es/ecma262/#prod-Module
    pub fn parse_module(&mut self) -> Result<Program, ParserError> {
        // Module code is always strict mode code.
        // https://tc39.es/ecma262/#sec-strict-mode-code
        self.context.strict_mode = true;

        let program_body = self.parse_module_body()?;

        Ok(Program {
            body: program_body,
            source_type: ProgramSource::Module,
            node: Node::new(0, self.cursor.lexer.len()),
        })
    }
}
