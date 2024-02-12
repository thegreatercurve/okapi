use crate::{Parser, ParserError, TokenKind};
use hippo_estree::*;

// 15 ECMAScript Language: Functions and Classes
// https://tc39.es/ecma262/#sec-ecmascript-language-functions-and-classes
impl<'a> Parser<'a> {
    // 15.1 Parameter Lists
    // https://tc39.es/ecma262/#prod-FormalParameters
    pub(crate) fn parse_formal_parameters(&mut self) -> Result<Vec<Pattern>, ParserError> {
        let mut parameter_list = vec![];

        if self.cursor.current_token_kind() == TokenKind::Ellipsis {
            parameter_list.push(self.parse_function_rest_parameter()?);

            return Ok(parameter_list);
        }

        parameter_list.extend(self.parse_formal_parameter_list()?);

        if self.cursor.current_token_kind() == TokenKind::Ellipsis {
            parameter_list.push(self.parse_function_rest_parameter()?);
        }

        Ok(parameter_list)
    }

    // https://tc39.es/ecma262/#prod-FormalParameterList
    fn parse_formal_parameter_list(&mut self) -> Result<Vec<Pattern>, ParserError> {
        let mut parameter_list = vec![];

        while self.cursor.current_token_kind() == TokenKind::Identifier {
            let formal_parameter = self.parse_formal_parameter()?;

            parameter_list.push(formal_parameter);

            if self.cursor.current_token_kind() == TokenKind::Comma {
                self.cursor.advance(); // Eat ',' token.
            } else {
                break;
            }
        }

        Ok(parameter_list)
    }

    // https://tc39.es/ecma262/#prod-FunctionRestParameter
    fn parse_function_rest_parameter(&mut self) -> Result<Pattern, ParserError> {
        self.parse_binding_rest_element()
    }

    // https://tc39.es/ecma262/#prod-FormalParameter
    fn parse_formal_parameter(&mut self) -> Result<Pattern, ParserError> {
        self.parse_binding_element()
    }
}
