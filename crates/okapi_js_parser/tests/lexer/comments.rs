use crate::lexer::{assert_lexer_eq, utils::string_literal};

#[test]
fn single_line_comment() {
    assert_lexer_eq!(
        r"'hello' // foo bar",
        vec![string_literal("hello", "'hello'", 0, 7, 1, 1)]
    );
    assert_lexer_eq!(
        r"// foo bar
// baz
//
'hello'",
        vec![string_literal("hello", "'hello'", 21, 28, 4, 1)]
    );
}

#[test]
fn multi_line_comment() {
    assert_lexer_eq!(
        "'hello' /* foo bar */",
        vec![string_literal("hello", "'hello'", 0, 7, 1, 1)]
    );
    assert_lexer_eq!(
        r"/* foo
* bar
*/
'hello'",
        vec![string_literal("hello", "'hello'", 16, 23, 4, 1)]
    );
}
