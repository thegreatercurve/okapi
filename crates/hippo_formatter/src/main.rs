use oxc_allocator::Allocator;

const DEFAULT_CODE: &str = "
function hello() {
    return \"world\";
}
";

fn main() {
    let allocator = Allocator::default();

    let parser_result = hippo_ast::parse(&allocator, DEFAULT_CODE);

    match parser_result {
        Ok(_ast) => {
            print_ast_to_doc();
        }
        Err(error) => {
            println!("{:#?}", error);
        }
    }
}

fn print_ast_to_doc() {}
