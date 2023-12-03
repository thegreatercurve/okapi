use crate::Parser;

macro_rules! assert_parser_eq {
    ($input_str: expr) => {{
        let parser = Parser::new($input_str);

        // while !scanner.parser() {
        //     let token = scanner.next_token();

        //     let expected_token = tests
        //         .pop_front()
        //         .unwrap_or_else(|| panic!("Unexpected end to queue"));

        //     println!("{:?} {:?}", token, expected_token);

        //     assert_eq!(
        //         expected_token, token,
        //         "Expected token {:?}, but found {:?}",
        //         expected_token, token,
        //     );
        // }
    }};
}

#[test]
fn keywords_and_identifiers() {
    assert_parser_eq!("const foo = 1;");
}
