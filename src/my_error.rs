use std::fmt;

#[derive(Debug)]
pub enum MyError {
    InvalidTable(String),
    InvalidColumn(String),
    InvalidSyntax(String),
    Error(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::InvalidTable(ref msg) => write!(f, "INVALID_TABLE: {}", msg),
            MyError::InvalidColumn(ref msg) => write!(f, "INVALID_COLUMN: {}", msg),
            MyError::InvalidSyntax(ref msg) => write!(f, "INVALID_SYNTAX: {}", msg),
            MyError::Error(ref msg) => write!(f, "ERROR: {}", msg),
        }
    }
}
