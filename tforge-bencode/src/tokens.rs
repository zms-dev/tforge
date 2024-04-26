use crate::error::{Error, Result};

pub const TOKEN_INTEGER: u8 = b'i';
pub const TOKEN_DICT: u8 = b'd';
pub const TOKEN_LIST: u8 = b'l';
pub const TOKEN_END: u8 = b'e';
pub const TOKEN_DELIM: u8 = b':';

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Token {
    Int,
    Bytes,
    List,
    Dict,
    End,
}

impl TryFrom<u8> for Token {
    type Error = Error;

    fn try_from(byte: u8) -> Result<Token> {
        match byte {
            TOKEN_INTEGER => Ok(Token::Int),
            TOKEN_DICT => Ok(Token::Dict),
            TOKEN_LIST => Ok(Token::List),
            TOKEN_END => Ok(Token::End),
            b'0'..=b'9' => Ok(Token::Bytes),
            _ => Err(Error::from_syntax(format!(
                "Unrecognized token: {}",
                byte as char
            ))),
        }
    }
}

impl TryInto<u8> for Token {
    type Error = Error;

    fn try_into(self) -> Result<u8> {
        match self {
            Token::Int => Ok(TOKEN_INTEGER),
            Token::Dict => Ok(TOKEN_DICT),
            Token::List => Ok(TOKEN_LIST),
            Token::End => Ok(TOKEN_END),
            Token::Bytes => Err(Error::from_syntax("Cannot convert Bytes to u8")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_try_from() {
        assert_eq!(Token::try_from(b'i').unwrap(), Token::Int);
        assert_eq!(Token::try_from(b'd').unwrap(), Token::Dict);
        assert_eq!(Token::try_from(b'l').unwrap(), Token::List);
        assert_eq!(Token::try_from(b'e').unwrap(), Token::End);

        let bytes = b"0123456789";
        for byte in bytes {
            assert_eq!(Token::try_from(*byte).unwrap(), Token::Bytes);
        }

        assert!(Token::try_from(b'x').is_err());
    }

    #[test]
    fn test_token_try_into() {
        let result: u8 = Token::Int.try_into().unwrap();
        assert_eq!(result, TOKEN_INTEGER);

        let result: u8 = Token::Dict.try_into().unwrap();
        assert_eq!(result, TOKEN_DICT);

        let result: u8 = Token::List.try_into().unwrap();
        assert_eq!(result, TOKEN_LIST);

        let result: u8 = Token::End.try_into().unwrap();
        assert_eq!(result, TOKEN_END);

        let result: Result<u8> = Token::Bytes.try_into();
        assert!(result.is_err());
    }
}
