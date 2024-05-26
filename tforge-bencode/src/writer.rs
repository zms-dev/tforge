use crate::{
    error::Result,
    tokens::{Token, TOKEN_DELIM},
};
use std::io::Write;

pub trait BencodeWriter {
    fn write_token(&mut self, value: Token) -> Result<()>;

    fn write_signed_integer(&mut self, value: i64) -> Result<()>;
    fn write_unsigned_integer(&mut self, value: u64) -> Result<()>;
    fn write_bytes(&mut self, value: &[u8]) -> Result<()>;
    fn write_list_start(&mut self) -> Result<()>;
    fn write_dict_start(&mut self) -> Result<()>;
    fn write_end(&mut self) -> Result<()>;
}

impl<T: Write> BencodeWriter for T {
    fn write_token(&mut self, value: Token) -> Result<()> {
        self.write_all(&[value.try_into()?])?;
        Ok(())
    }

    fn write_signed_integer(&mut self, value: i64) -> Result<()> {
        self.write_token(Token::Int)?;
        self.write_all(value.to_string().as_bytes())?;
        self.write_token(Token::End)?;
        Ok(())
    }

    fn write_unsigned_integer(&mut self, value: u64) -> Result<()> {
        self.write_token(Token::Int)?;
        self.write_all(value.to_string().as_bytes())?;
        self.write_token(Token::End)?;
        Ok(())
    }

    fn write_bytes(&mut self, value: &[u8]) -> Result<()> {
        self.write_all(value.len().to_string().as_bytes())?;
        self.write_all(&[TOKEN_DELIM])?;
        self.write_all(value)?;
        Ok(())
    }

    fn write_list_start(&mut self) -> Result<()> {
        self.write_token(Token::List)
    }

    fn write_dict_start(&mut self) -> Result<()> {
        self.write_token(Token::Dict)
    }

    fn write_end(&mut self) -> Result<()> {
        self.write_token(Token::End)
    }
}
