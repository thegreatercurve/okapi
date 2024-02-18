use crate::{
    BindingPattern, ClassDeclaration, Expression, FunctionDeclaration, Identifier, Literal, Node,
    StatementListItem, VariableDeclaration,
};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct Program {
    #[serde(flatten)]
    pub node: Node,
    pub body: ProgramBody,
    #[serde(rename = "sourceType")]
    pub source_type: ProgramSourceTypes,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ProgramBody {
    StatementList(Vec<StatementListItem>),
    Module(Vec<ModuleItem>),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ProgramSourceTypes {
    Script,
    Module,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ModuleItem {
    ImportDeclaration(ImportDeclaration),
    ExportDeclaration(ExportDeclaration),
    StatementListItem(StatementListItem),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub specifiers: Vec<ImportSpecifier>,
    pub source: Literal,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct ImportSpecifier {
    #[serde(rename = "type")]
    pub kind: ImportSpecifierKind,
    #[serde(flatten)]
    pub node: Node,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported: Option<ImportSpecifierImported>,
    pub local: Identifier,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ImportSpecifierKind {
    ImportSpecifier,
    ImportDefaultSpecifier,
    ImportNamespaceSpecifier,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ImportSpecifierImported {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportDeclaration {
    ExportAllDeclaration(ExportAllDeclaration),
    ExportDefaultDeclaration(ExportDefaultDeclaration),
    ExportNamedDeclaration(ExportNamedDeclaration),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportAllDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub exported: Option<ExportAllDeclarationExported>,
    pub source: Literal,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportAllDeclarationExported {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportDefaultDeclaration {
    #[serde(flatten)]
    pub node: Node,
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
pub struct ExportNamedDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declaration: Option<ExportNamedDeclarationDeclaration>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<Literal>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportNamedDeclarationDeclaration {
    Class(ClassDeclaration),
    Function(FunctionDeclaration),
    Variable(VariableDeclaration),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportSpecifier {
    #[serde(flatten)]
    pub node: Node,
    pub local: Box<ExportSpecifierLocal>,
    pub exported: Box<ExportSpecifierExported>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportSpecifierExported {
    Identifier(Identifier),
    Literal(Literal),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ExportSpecifierLocal {
    Identifier(Identifier),
    Literal(Literal),
}
