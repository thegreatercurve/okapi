use serde::Deserialize;

use crate::{errors::ErrorKind, test::Test};

#[derive(Debug)]
pub(crate) struct ParserTest {
    meta_data: ParserTestMetaData,
    source: String,
}

impl<'a> Test<'a> for ParserTest {
    type MetaDataType = ParserTestMetaData;

    fn new(source: &str, meta_data: Self::MetaDataType) -> Self {
        Self {
            source: String::from(source),
            meta_data,
        }
    }

    fn source(&self) -> &str {
        &self.source
    }

    fn description(&self) -> &str {
        &self.meta_data.file_name
    }

    fn run(&self) -> Result<bool, ErrorKind> {
        if self.meta_data.module {
            self.parse_module(true)
        } else {
            self.parse_script(true)
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ParserTestMetaData {
    // Possible test ID.
    pub module: bool,
    // File name
    pub file_name: String,
}
