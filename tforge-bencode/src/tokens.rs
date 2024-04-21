use anyhow::{anyhow, Error, Result};

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
            _ => Err(anyhow!("Invalid token")),
        }
    }
}

impl TryInto<u8> for Token {
    type Error = Error;

    fn try_into(self) -> Result<u8> {
        match self {
            Token::Int => Ok(TOKEN_INTEGER),
            Token::List => Ok(TOKEN_LIST),
            Token::Dict => Ok(TOKEN_DICT),
            Token::End => Ok(TOKEN_END),
            Token::Bytes => Err(anyhow!("Bytes token is not supported")),
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
        let int_token: u8 = Token::Int.try_into().unwrap();
        assert_eq!(int_token, TOKEN_INTEGER);

        let dict_token: u8 = Token::Dict.try_into().unwrap();
        assert_eq!(dict_token, TOKEN_DICT);

        let list_token: u8 = Token::List.try_into().unwrap();
        assert_eq!(list_token, TOKEN_LIST);

        let end_token: u8 = Token::End.try_into().unwrap();
        assert_eq!(end_token, TOKEN_END);

        let bytes_token: Result<u8> = Token::Bytes.try_into();
        assert!(bytes_token.is_err());
    }
}
