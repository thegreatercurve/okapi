macro_rules! assert_parser_eq {
    ($filename: expr) => {{
        use hippo_js_parser::Parser;
        use pretty_assertions::assert_eq;
        use std::{
            fs::File,
            io::{BufReader, Error, Read},
        };

        const FILE_DIR: &str = "tests/parser/fixtures";

        pub fn read_file_with_buffer(file_path: &str) -> Result<String, Error> {
            let file = File::open(file_path)?;
            let mut reader = BufReader::new(file);
            let mut buffer = String::new();

            reader.read_to_string(&mut buffer)?;

            Ok(buffer)
        }

        let input_path = format!("{}/{}.js", FILE_DIR, $filename);

        let input_buffer = read_file_with_buffer(input_path.as_str()).unwrap();

        let output_path = format!("{}/{}.json", FILE_DIR, $filename);

        let output_buffer = read_file_with_buffer(output_path.as_str())
            .unwrap()
            .replace(char::is_whitespace, "");

        let mut parser = Parser::new(&input_buffer);

        let input_ast = parser.parse_json().unwrap();

        assert_eq!(
            output_buffer, input_ast,
            "Expected token {:#?}, but found {:#?}",
            output_buffer, input_ast,
        );
    }};
}

pub(crate) use assert_parser_eq;
