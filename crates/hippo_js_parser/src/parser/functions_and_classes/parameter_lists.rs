use crate::ast::*;
use crate::{Parser, ParserError, TokenKind};

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl Parser {
    // 15.1 Parameter Lists
    // https://tc39.es/ecma262/#prod-FormalParameters
    pub(crate) fn parse_formal_parameters(
        &mut self,
    ) -> Result<Vec<ArrayPatternElement>, ParserError> {
        let mut parameter_list = vec![];

        if self.token_kind() == TokenKind::Ellipsis {
            parameter_list.push(self.parse_function_rest_parameter()?);

            return Ok(parameter_list);
        }

        parameter_list.extend(self.parse_formal_parameter_list()?);

        if self.token_kind() == TokenKind::Ellipsis {
            parameter_list.push(self.parse_function_rest_parameter()?);
        }

        if self.token_kind() == TokenKind::Comma {
            self.advance_any(); // Eat ',' token.
        }

        Ok(parameter_list)
    }

    // https://tc39.es/ecma262/#prod-FormalParameterList
    fn parse_formal_parameter_list(&mut self) -> Result<Vec<ArrayPatternElement>, ParserError> {
        let mut parameter_list = vec![];

        while self.token_kind().is_binding_identifier()
            || self.token_kind().is_binding_pattern_start()
        {
            let formal_parameter = self.parse_formal_parameter()?;

            parameter_list.push(formal_parameter);

            if self.token_kind() == TokenKind::Comma {
                self.advance_any(); // Eat ',' token.
            } else {
                break;
            }
        }

        Ok(parameter_list)
    }

    // https://tc39.es/ecma262/#prod-FunctionRestParameter
    fn parse_function_rest_parameter(&mut self) -> Result<ArrayPatternElement, ParserError> {
        self.parse_binding_rest_element()
    }

    // https://tc39.es/ecma262/#prod-FormalParameter
    pub(crate) fn parse_formal_parameter(&mut self) -> Result<ArrayPatternElement, ParserError> {
        self.parse_binding_element()
    }
}
