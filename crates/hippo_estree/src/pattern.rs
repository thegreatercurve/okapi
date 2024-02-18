use crate::{Expression, Identifier, Node, Property};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FunctionParameter {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
    Assignment(AssignmentPattern),
    Rest(RestElement),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Pattern {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
    Rest(Box<RestElement>),
    Assignment(Box<AssignmentPattern>),
}

impl From<ArrayPatternElement> for Pattern {
    fn from(pattern: ArrayPatternElement) -> Self {
        match pattern {
            ArrayPatternElement::Identifier(identifier) => Pattern::Identifier(identifier),
            ArrayPatternElement::Object(object_pattern) => Pattern::Object(object_pattern),
            ArrayPatternElement::Array(array_pattern) => Pattern::Array(array_pattern),
            ArrayPatternElement::Rest(rest_element) => Pattern::Rest(Box::new(rest_element)),
            ArrayPatternElement::Assignment(assignment_pattern) => {
                Pattern::Assignment(Box::new(assignment_pattern))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ArrayPattern {
    #[serde(flatten)]
    pub node: Node,
    pub elements: Vec<Option<ArrayPatternElement>>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ArrayPatternElement {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
    Rest(RestElement),
    Assignment(AssignmentPattern),
}

impl ArrayPatternElement {
    pub fn to_function_parameter(self) -> FunctionParameter {
        match self {
            ArrayPatternElement::Identifier(identifier) => {
                FunctionParameter::Identifier(identifier)
            }
            ArrayPatternElement::Object(object_pattern) => {
                FunctionParameter::Object(object_pattern)
            }
            ArrayPatternElement::Array(array_pattern) => FunctionParameter::Array(array_pattern),

            ArrayPatternElement::Assignment(assignment_pattern) => {
                FunctionParameter::Assignment(assignment_pattern)
            }

            ArrayPatternElement::Rest(assignment_pattern) => {
                FunctionParameter::Rest(assignment_pattern)
            }
        }
    }

    pub fn to_pattern(self) -> Pattern {
        match self {
            ArrayPatternElement::Identifier(identifier) => Pattern::Identifier(identifier),
            ArrayPatternElement::Object(object_pattern) => Pattern::Object(object_pattern),
            ArrayPatternElement::Array(array_pattern) => Pattern::Array(array_pattern),
            ArrayPatternElement::Rest(rest_element) => Pattern::Rest(Box::new(rest_element)),
            ArrayPatternElement::Assignment(assignment_pattern) => {
                Pattern::Assignment(Box::new(assignment_pattern))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RestElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Pattern,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentPattern {
    #[serde(flatten)]
    pub node: Node,
    pub left: Pattern,
    pub right: Expression,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectPattern {
    #[serde(flatten)]
    pub node: Node,
    pub properties: Vec<ObjectPatternProperty>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObjectPatternProperty {
    Property(Property),
    Rest(RestElement),
}
