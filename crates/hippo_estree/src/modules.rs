use crate::{
    ClassDeclaration, Declaration, Expression, FunctionDeclaration, Identifier, Literal, Node,
};
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportOrExportDeclaration {}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ModuleSpecifier {
    pub local: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub specifiers: Vec<ImportSpecifiers>,
    pub source: Literal,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ImportSpecifiers {
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
    pub imported: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportDefaultSpecifier {
    #[serde(flatten)]
    pub node: Node,
    pub local: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportNamespaceSpecifier {
    #[serde(flatten)]
    pub node: Node,
    pub local: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportNamedDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declaration: Option<Declaration>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<Literal>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportSpecifier {
    #[serde(flatten)]
    pub node: Node,
    pub local: Identifier,
    pub exported: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AnonymousDefaultExportedFunctionDeclaration {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AnonymousDefaultExportedClassDeclaration {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportDefaultDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declaration: ExportDefaultDeclarationDeclaration,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ExportDefaultDeclarationDeclaration {
    AnonymousDefaultExportedFunction(AnonymousDefaultExportedFunctionDeclaration),
    Function(FunctionDeclaration),
    AnonymousDefaultExportedClass(AnonymousDefaultExportedClassDeclaration),
    Class(ClassDeclaration),
    Expression(Box<Expression>),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportAllDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub source: Literal,
}
