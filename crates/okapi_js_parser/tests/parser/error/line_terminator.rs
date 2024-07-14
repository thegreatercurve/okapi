use okapi_js_parser::ParserError;

use crate::parser::test_helper_macros::assert_parse_module_to_throw;

#[test]
fn line_terminator_throw_expression() {
    assert_parse_module_to_throw!(
        r"throw
Error()",
        ParserError::UnexpectedLineTerminator
    );
}
