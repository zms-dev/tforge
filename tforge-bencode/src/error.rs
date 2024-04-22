pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    IO(std::io::Error),
    UTF8(std::string::FromUtf8Error),
    ParseInt(std::num::ParseIntError),
    Syntax(String),
    EOF,
    TrailingData(Vec<u8>),
    ExpectedBytes,
    ExpectedDelimiter,
    ExpectedEnd,
    ExpectedList,
    UnsupportedType(std::any::TypeId),
}

impl Error {
    pub fn from_internal(err: String) -> Self {
        Error::Internal(err)
    }

    pub fn from_io(err: std::io::Error) -> Self {
        Error::IO(err)
    }

    pub fn from_utf8(err: std::string::FromUtf8Error) -> Self {
        Error::UTF8(err)
    }

    pub fn from_parse_int(err: std::num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }

    pub fn from_syntax(err: impl Into<String>) -> Self {
        Error::Syntax(err.into())
    }

    pub fn from_trailing_data(data: Vec<u8>) -> Self {
        Error::TrailingData(data)
    }

    pub fn from_unsupported_type<T: 'static>() -> Self {
        Error::UnsupportedType(std::any::TypeId::of::<T>())
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Internal(err) => write!(f, "Internal error: {}", err),
            Error::IO(err) => write!(f, "IO error: {}", err),
            Error::UTF8(err) => write!(f, "UTF8 error: {}", err),
            Error::ParseInt(err) => write!(f, "ParseInt error: {}", err),
            Error::EOF => write!(f, "EOF"),
            Error::ExpectedBytes => write!(f, "Expected bytes"),
            Error::ExpectedDelimiter => write!(f, "Expected delimiter"),
            Error::ExpectedEnd => write!(f, "Expected end"),
            Error::ExpectedList => write!(f, "Expected list"),
            Error::TrailingData(data) => {
                write!(f, "Trailing data: {}", String::from_utf8_lossy(data))
            }
            Error::Syntax(err) => write!(f, "Syntax error: {}", err),
            Error::UnsupportedType(type_id) => {
                write!(f, "Unsupported type: {:?}", type_id)
            }
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::from_internal(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::from_internal(msg.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::from_io(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::from_utf8(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::from_parse_int(err)
    }
}
