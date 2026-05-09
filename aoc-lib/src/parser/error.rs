use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    EmptyInput,
    NotADigit(char),
    WrongLength {
        expected: usize,
        got: usize,
        input: String,
    },
    NotWrapped {
        open: String,
        close: String,
    },
    Other(String),
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::EmptyInput => write!(f, "unexpected empty input"),
            Self::NotADigit(c) => write!(f, "expected a digit, got '{c}'"),
            Self::WrongLength {
                expected,
                got,
                input,
            } => {
                write!(f, "expected {expected} items, got {got} in \"{input}\"")
            }
            Self::NotWrapped { open, close } => {
                write!(f, "expected input wrapped in {open} ... {close}")
            }
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<String> for ParseError {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

impl From<&str> for ParseError {
    fn from(s: &str) -> Self {
        Self::Other(s.to_string())
    }
}
