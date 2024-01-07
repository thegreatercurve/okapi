use std::fs;
use std::io::{self, BufReader, Read};
use std::path::Path;

fn parse_file_contents(file_path: &Path) -> Result<(), io::Error> {
    let file = fs::File::open(file_path)?;
    let mut reader = BufReader::new(file);

    // Buffer to store the file contents
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    // Process the file contents as needed
    println!("File: {:?}\nContents:\n{}", file_path, buffer);

    // Add your parsing logic here

    Ok(())
}

fn parse_files_in_directory(directory_path: &str) -> Result<(), io::Error> {
    let entries = fs::read_dir(directory_path)?;

    for entry in entries {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            if let Err(err) = parse_file_contents(&file_path) {
                eprintln!("Error parsing file {:?}: {}", file_path, err);
            }
        } else if file_path.is_dir() {
            // You can choose to recurse into subdirectories if needed
            // parse_files_in_directory(file_path.to_str().unwrap())?;
        }
    }

    Ok(())
}
