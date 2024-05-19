use std::path::PathBuf;

use walkdir::WalkDir;

use crate::errors::ErrorKind;

pub(crate) struct Harness {
    pub tests: Vec<PathBuf>,
}

impl Harness {
    pub fn new(file_path: &str) -> Result<Self, ErrorKind> {
        let tests = read_tests(file_path)?;

        Ok(Harness { tests })
    }
}

fn read_tests(test_path: &str) -> Result<Vec<PathBuf>, ErrorKind> {
    let mut full_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    full_path.push(test_path);

    let tests = WalkDir::new(full_path)
        .min_depth(1)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            let path = entry.path();

            if path.is_dir() {
                return false;
            }

            let Some(ext) = path.extension() else {
                return false;
            };

            if ext == "js" {
                let Some(file_name) = path.file_name() else {
                    return false;
                };

                let Some(file_name) = file_name.to_str() else {
                    return false;
                };

                if file_name.ends_with("_FIXTURE.js") {
                    return false;
                } else {
                    return true;
                }
            } else {
                return false;
            }
        })
        .map(|entry| entry.path().to_path_buf())
        .collect::<Vec<PathBuf>>();

    Ok(tests)
}
