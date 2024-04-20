pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    TrailingData,
    InvalidSyntax(&'static str),
    EmptyBuffer,
    InvalidToken(char),
    IO(std::io::Error),
    UTF8(std::string::FromUtf8Error),
    ParseInt(std::num::ParseIntError),
    ExpectedByte(u8),
    Serde(String),
}

impl Error {
    pub fn from_io(err: std::io::Error) -> Self {
        Error::IO(err)
    }

    pub fn from_utf8(err: std::string::FromUtf8Error) -> Self {
        Error::UTF8(err)
    }

    pub fn from_parse_int(err: std::num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }

    pub fn from_serde(err: String) -> Self {
        Error::Serde(err)
    }

    pub fn expected_byte(expected: u8) -> Self {
        Error::ExpectedByte(expected)
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::TrailingData => write!(f, "Trailing data"),
            Error::InvalidSyntax(str) => write!(f, "Invalid syntax: {}", str),
            Error::EmptyBuffer => write!(f, "Empty buffer"),
            Error::IO(err) => write!(f, "IO error: {}", err),
            Error::UTF8(err) => write!(f, "UTF-8 error: {}", err),
            Error::ParseInt(err) => write!(f, "ParseInt error: {}", err),
            Error::ExpectedByte(expected) => {
                write!(f, "Expected byte: {}", *expected as char)
            }
            Error::Serde(err) => write!(f, "Serde error: {}", err),
            Error::InvalidToken(c) => write!(f, "Invalid token: {}", *c),
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::from_serde(msg.to_string())
    }
}
