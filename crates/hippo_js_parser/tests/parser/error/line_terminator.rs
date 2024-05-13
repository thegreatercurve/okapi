use hippo_js_parser::ParserError;

use crate::parser::test_helper_macros::assert_parse_module_to_throw;

#[test]
fn line_terminator_throw_expression() {
    assert_parse_module_to_throw!(
        r"throw
Error()",
        ParserError::UnexpectedLineTerminator
    );
}

#[test]
fn line_terminator_arrow_function() {
    assert_parse_module_to_throw!(
        r"() 
=> null",
        ParserError::UnexpectedLineTerminator
    );
}
