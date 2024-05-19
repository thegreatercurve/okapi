use errors::ErrorKind;
use parser_262::run_tests as run_parser_pass_tests;

mod errors;
mod harness;
mod parser_262;
mod runtime_262;
mod test;

pub fn main() -> Result<(), ErrorKind> {
    run_parser_pass_tests()?;

    Ok(())
}
