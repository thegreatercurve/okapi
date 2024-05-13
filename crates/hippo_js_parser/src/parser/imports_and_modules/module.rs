use crate::ast::*;
use crate::{KeywordKind, Parser, ParserError, TokenKind};

// 16 ECMAScript Language: Scripts and Modules
// https://tc39.es/ecma262/#sec-ecmascript-language-scripts-and-modules
impl Parser {
    // 16.2 Module
    // https://tc39.es/ecma262/#prod-ModuleBody
    pub(crate) fn parse_module_body(&mut self) -> Result<ProgramBody, ParserError> {
        let mut body = vec![];

        while self.token_kind() != TokenKind::EOF {
            let module_item = self.parse_module_item()?;

            body.push(module_item);
        }

        Ok(ProgramBody::Module(body))
    }

    // https://tc39.es/ecma262/#prod-ModuleItem
    fn parse_module_item(&mut self) -> Result<ModuleItem, ParserError> {
        let module_item = match self.token_kind() {
            TokenKind::Keyword(KeywordKind::Import) => {
                ModuleItem::ImportDeclaration(self.parse_import_declaration()?)
            }
            TokenKind::Keyword(KeywordKind::Export) => {
                ModuleItem::ExportDeclaration(self.parse_export_declaration()?)
            }
            _ => ModuleItem::StatementListItem(self.parse_statement_list_item()?),
        };

        Ok(module_item)
    }
}
