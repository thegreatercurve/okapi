use std::{fs::File, io::Read, path::PathBuf};

fn load_harness(path: &str) -> String {
    // Get the path to the Cargo.toml file (project root)
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    // Construct the full path by joining the project root and the relative path
    let full_path = PathBuf::from(manifest_dir).join(format!("tests/test262/{}", path));

    let mut file = File::open(full_path).unwrap();

    let mut code = String::new();

    file.read_to_string(&mut code).unwrap();

    return code;
}

#[test]
fn test_262_all() {
    load_harness("harness/assert.js");
    load_harness("harness/sta.js");
    load_harness("harness/compareArray.js");

    // assert_eq!(true, true)
}
