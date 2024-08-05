#[derive(Debug)]
pub enum DemangleError {
    Utf8Error(std::str::Utf8Error),
    Null,
}

impl std::error::Error for DemangleError {}

impl std::fmt::Display for DemangleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DemangleError::Utf8Error(err) => {
                write!(f, "Error: {:?}", err)
            }
            DemangleError::Null => write!(f, "demangle returned null"),
        }
    }
}