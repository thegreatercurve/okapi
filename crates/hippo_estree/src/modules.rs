use crate::{BaseNode, ClassDeclaration, ExpressionData, FunctionDeclaration, Identifier, Literal};
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
    pub node: BaseNode,
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
    pub node: BaseNode,
    pub local: Identifier,
    pub imported: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportDefaultSpecifier {
    #[serde(flatten)]
    pub node: BaseNode,
    pub local: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ImportNamespaceSpecifier {
    #[serde(flatten)]
    pub node: BaseNode,
    pub local: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportNamedDeclaration {
    #[serde(flatten)]
    pub node: BaseNode,
    pub declaration: Option<Declaration>,
    pub specifiers: Vec<ExportSpecifier>,
    pub source: Option<Literal>,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportSpecifier {
    #[serde(flatten)]
    pub node: BaseNode,
    pub local: Identifier,
    pub exported: Identifier,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AnonymousDefaultExportedFunctionDeclaration {
    #[serde(flatten)]
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AnonymousDefaultExportedClassDeclaration {
    #[serde(flatten)]
    pub node: BaseNode,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportDefaultDeclaration {
    #[serde(flatten)]
    pub node: BaseNode,
    pub declaration: Declaration,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum Declaration {
    AnonymousDefaultExportedFunctionDeclaration(AnonymousDefaultExportedFunctionDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    AnonymousDefaultExportedClassDeclaration(AnonymousDefaultExportedClassDeclaration),
    ClassDeclaration(ClassDeclaration),
    Expression(Box<ExpressionData>),
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ExportAllDeclaration {
    #[serde(flatten)]
    pub node: BaseNode,
    pub source: Literal,
}
