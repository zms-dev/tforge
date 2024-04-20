use crate::{
    error::{Error, Result},
    tokens,
};
use std::io::BufRead;

#[derive(Debug, Eq, PartialEq)]
pub enum BencodeNode {
    Int(i64),
    Bytes(Vec<u8>),
    List,
    Map,
    End,
}

pub struct Decoder<'a, R: BufRead> {
    reader: &'a mut R,
}

impl<'a, R: BufRead> Decoder<'a, R> {
    pub fn new(reader: &'a mut R) -> Self {
        Decoder { reader }
    }
}

impl<'a, R: BufRead> Decoder<'a, R> {
    fn peek_byte(&mut self) -> Result<u8> {
        let buf = self.reader.fill_buf().map_err(Error::from_io)?;
        if buf.len() == 0 {
            return Err(Error::EmptyBuffer);
        }
        Ok(buf[0])
    }

    fn consume_byte(&mut self, expected: u8) -> Result<()> {
        let mut buf = [0; 1];
        self.reader
            .read_exact(&mut buf)
            .map_err(|err| Error::from_io(err))?;
        if buf[0] != expected {
            return Err(Error::expected_byte(expected));
        }
        Ok(())
    }

    fn parse_integer(&mut self) -> Result<i64> {
        let mut buf = Vec::new();
        self.reader
            .read_until(tokens::TOKEN_END, &mut buf)
            .map_err(Error::from_io)?;

        let int_str = String::from_utf8(buf).map_err(|err| Error::from_utf8(err))?;
        let parsed_int = &int_str[..int_str.len() - 1]
            .parse::<i64>()
            .map_err(|err| Error::from_parse_int(err))?;

        eprintln!("Parsed integer: {}", parsed_int);

        Ok(*parsed_int)
    }

    fn parse_bytes(&mut self) -> Result<Vec<u8>> {
        let mut length_buf = Vec::new();
        self.reader
            .read_until(tokens::TOKEN_DELIM, &mut length_buf)
            .map_err(Error::from_io)?;

        let lenth_str = String::from_utf8(length_buf).map_err(|err| Error::from_utf8(err))?;
        let length_int = &lenth_str[..lenth_str.len() - 1]
            .parse::<u64>()
            .map_err(|err| Error::from_parse_int(err))?;

        let mut buf = vec![0; *length_int as usize];
        self.reader
            .read_exact(&mut buf)
            .map_err(|err| Error::from_io(err))?;

        Ok(buf)
    }

    fn parse(&mut self) -> Result<BencodeNode> {
        self.peek_byte().and_then(|byte| match byte {
            tokens::TOKEN_INTEGER => {
                self.consume_byte(tokens::TOKEN_INTEGER)?;
                Ok(BencodeNode::Int(self.parse_integer()?))
            }
            tokens::TOKEN_LIST => {
                self.consume_byte(tokens::TOKEN_LIST)?;
                Ok(BencodeNode::List)
            }
            tokens::TOKEN_DICT => {
                self.consume_byte(tokens::TOKEN_DICT)?;
                Ok(BencodeNode::Map)
            }
            tokens::TOKEN_END => {
                self.consume_byte(tokens::TOKEN_END)?;
                Ok(BencodeNode::End)
            }
            b'0'..=b'9' => Ok(BencodeNode::Bytes(self.parse_bytes()?)),
            c => Err(Error::InvalidToken(c as char)),
        })
    }

    pub fn try_consume_end_token(&mut self) -> Option<Result<()>> {
        match self.peek_byte() {
            Ok(tokens::TOKEN_END) => Some(self.consume_byte(tokens::TOKEN_END)),
            Ok(_) => None,
            Err(err) => Some(Err(err)),
        }
    }

    pub fn has_data_left(&mut self) -> Result<bool> {
        eprintln!(
            "Checking if there is data left: {:?}",
            String::from_utf8_lossy(self.reader.fill_buf().map_err(Error::from_io)?),
        );
        self.reader.has_data_left().map_err(Error::from_io)
    }
}

impl<'a, R: BufRead> Iterator for Decoder<'a, R> {
    type Item = Result<BencodeNode>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.has_data_left() {
            Err(err) => return Some(Err(err)),
            Ok(false) => return None,
            Ok(true) => Some(self.parse()),
        }
    }
}
