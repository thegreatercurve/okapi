use crate::{KeywordKind, Parser, ParserError, TokenKind};
use hippo_estree::*;

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl<'a> Parser<'a> {
    // 16.2 Module
    // https://tc39.es/ecma262/#prod-ModuleBody
    pub(crate) fn parse_module_body(&mut self) -> Result<Vec<Statement>, ParserError> {
        // TODO Parse parser statement of declaration.
        let mut body = vec![];

        while self.cursor.current_token_kind() != TokenKind::EOF {
            let module_item = self.parse_module_item()?;

            body.push(module_item);
        }

        Ok(body)
    }

    // https://tc39.es/ecma262/#prod-ModuleItem
    fn parse_module_item(&mut self) -> Result<Statement, ParserError> {
        let module_item = match self.cursor.current_token_kind() {
            TokenKind::Keyword(KeywordKind::Import) => {
                Statement::Declaration(Declaration::Import(self.parse_import_declaration()?))
            }
            TokenKind::Keyword(KeywordKind::Export) => {
                Statement::Declaration(Declaration::Export(self.parse_export_declaration()?))
            }
            _ => Declaration::StatementListItem(self.parse_statement_list_item()?),
        };

        Ok(module_item)
    }
}
