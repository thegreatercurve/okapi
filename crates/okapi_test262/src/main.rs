use errors::ErrorKind;
use parser_262::run_pass_explicit_tests;

mod errors;
mod harness;
mod parser_262;
mod test;

pub fn main() -> Result<(), ErrorKind> {
    run_pass_explicit_tests()?;

    Ok(())
}
