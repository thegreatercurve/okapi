use crate::{
    ExportAllDeclarationExported, ExportSpecifierExported, ExportSpecifierLocal, Identifier,
    ImportSpecifierImported, Literal,
};
use serde::Serialize;

// https://tc39.es/ecma262/#prod-ModuleExportName
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ModuleExportName {
    Identifier(Identifier),
    Literal(Literal),
}

impl ModuleExportName {
    pub fn to_import_specifier_imported(self) -> ImportSpecifierImported {
        match self {
            ModuleExportName::Identifier(identifier) => {
                ImportSpecifierImported::Identifier(identifier)
            }
            ModuleExportName::Literal(literal) => ImportSpecifierImported::Literal(literal),
        }
    }

    pub fn to_export_all_declaration_exported(self) -> ExportAllDeclarationExported {
        match self {
            ModuleExportName::Identifier(identifier) => {
                ExportAllDeclarationExported::Identifier(identifier)
            }
            ModuleExportName::Literal(literal) => ExportAllDeclarationExported::Literal(literal),
        }
    }

    pub fn to_export_specifier_local(self) -> ExportSpecifierLocal {
        match self {
            ModuleExportName::Identifier(identifier) => {
                ExportSpecifierLocal::Identifier(identifier)
            }
            ModuleExportName::Literal(literal) => ExportSpecifierLocal::Literal(literal),
        }
    }

    pub fn to_export_specifier_exported(self) -> ExportSpecifierExported {
        match self {
            ModuleExportName::Identifier(identifier) => {
                ExportSpecifierExported::Identifier(identifier)
            }
            ModuleExportName::Literal(literal) => ExportSpecifierExported::Literal(literal),
        }
    }
}
