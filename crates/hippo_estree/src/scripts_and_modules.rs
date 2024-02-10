use crate::{
    BindingPattern, ClassDeclaration, Expression, FunctionDeclaration, Identifier, Literal, Node,
    StatementListItem, VariableDeclaration,
};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub struct Program {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<ProgramBody>,
    pub source_type: ProgramSourceTypes,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ProgramBody {
    StatementListItem(StatementListItem),
    Module(ModuleItem),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ProgramSourceTypes {
    Script,
    Module,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ModuleItem {
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    StatementListItem(StatementListItem),
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ImportDeclaration {
    #[serde(rename = "type")]
    pub kind: ImportDeclarationKind,
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<ImportSpecifier>,
    pub source: Literal,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ImportDeclarationKind {
    ImportSpecifier,
    ImportDefaultSpecifier,
    ImportNamespaceSpecifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportSpecifier {
    #[serde(flatten)]
    pub node: Node,
    pub local: Identifier,
    pub imported: Option<Identifier>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExportDeclaration {
    ExportAllDeclaration,
    ExportDefaultDeclaration,
    ExportNamedDeclaration,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
struct ExportAllDeclaration {
    pub source: Literal,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
struct ExportDefaultDeclaration {
    pub declaration: ExportDefaultDeclarationDeclaration,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExportDefaultDeclarationDeclaration {
    Identifier(Identifier),
    BindingPattern(BindingPattern),
    ClassDeclaration(ClassDeclaration),
    Expression(Expression),
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
struct ExportNamedDeclaration {
    pub declaration: ExportNamedDeclarationDeclaration,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Literal,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExportNamedDeclarationDeclaration {
    ClassDeclaration(ClassDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    VariableDeclaration(VariableDeclaration),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
struct ExportSpecifier {
    pub exported: Identifier,
    pub local: Identifier,
}
