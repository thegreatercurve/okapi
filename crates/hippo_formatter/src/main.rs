use std::collections::VecDeque;

use oxc_allocator::Allocator;
use oxc_ast::ast::Program;

const DEFAULT_CODE: &str = "
function hello() {
    return \"world\";
}
";

fn main() {
    let allocator = Allocator::default();

    let parser_result = hippo_ast::parse(&allocator, DEFAULT_CODE);

    match parser_result {
        Ok(ast) => {
            print_ast_to_doc(ast);
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}

fn print_ast_to_doc<'a>(ast: Program<'a>) {
    let body = ast.body;
}
