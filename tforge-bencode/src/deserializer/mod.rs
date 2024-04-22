use crate::{
    error::{Error, Result},
    reader::BencodeReader,
    tokens::Token,
};

pub struct Deserializer<'de, R: BencodeReader> {
    reader: &'de mut R,
}

impl<'de, R: BencodeReader> Deserializer<'de, R> {
    pub fn from_reader(reader: &'de mut R) -> Self {
        Deserializer { reader }
    }
}

pub fn from_reader<'a, R: BencodeReader, T>(reader: &'a mut R) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_reader(reader);

    T::deserialize(&mut deserializer).and_then(|result| {
        deserializer
            .reader
            .has_tokens_left()
            .and_then(|has_tokens_left| {
                if has_tokens_left {
                    Err(Error::from_trailing_data(deserializer.reader.read()?))
                } else {
                    Ok(result)
                }
            })
    })
}

impl<'de, R: BencodeReader> serde::de::Deserializer<'de> for &mut Deserializer<'de, R> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.reader.peek_token().and_then(|token| match token {
            Token::Int => {
                self.reader.consume_current_token()?;
                visitor.visit_i64(self.reader.read_i64()?)
            }
            Token::Bytes => visitor.visit_bytes(self.reader.read_bytes()?.as_ref()),
            Token::List => {
                self.reader.consume_current_token()?;
                visitor.visit_seq(DeserializerAccess::new(self))
            }
            Token::Dict => {
                self.reader.consume_current_token()?;
                visitor.visit_map(DeserializerAccess::new(self))
            }
            Token::End => Err(Error::from_syntax("Unexpected end")),
        })
    }

    serde::forward_to_deserialize_any! {
        bool char i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 unit bytes byte_buf seq map unit_struct
        tuple_struct ignored_any struct
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_some(self)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        visitor.visit_enum(DeserializerAccess::new(self))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.reader.peek_token().and_then(|token| match token {
            Token::Bytes => {
                let string = self.reader.read_string()?;
                visitor.visit_str(&string)
            }
            _ => Err(Error::ExpectedBytes),
        })
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_tuple<V>(self, size: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.reader.peek_token().and_then(|token| match token {
            Token::List => {
                self.reader.consume_current_token()?;
                visitor.visit_seq(DeserializerAccess::new_with_len(self, size))
            }
            _ => Err(Error::ExpectedList),
        })
    }
}

struct DeserializerAccess<'a, 'de: 'a, R: BencodeReader> {
    deserializer: &'a mut Deserializer<'de, R>,
    len: Option<usize>,
}

impl<'a, 'de, R: BencodeReader> DeserializerAccess<'a, 'de, R> {
    fn new(deserializer: &'a mut Deserializer<'de, R>) -> Self {
        DeserializerAccess {
            deserializer,
            len: None,
        }
    }

    fn new_with_len(deserializer: &'a mut Deserializer<'de, R>, len: usize) -> Self {
        DeserializerAccess {
            deserializer,
            len: Some(len),
        }
    }
}

impl<'a, 'de, R: BencodeReader> serde::de::SeqAccess<'de> for DeserializerAccess<'a, 'de, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let result = self
            .deserializer
            .reader
            .peek_token()
            .and_then(|token| match token {
                Token::End => {
                    self.deserializer.reader.consume_current_token()?;
                    Ok(None)
                }
                _ => seed.deserialize(&mut *self.deserializer).map(Some),
            });

        if let Some(len) = self.len {
            let len = len - 1;
            self.len = Some(len);
            if len == 0 {
                self.deserializer.reader.peek_token().and_then(|token| {
                    if token == Token::End {
                        self.deserializer.reader.consume_current_token()?;
                        Ok(())
                    } else {
                        Err(Error::ExpectedEnd)
                    }
                })?;
            }
        }

        result
    }
}

impl<'a, 'de, R: BencodeReader> serde::de::MapAccess<'de> for DeserializerAccess<'a, 'de, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        self.deserializer
            .reader
            .peek_token()
            .and_then(|token| match token {
                Token::End => {
                    self.deserializer.reader.consume_current_token()?;
                    Ok(None)
                }
                _ => seed.deserialize(&mut *self.deserializer).map(Some),
            })
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

impl<'a, 'de, R: BencodeReader> serde::de::EnumAccess<'de> for DeserializerAccess<'a, 'de, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.deserializer)?, self))
    }
}

impl<'a, 'de, R: BencodeReader> serde::de::VariantAccess<'de> for DeserializerAccess<'a, 'de, R> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserializer
            .reader
            .peek_token()
            .and_then(|token| match token {
                Token::List => {
                    self.deserializer.reader.consume_current_token()?;
                    visitor.visit_seq(DeserializerAccess::new_with_len(
                        &mut *self.deserializer,
                        len,
                    ))
                }
                _ => Err(Error::ExpectedList),
            })
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        serde::de::Deserializer::deserialize_tuple(&mut *self.deserializer, fields.len(), visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_deserialize_i64() {
        let mut reader = BufReader::new(Cursor::new(b"i123e"));
        let result: i64 = from_reader(&mut reader).unwrap();
        assert_eq!(result, 123);
    }

    #[test]
    fn test_deserialize_negative_i64() {
        let mut reader = BufReader::new(Cursor::new(b"i-123e"));
        let result: i64 = from_reader(&mut reader).unwrap();
        assert_eq!(result, -123);
    }

    #[test]
    fn test_deserialize_i64_errors_with_trailing_data() {
        let mut reader = BufReader::new(Cursor::new(b"i123e123"));
        let result: Result<i64> = from_reader(&mut reader);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::TrailingData(_)));
    }

    #[test]
    fn test_deserialize_bytes_to_string() {
        let mut reader = BufReader::new(Cursor::new(b"4:spam"));
        let result: String = from_reader(&mut reader).unwrap();
        assert_eq!(result, "spam");
    }

    #[test]
    fn test_deserialize_string_list() {
        let mut reader = BufReader::new(Cursor::new(b"l3:foo3:bare"));
        let result: Vec<String> = from_reader(&mut reader).unwrap();
        assert_eq!(result, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn test_deserialize_i64_list() {
        let mut reader = BufReader::new(Cursor::new(b"li123ei456ee"));
        let result: Vec<i64> = from_reader(&mut reader).unwrap();
        assert_eq!(result, vec![123, 456]);
    }

    #[test]
    fn test_deserialize_i64_list_inside_list() {
        let mut reader = BufReader::new(Cursor::new(b"lli123ei456eee"));
        let result: Vec<Vec<i64>> = from_reader(&mut reader).unwrap();
        assert_eq!(result, vec![vec![123, 456]]);
    }

    #[test]
    fn test_deserialize_i64_tuple() {
        let mut reader = BufReader::new(Cursor::new(b"li123ei456ee"));
        let result: [i64; 2] = from_reader(&mut reader).unwrap();
        assert_eq!(result, [123, 456]);
    }

    #[test]
    fn test_deserialize_map() {
        let mut reader = BufReader::new(Cursor::new(b"d3:foo3:bare"));
        let result: std::collections::HashMap<String, String> = from_reader(&mut reader).unwrap();
        let mut expected = std::collections::HashMap::new();
        expected.insert("foo".to_string(), "bar".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_option_some() {
        let mut reader = BufReader::new(Cursor::new(b"i123e"));
        let result: Option<i64> = from_reader(&mut reader).unwrap();
        assert_eq!(result, Some(123));
    }

    #[derive(Deserialize, PartialEq, Debug)]
    struct TestStruct {
        int_prop: i64,
        string_prop: String,
    }

    #[test]
    fn test_deserialize_struct() {
        let mut reader = BufReader::new(Cursor::new(b"d8:int_propi123e11:string_prop3:baze"));
        let result: TestStruct = from_reader(&mut reader).unwrap();
        assert_eq!(
            result,
            TestStruct {
                int_prop: 123,
                string_prop: "baz".to_string(),
            }
        );
    }
}
