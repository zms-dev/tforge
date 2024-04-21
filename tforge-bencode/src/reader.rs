use crate::tokens::{Token, TOKEN_DELIM, TOKEN_END};
use anyhow::{anyhow, Context, Result};
use std::io::BufRead;

pub trait BencodeReader {
    fn peek_token(&mut self) -> Result<Token>;
    fn consume_current_token(&mut self) -> Result<()>;
    fn has_tokens_left(&mut self) -> Result<bool>;
    fn read_until_delim(&mut self) -> Result<Vec<u8>>;
    fn read_until_end(&mut self) -> Result<Vec<u8>>;
    fn read_of_size(&mut self, size: usize) -> Result<Vec<u8>>;
    fn read_i64(&mut self) -> Result<i64>;
    fn read_bytes(&mut self) -> Result<Vec<u8>>;
}

impl<T: BufRead> BencodeReader for T {
    fn peek_token(&mut self) -> Result<Token> {
        let buf = self.fill_buf().context("Failed to fill buffer")?;
        if buf.is_empty() {
            return Err(anyhow!("Empty buffer"));
        }
        Ok(Token::try_from(buf[0])?)
    }

    fn consume_current_token(&mut self) -> Result<()> {
        self.fill_buf().context("Failed to fill buffer")?;
        self.consume(1);
        Ok(())
    }

    fn has_tokens_left(&mut self) -> Result<bool> {
        let has_left = self
            .has_data_left()
            .context("Failed to check if there is data left")?;
        Ok(has_left)
    }

    fn read_until_delim(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.read_until(TOKEN_DELIM, &mut buf)
            .context("Failed to read until delimiter")?;
        if let Some(last) = buf.last() {
            if *last != TOKEN_DELIM {
                return Err(anyhow!("Buffer expected delimiter token"));
            }
        }
        Ok(buf)
    }

    fn read_until_end(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        self.read_until(TOKEN_END, &mut buf)
            .context("Failed to read until end token")?;
        if let Some(last) = buf.last() {
            if *last != TOKEN_END {
                return Err(anyhow!("Buffer expected end token"));
            }
        }
        Ok(buf)
    }

    fn read_of_size(&mut self, size: usize) -> Result<Vec<u8>> {
        let mut buf = vec![0; size];
        self.read_exact(&mut buf)
            .context("Failed to read of size")?;
        Ok(buf)
    }

    fn read_i64(&mut self) -> Result<i64> {
        let buf = self.read_until_end()?;
        let int_str = String::from_utf8(buf).context("Failed to convert integer buffer to UTF8")?;
        let parsed_int = &int_str[..int_str.len() - 1]
            .parse::<i64>()
            .context("Faile to parse integer string to i64")?;
        Ok(*parsed_int)
    }

    fn read_bytes(&mut self) -> Result<Vec<u8>> {
        let length_buf = self.read_until_delim()?;
        let lenth_str =
            String::from_utf8(length_buf).context("Failed to convert buffer length to UTF8")?;
        let length_int = &lenth_str[..lenth_str.len() - 1]
            .parse::<u64>()
            .context("Faile to parse buffer length string to i64")?;
        let buf = self.read_of_size(*length_int as usize)?;
        Ok(buf)
    }
}

impl Iterator for dyn BencodeReader {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.has_tokens_left() {
            Err(err) => return Some(Err(err)),
            Ok(false) => return None,
            Ok(true) => Some(self.peek_token()),
        }
    }
}

#[cfg(test)]
mod tests {
    mod bufread_impl {
        use super::super::*;
        use crate::tokens::Token;
        use std::io::Cursor;

        #[test]
        fn test_peek_token() {
            let mut reader = Cursor::new(b"i");
            assert_eq!(reader.peek_token().unwrap(), Token::Int);

            let mut reader = Cursor::new(b"l");
            assert_eq!(reader.peek_token().unwrap(), Token::List);

            let mut reader = Cursor::new(b"d");
            assert_eq!(reader.peek_token().unwrap(), Token::Dict);

            let mut reader = Cursor::new(b"e");
            assert_eq!(reader.peek_token().unwrap(), Token::End);

            let bytes = b"0123456789";
            for byte in bytes {
                let byte_array = [*byte];
                let mut reader = Cursor::new(&byte_array);
                assert_eq!(reader.peek_token().unwrap(), Token::Bytes);
            }

            let mut reader = Cursor::new(b"x");
            assert!(reader.peek_token().is_err());
        }

        #[test]
        fn test_consume_current_token() {
            let mut reader = Cursor::new(b"a");
            assert!(reader.consume_current_token().is_ok());
            assert!(
                !reader.has_tokens_left().unwrap(),
                "Expected no tokens left"
            );
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"ab");
            assert!(reader.consume_current_token().is_ok());
            assert!(
                reader.has_tokens_left().unwrap(),
                "Expected some tokens left"
            );
            assert_eq!(reader.fill_buf().unwrap(), b"b");
        }

        #[test]
        fn test_has_tokens_left() {
            let mut reader = Cursor::new(b"");
            assert!(
                !reader.has_tokens_left().unwrap(),
                "Expected no tokens left"
            );

            let mut reader = Cursor::new(b"a");
            assert!(
                reader.has_tokens_left().unwrap(),
                "Expected some tokens left"
            );

            let mut reader = Cursor::new(b"ab");
            assert!(
                reader.has_tokens_left().unwrap(),
                "Expected some tokens left"
            );
        }

        #[test]
        fn test_read_until_delim() {
            let mut reader = Cursor::new(b"hello world");
            assert!(
                reader.read_until_delim().is_err(),
                "expected error when no delimiter"
            );
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b":hello world");
            assert_eq!(reader.read_until_delim().unwrap(), b":");
            assert_eq!(reader.fill_buf().unwrap(), b"hello world");

            let mut reader = Cursor::new(b"hello world:");
            assert_eq!(reader.read_until_delim().unwrap(), b"hello world:");
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"hello:world");
            assert_eq!(reader.read_until_delim().unwrap(), b"hello:");
            assert_eq!(reader.fill_buf().unwrap(), b"world");
        }

        #[test]
        fn test_read_until_end() {
            let mut reader = Cursor::new(b"12345");
            assert!(
                reader.read_until_end().is_err(),
                "expected error when no end token"
            );
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"12345e");
            assert_eq!(reader.read_until_end().unwrap(), b"12345e");
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"e12345");
            assert_eq!(reader.read_until_end().unwrap(), b"e");
            assert_eq!(reader.fill_buf().unwrap(), b"12345");

            let mut reader = Cursor::new(b"123e45");
            assert_eq!(reader.read_until_end().unwrap(), b"123e");
            assert_eq!(reader.fill_buf().unwrap(), b"45");
        }

        #[test]
        fn test_read_of_size() {
            let mut reader = Cursor::new(b"hello world");
            assert_eq!(reader.read_of_size(5).unwrap(), b"hello");
            assert_eq!(reader.fill_buf().unwrap(), b" world");

            let mut reader = Cursor::new(b"hello world");
            assert_eq!(reader.read_of_size(0).unwrap(), b"");
            assert_eq!(reader.fill_buf().unwrap(), b"hello world");

            let mut reader = Cursor::new(b"hello world");
            assert_eq!(reader.read_of_size(11).unwrap(), b"hello world");
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"hello world");
            assert!(reader.read_of_size(12).is_err());
            assert_eq!(reader.fill_buf().unwrap(), b"hello world");
        }

        #[test]
        fn test_read_i64() {
            let mut reader = Cursor::new(b"12345e");
            assert_eq!(reader.read_i64().unwrap(), 12345);
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"-12345e");
            assert_eq!(reader.read_i64().unwrap(), -12345);
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"12345");
            assert!(reader.read_i64().is_err());
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"12345e12345");
            assert_eq!(reader.read_i64().unwrap(), 12345);
            assert_eq!(reader.fill_buf().unwrap(), b"12345");
        }

        #[test]
        fn test_read_bytes() {
            let mut reader = Cursor::new(b"5:hello");
            assert_eq!(reader.read_bytes().unwrap(), b"hello");
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"0:");
            assert_eq!(reader.read_bytes().unwrap(), b"");
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"5:hello world");
            assert_eq!(reader.read_bytes().unwrap(), b"hello");
            assert_eq!(reader.fill_buf().unwrap(), b" world");

            let mut reader = Cursor::new(b"5:hello5:world");
            assert_eq!(reader.read_bytes().unwrap(), b"hello");
            assert_eq!(reader.fill_buf().unwrap(), b"5:world");

            let mut reader = Cursor::new(b"11:hello world");
            assert_eq!(reader.read_bytes().unwrap(), b"hello world");
            assert_eq!(reader.fill_buf().unwrap(), b"");

            let mut reader = Cursor::new(b"100:hello world");
            assert!(reader.read_bytes().is_err());
            assert_eq!(reader.fill_buf().unwrap(), b"hello world");
        }
    }
}
