use crate::lexer::Lexer;

pub mod lexer;
fn main() {
    let lexer = Lexer::new("const hello = 'world';");

    println!("{:?}", lexer);
}
