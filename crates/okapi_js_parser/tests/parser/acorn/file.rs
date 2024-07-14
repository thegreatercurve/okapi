use std::{
    fs::File,
    io::{self, BufReader, Read},
    path::PathBuf,
};

pub(crate) fn read_file(path: &PathBuf) -> io::Result<String> {
    let file = File::open(path)
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("File not found: {:?}", e)))?;

    let mut reader = BufReader::new(file);
    let mut contents = String::new();

    reader.read_to_string(&mut contents)?;

    Ok(contents)
}
