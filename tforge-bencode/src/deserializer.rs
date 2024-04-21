use crate::{
    decoder,
    error::{Error, Result},
};
use std::{io::BufRead, str};

pub struct Deserializer<'de, R: BufRead> {
    decoder: decoder::Decoder<'de, R>,
}

impl<'de, R: BufRead> Deserializer<'de, R> {
    pub fn from_buffer(reader: &'de mut R) -> Self {
        Deserializer {
            decoder: decoder::Decoder::new(reader),
        }
    }
}

pub fn from_buffer<'a, R: BufRead, T>(reader: &'a mut R) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_buffer(reader);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.decoder.has_data_left()? {
        Err(Error::TrailingData)
    } else {
        Ok(t)
    }
}

impl<'de, R: BufRead> serde::de::Deserializer<'de> for &mut Deserializer<'de, R> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: serde::de::Visitor<'de>,
    {
        self.decoder
            .next()
            .ok_or(Error::InvalidSyntax("unexpected end"))?
            .and_then(|node| match node {
                decoder::BencodeNode::Int(i) => visitor.visit_i64(i),
                decoder::BencodeNode::Bytes(b) => visitor.visit_bytes(b.as_ref()),
                decoder::BencodeNode::List => visitor.visit_seq(DeserializerAccess::new(self)),
                decoder::BencodeNode::Map => visitor.visit_map(DeserializerAccess::new(self)),
                decoder::BencodeNode::End => Err(Error::InvalidSyntax("unexpected end")),
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
        visitor.visit_str(
            &self
                .decoder
                .next()
                .ok_or(Error::InvalidSyntax("unexpected end"))?
                .and_then(|node| match node {
                    decoder::BencodeNode::Bytes(bytes) => {
                        String::from_utf8(bytes).map_err(Error::from_utf8)
                    }
                    _ => Err(Error::InvalidSyntax("expected bytes")),
                })?,
        )
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
        self.decoder
            .next()
            .ok_or(Error::InvalidSyntax("unexpected end"))?
            .and_then(|node| match node {
                decoder::BencodeNode::List => {
                    visitor.visit_seq(DeserializerAccess::new_with_len(self, size))
                }
                _ => Err(Error::InvalidSyntax("expected list")),
            })
    }
}

struct DeserializerAccess<'a, 'de: 'a, R: BufRead> {
    deserializer: &'a mut Deserializer<'de, R>,
    len: Option<usize>,
}

impl<'a, 'de, R: BufRead> DeserializerAccess<'a, 'de, R> {
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

impl<'a, 'de, R: BufRead> serde::de::SeqAccess<'de> for DeserializerAccess<'a, 'de, R> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        let result = match self.deserializer.decoder.try_consume_end_token() {
            Some(Ok(())) => return Ok(None),
            Some(Err(err)) => return Err(err),
            None => {
                if let Some(len) = self.len {
                    if len == 0 {
                        return Err(Error::InvalidSyntax("expected end"));
                    }
                }
                seed.deserialize(&mut *self.deserializer).map(Some)
            }
        };

        if let Some(len) = self.len {
            self.len = Some(len - 1);
        }

        result
    }
}

impl<'a, 'de, R: BufRead> serde::de::MapAccess<'de> for DeserializerAccess<'a, 'de, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.deserializer.decoder.try_consume_end_token() {
            Some(Ok(())) => return Ok(None),
            Some(Err(err)) => return Err(err),
            None => seed.deserialize(&mut *self.deserializer).map(Some),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.deserializer)
    }
}

impl<'a, 'de, R: BufRead> serde::de::EnumAccess<'de> for DeserializerAccess<'a, 'de, R> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self)>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Ok((seed.deserialize(&mut *self.deserializer)?, self))
    }
}

impl<'a, 'de, R: BufRead> serde::de::VariantAccess<'de> for DeserializerAccess<'a, 'de, R> {
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
            .decoder
            .next()
            .ok_or(Error::InvalidSyntax("unexpected end"))?
            .and_then(|node| match node {
                decoder::BencodeNode::List => visitor.visit_seq(DeserializerAccess::new_with_len(
                    &mut *self.deserializer,
                    len,
                )),
                _ => Err(Error::InvalidSyntax("expected list")),
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
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_deserialize_i64() {
        let mut reader = BufReader::new(Cursor::new(b"i123e"));
        let result: i64 = from_buffer(&mut reader).unwrap();
        assert_eq!(result, 123);
    }

    #[test]
    fn test_deserialize_negative_i64() {
        let mut reader = BufReader::new(Cursor::new(b"i-123e"));
        let result: i64 = from_buffer(&mut reader).unwrap();
        assert_eq!(result, -123);
    }

    #[test]
    fn test_deserialize_i64_errors_with_trailing_data() {
        let mut reader = BufReader::new(Cursor::new(b"i123e123"));
        let result: Result<i64> = from_buffer(&mut reader);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::TrailingData));
    }

    #[test]
    fn test_deserialize_bytes_to_string() {
        let mut reader = BufReader::new(Cursor::new(b"4:spam"));
        let result: String = from_buffer(&mut reader).unwrap();
        assert_eq!(result, "spam");
    }

    #[test]
    fn test_deserialize_string_list() {
        let mut reader = BufReader::new(Cursor::new(b"l3:foo3:bare"));
        let result: Vec<String> = from_buffer(&mut reader).unwrap();
        assert_eq!(result, vec!["foo".to_string(), "bar".to_string()]);
    }

    #[test]
    fn test_deserialize_i64_list() {
        let mut reader = BufReader::new(Cursor::new(b"li123ei456ee"));
        let result: Vec<i64> = from_buffer(&mut reader).unwrap();
        assert_eq!(result, vec![123, 456]);
    }

    #[test]
    fn test_deserialize_i64_list_inside_list() {
        let mut reader = BufReader::new(Cursor::new(b"lli123ei456eee"));
        let result: Vec<Vec<i64>> = from_buffer(&mut reader).unwrap();
        assert_eq!(result, vec![vec![123, 456]]);
    }

    // #[test]
    // fn test_deserialize_i64_tuple() {
    //     let mut reader = BufReader::new(Cursor::new(b"li123ei456ee"));
    //     let result: [i64; 2] = from_buffer(&mut reader).unwrap();
    //     assert_eq!(result, [123, 456]);
    // }

    #[test]
    fn test_deserialize_map() {
        let mut reader = BufReader::new(Cursor::new(b"d3:foo3:bare"));
        let result: std::collections::HashMap<String, String> = from_buffer(&mut reader).unwrap();
        let mut expected = std::collections::HashMap::new();
        expected.insert("foo".to_string(), "bar".to_string());
        assert_eq!(result, expected);
    }
}
