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
}
