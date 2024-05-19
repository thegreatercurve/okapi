use std::{fs::File, io::BufReader, path::PathBuf};

use hippo_js_parser::{Config, Parser};

use crate::errors::ErrorKind;

pub(crate) trait Test<'a> {
    type MetaDataType;
    fn new(source: &str, metadata: Self::MetaDataType) -> Self;
    fn description(&self) -> &str;
    fn source(&self) -> &str;
    fn run(&self) -> Result<bool, ErrorKind>;
    fn read_file(path: &PathBuf) -> Result<String, ErrorKind> {
        use std::io::Read;

        let file = File::open(&path).map_err(ErrorKind::Io)?;

        let mut reader = BufReader::new(file);
        let mut contents = String::new();

        reader
            .read_to_string(&mut contents)
            .map_err(ErrorKind::Io)?;

        Ok(contents)
    }
    fn parse_module(&self, strict_mode: bool) -> Result<bool, ErrorKind> {
        let source = &self.source();

        let mut parser = Parser::new(&source, Config::new(strict_mode));

        match parser.parse_module() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    fn parse_script(&self, strict_mode: bool) -> Result<bool, ErrorKind> {
        let source = &self.source();

        let mut parser = Parser::new(&source, Config::new(strict_mode));

        match parser.parse_script() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
