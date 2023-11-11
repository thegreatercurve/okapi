use oxc_allocator::Allocator;
use oxc_ast::ast::Program;
use oxc_diagnostics::Error;
use oxc_parser::Parser;
use oxc_span::SourceType;

pub fn parse<'a>(
    allocator: &'a Allocator,
    source_text: &'a str,
) -> Result<Program<'a>, Vec<Error>> {
    let source_type = SourceType::default();
    let ret = Parser::new(&allocator, &source_text, source_type).parse();

    if ret.errors.is_empty() {
        return Ok(ret.program);
    } else {
        return Err(ret.errors);
    }
}
