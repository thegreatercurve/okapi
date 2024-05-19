#[derive(Debug)]
pub enum ErrorKind {
    InvalidMetaData,
    Io(std::io::Error),
    RegEx(regex::Error),
    WalkDir(walkdir::Error),
    Yaml(serde_yaml::Error),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidMetaData => write!(f, "Invalid metadata in test header."),
            ErrorKind::Io(e) => e.fmt(f),
            ErrorKind::RegEx(e) => e.fmt(f),
            ErrorKind::WalkDir(e) => e.fmt(f),
            ErrorKind::Yaml(e) => e.fmt(f),
        }
    }
}

impl From<serde_yaml::Error> for ErrorKind {
    fn from(err: serde_yaml::Error) -> Self {
        ErrorKind::Yaml(err)
    }
}
