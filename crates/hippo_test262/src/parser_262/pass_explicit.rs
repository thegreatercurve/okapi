use std::path::PathBuf;

use crate::errors::ErrorKind;
use crate::harness::Harness;
use crate::test::Test;

use crate::parser_262::{ParserTest, ParserTestMetaData};

const TEST_PATH: &str = "test262-parser-tests/pass-explicit";

pub fn run_tests() -> Result<(), ErrorKind> {
    let harness = Harness::new(TEST_PATH)?;

    let tests = harness
        .tests
        .into_iter()
        .filter(|x| x.to_str().unwrap().contains("4fee4ac53bdfd7f7"))
        .collect::<Vec<PathBuf>>();

    for test_path in tests {
        let source = ParserTest::read_file(&test_path)?;

        let Some(file_name) = test_path.file_name() else {
            return Err(ErrorKind::InvalidMetaData);
        };

        let Some(file_name) = file_name.to_str() else {
            return Err(ErrorKind::InvalidMetaData);
        };

        let is_module = file_name.contains("module");

        let Some(file_name) = file_name.split('.').next() else {
            return Err(ErrorKind::InvalidMetaData);
        };

        let meta_data = ParserTestMetaData {
            module: is_module,
            file_name: file_name.to_string(),
        };

        let test = ParserTest::new(&source, meta_data);

        println!("Test running: {}", test.description());

        match test.run() {
            Ok(true) => println!("Test passed: {}", test.description()),
            Ok(false) => println!(
                "
Test failed: {}
```
{}
```
",
                test.description(),
                test.source()
            ),
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        };
    }

    Ok(())
}
