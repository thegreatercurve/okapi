use crate::{
    ast::{
        ArrayExpression, ArrayExpressionElement, AssignmentExpression, AssignmentExpressionLeft,
        Expression, Identifier, MemberExpression, Node, ObjectExpression, ObjectExpressionProperty,
        Property,
    },
    ParserError,
};
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum FunctionParameter {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
    Assignment(AssignmentPattern),
    RestElement(RestElement),
    MemberExpression(MemberExpression),
}

impl TryFrom<ArrayPatternElement> for FunctionParameter {
    type Error = ParserError;

    fn try_from(pattern: ArrayPatternElement) -> Result<Self, Self::Error> {
        match pattern {
            ArrayPatternElement::Identifier(identifier) => {
                Ok(FunctionParameter::Identifier(identifier))
            }
            ArrayPatternElement::Object(object_pattern) => {
                Ok(FunctionParameter::Object(object_pattern))
            }
            ArrayPatternElement::Array(array_pattern) => {
                Ok(FunctionParameter::Array(array_pattern))
            }
            ArrayPatternElement::RestElement(rest_element) => {
                Ok(FunctionParameter::RestElement(rest_element))
            }
            ArrayPatternElement::Assignment(assignment_pattern) => {
                Ok(FunctionParameter::Assignment(assignment_pattern))
            }
            ArrayPatternElement::MemberExpression(member_expression) => {
                Ok(FunctionParameter::MemberExpression(member_expression))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Pattern {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
    RestElement(RestElement),
    Assignment(AssignmentPattern),
    MemberExpression(MemberExpression), // There is an open issue within ESTree (https://github.com/estree/estree/issues/162) whether or not this is a Pattern. For now, we will treat it as such, similat to Rollup: https://github.com/rollup/rollup/pull/2760
}

impl TryFrom<Expression> for Pattern {
    type Error = ParserError;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Identifier(identifier) => Ok(Pattern::Identifier(identifier)),
            Expression::Object(object_expression) => {
                Ok(Pattern::Object(ObjectPattern::try_from(object_expression)?))
            }
            Expression::Array(array_expression) => {
                Ok(Pattern::Array(ArrayPattern::try_from(array_expression)?))
            }
            Expression::Member(member_expression) => {
                Ok(Pattern::MemberExpression(member_expression))
            }
            Expression::Assignment(assignment_expression) => Ok(Pattern::Assignment(
                AssignmentPattern::try_from(assignment_expression)?,
            )),
            _ => Err(ParserError::InvalidExpressionToPatternConversion),
        }
    }
}

impl TryFrom<ArrayPatternElement> for Pattern {
    type Error = ParserError;

    fn try_from(pattern: ArrayPatternElement) -> Result<Self, Self::Error> {
        match pattern {
            ArrayPatternElement::Identifier(identifier) => Ok(Pattern::Identifier(identifier)),
            ArrayPatternElement::Object(object_pattern) => Ok(Pattern::Object(object_pattern)),
            ArrayPatternElement::Array(array_pattern) => Ok(Pattern::Array(array_pattern)),
            ArrayPatternElement::RestElement(rest_element) => {
                Ok(Pattern::RestElement(rest_element))
            }
            ArrayPatternElement::Assignment(assignment_pattern) => {
                Ok(Pattern::Assignment(assignment_pattern))
            }
            ArrayPatternElement::MemberExpression(member_expression) => {
                Ok(Pattern::MemberExpression(member_expression))
            }
        }
    }
}

impl TryFrom<AssignmentExpressionLeft> for Pattern {
    type Error = ParserError;

    fn try_from(assignment_expression_left: AssignmentExpressionLeft) -> Result<Self, Self::Error> {
        match assignment_expression_left {
            AssignmentExpressionLeft::Pattern(pattern) => Ok(pattern),
            AssignmentExpressionLeft::Expression(expression) => Pattern::try_from(expression),
        }
    }
}

impl TryFrom<FunctionParameter> for Pattern {
    type Error = ParserError;

    fn try_from(assignment_expression_left: FunctionParameter) -> Result<Self, Self::Error> {
        match assignment_expression_left {
            FunctionParameter::Identifier(identifier) => Ok(Pattern::Identifier(identifier)),
            FunctionParameter::Object(object_pattern) => Ok(Pattern::Object(object_pattern)),
            FunctionParameter::Array(array_pattern) => Ok(Pattern::Array(array_pattern)),
            FunctionParameter::RestElement(rest_element) => Ok(Pattern::RestElement(rest_element)),
            FunctionParameter::Assignment(assignment_pattern) => {
                Ok(Pattern::Assignment(assignment_pattern))
            }
            FunctionParameter::MemberExpression(member_expression) => {
                Ok(Pattern::MemberExpression(member_expression))
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

impl TryFrom<ArrayExpression> for ArrayPattern {
    type Error = ParserError;

    fn try_from(array_expression: ArrayExpression) -> Result<Self, Self::Error> {
        let elements = array_expression
            .elements
            .into_iter()
            .filter_map(|element| {
                element.map(|element| ArrayPatternElement::try_from(element).ok())
            })
            .collect::<Vec<Option<ArrayPatternElement>>>();

        Ok(ArrayPattern {
            node: array_expression.node,
            elements,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ArrayPatternElement {
    Identifier(Identifier),
    Object(ObjectPattern),
    Array(ArrayPattern),
    RestElement(RestElement),
    Assignment(AssignmentPattern),
    MemberExpression(MemberExpression), // There is an open issue within ESTree (https://github.com/estree/estree/issues/162) whether or not this is a Pattern. For now, we will treat it as such, similat to Rollup: https://github.com/rollup/rollup/pull/2760
}

impl TryFrom<Expression> for ArrayPatternElement {
    type Error = ParserError;

    fn try_from(expression: Expression) -> Result<Self, Self::Error> {
        match expression {
            Expression::Identifier(identifier) => Ok(ArrayPatternElement::Identifier(identifier)),
            Expression::Object(object_expression) => Ok(ArrayPatternElement::Object(
                ObjectPattern::try_from(object_expression)?,
            )),
            Expression::Array(array_expression) => Ok(ArrayPatternElement::Array(
                ArrayPattern::try_from(array_expression)?,
            )),
            Expression::Member(member_expression) => {
                Ok(ArrayPatternElement::MemberExpression(member_expression))
            }
            Expression::Assignment(assignment_expression) => Ok(ArrayPatternElement::Assignment(
                AssignmentPattern::try_from(assignment_expression)?,
            )),
            _ => Err(ParserError::InvalidExpressionToArrayPatternElementConversion),
        }
    }
}

impl TryFrom<ArrayExpressionElement> for ArrayPatternElement {
    type Error = ParserError;

    fn try_from(element: ArrayExpressionElement) -> Result<Self, Self::Error> {
        match element {
            ArrayExpressionElement::Expression(expression) => {
                ArrayPatternElement::try_from(expression)
            }
            ArrayExpressionElement::SpreadElement(spread_element) => {
                Ok(ArrayPatternElement::RestElement(RestElement {
                    node: spread_element.node,
                    argument: Box::new(Pattern::try_from(spread_element.argument)?),
                }))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct RestElement {
    #[serde(flatten)]
    pub node: Node,
    pub argument: Box<Pattern>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct AssignmentPattern {
    #[serde(flatten)]
    pub node: Node,
    pub left: Box<Pattern>,
    pub right: Expression,
}

impl TryFrom<AssignmentExpression> for AssignmentPattern {
    type Error = ParserError;

    fn try_from(assignment_expression: AssignmentExpression) -> Result<Self, Self::Error> {
        Ok(AssignmentPattern {
            node: assignment_expression.node,
            left: Box::new(Pattern::try_from(*assignment_expression.left)?),
            right: *assignment_expression.right,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub struct ObjectPattern {
    #[serde(flatten)]
    pub node: Node,
    pub properties: Vec<ObjectPatternProperty>,
}

impl TryFrom<ObjectExpression> for ObjectPattern {
    type Error = ParserError;

    fn try_from(object_expression: ObjectExpression) -> Result<Self, Self::Error> {
        let properties = object_expression
            .properties
            .into_iter()
            .map(ObjectPatternProperty::try_from)
            .collect::<Result<Vec<ObjectPatternProperty>, ParserError>>()?;

        Ok(ObjectPattern {
            node: object_expression.node,
            properties,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum ObjectPatternProperty {
    Property(Property),
    Rest(RestElement),
}

impl TryFrom<ObjectExpressionProperty> for ObjectPatternProperty {
    type Error = ParserError;

    fn try_from(property: ObjectExpressionProperty) -> Result<Self, Self::Error> {
        match property {
            ObjectExpressionProperty::Property(property) => {
                Ok(ObjectPatternProperty::Property(property))
            }
            ObjectExpressionProperty::SpreadElement(spread_element) => {
                Ok(ObjectPatternProperty::Rest(RestElement {
                    node: spread_element.node,
                    argument: Box::new(Pattern::try_from(spread_element.argument)?),
                }))
            }
        }
    }
}
