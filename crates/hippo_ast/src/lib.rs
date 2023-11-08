use hermes::parser::{HermesParser, NodePtr, ParserDialect, ParserFlags};
use juno_support::NullTerminatedBuf;

pub fn parse(source: &str) -> NodePtr {
    let buf = NullTerminatedBuf::from_str_copy(source);
    let result = HermesParser::parse(
        ParserFlags {
            dialect: ParserDialect::TypeScript,
            enable_jsx: true,
            store_doc_block: true,
            strict_mode: true,
        },
        &buf,
    );

    result.root().unwrap()
}
