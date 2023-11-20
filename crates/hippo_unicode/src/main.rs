use reqwest::{self, Error as ReqwestError};
use std::env;
use std::fmt::Error;

fn download_derived_core_properties(version: &str) -> Result<String, ReqwestError> {
    const BASEURL: &str = "https://unicode.org/Public";
    const FILENAME: &str = "DerivedCoreProperties.txt";

    let url = match version {
        "UNIDATA" => format!("{}/{}/{}", BASEURL, version, FILENAME),
        _ => format!("{}/{}/uct/{}", BASEURL, version, FILENAME),
    };

    let response = reqwest::blocking::get(url);

    println!("{:?}", response?.status());

    response?.text()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut args_iter = args.iter().skip(2);

    let mut version: &str = "UNIDATA";

    while let Some(arg) = args_iter.next() {
        match arg.as_str() {
            "--version" | "-v" => {
                version = match args_iter.next() {
                    Some(value) => value,
                    None => "UNIDATA",
                };
            }
            _ => {
                eprintln!("Unknown argument: {}", arg);
                std::process::exit(1);
            }
        }
    }

    println!("{}", download_derived_core_properties(version).unwrap());
}
