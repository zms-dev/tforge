use crate::value::Value;
use serde::{ser, Serialize};

#[derive(PartialEq)]
pub enum Error {
    Custom(String),
}

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub struct Serializer {}

impl Serializer {
    pub fn new() -> Self {
        Serializer {}
    }
}

impl<'a> ser::Serializer for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = SeqSerializer<'a>;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(v))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::None())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::None())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ser::Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(len) => Ok(SeqSerializer::new_with_capacity(self, len)),
            None => Ok(SeqSerializer::new(self)),
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(SeqSerializer::new_with_capacity(self, len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(self)
    }
}

pub struct SeqSerializer<'a> {
    ser: &'a Serializer,
    values: Vec<Value>,
}

impl<'a> SeqSerializer<'a> {
    pub fn new(ser: &'a Serializer) -> Self {
        SeqSerializer {
            ser,
            values: Vec::new(),
        }
    }

    pub fn new_with_capacity(ser: &'a Serializer, capacity: usize) -> Self {
        SeqSerializer {
            ser,
            values: Vec::with_capacity(capacity),
        }
    }
}

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = value.serialize(self.ser)?;
        self.values.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(self.values))
    }
}

impl<'a> ser::SerializeTuple for SeqSerializer<'a> {
    type Ok = Value;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let value = value.serialize(self.ser)?;
        self.values.push(value);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Value::from(self.values))
    }
}

impl<'a> ser::SerializeTupleStruct for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleVariant for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStruct for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a Serializer {
    type Ok = Value;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }
}

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize,
{
    let serializer = Serializer::new();
    value.serialize(&serializer).map(|op| op.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serializer as _;

    #[test]
    fn test_serialize_bool() {
        let ser = &Serializer::new();
        let result = ser.serialize_bool(true);
        assert_eq!(result, Ok(Value::String("true".to_string())));
    }

    #[test]
    fn test_serialize_i8() {
        let ser = &Serializer::new();
        let result = ser.serialize_i8(-42i8);
        assert_eq!(result, Ok(Value::String("-42".to_string())));
    }

    #[test]
    fn test_serialize_i16() {
        let ser = &Serializer::new();
        let result = ser.serialize_i16(-42i16);
        assert_eq!(result, Ok(Value::String("-42".to_string())));
    }

    #[test]
    fn test_serialize_i32() {
        let ser = &Serializer::new();
        let result = ser.serialize_i32(-42i32);
        assert_eq!(result, Ok(Value::String("-42".to_string())));
    }

    #[test]
    fn test_serialize_i64() {
        let ser = &Serializer::new();
        let result = ser.serialize_i64(-42i64);
        assert_eq!(result, Ok(Value::String("-42".to_string())));
    }

    #[test]
    fn test_serialize_u8() {
        let ser = &Serializer::new();
        let result = ser.serialize_u8(42u8);
        assert_eq!(result, Ok(Value::String("42".to_string())));
    }

    #[test]
    fn test_serialize_u16() {
        let ser = &Serializer::new();
        let result = ser.serialize_u16(42u16);
        assert_eq!(result, Ok(Value::String("42".to_string())));
    }

    #[test]
    fn test_serialize_u32() {
        let ser = &Serializer::new();
        let result = ser.serialize_u32(42u32);
        assert_eq!(result, Ok(Value::String("42".to_string())));
    }

    #[test]
    fn test_serialize_u64() {
        let ser = &Serializer::new();
        let result = ser.serialize_u64(42u64);
        assert_eq!(result, Ok(Value::String("42".to_string())));
    }

    #[test]
    fn test_serialize_f32() {
        let ser = &Serializer::new();
        let result = ser.serialize_f32(42.5f32);
        assert_eq!(result, Ok(Value::String("42.5".to_string())));
    }

    #[test]
    fn test_serialize_f64() {
        let ser = &Serializer::new();
        let result = ser.serialize_f64(42.5f64);
        assert_eq!(result, Ok(Value::String("42.5".to_string())));
    }

    #[test]
    fn test_serialize_char() {
        let ser = &Serializer::new();
        let result = ser.serialize_char('a');
        assert_eq!(result, Ok(Value::String("a".to_string())));
    }

    #[test]
    fn test_serialize_char_emojii() {
        let ser = &Serializer::new();
        let result = ser.serialize_char('😀');
        assert_eq!(result, Ok(Value::String("%F0%9F%98%80".to_string())));
    }

    #[test]
    fn test_serialize_char_space() {
        let ser = &Serializer::new();
        let result = ser.serialize_char(' ');
        assert_eq!(result, Ok(Value::String("%20".to_string())));
    }

    #[test]
    fn test_serialize_str() {
        let ser = &Serializer::new();
        let result = ser.serialize_str("hello world");
        assert_eq!(result, Ok(Value::String("hello%20world".to_string())));
    }

    #[test]
    fn test_serialize_bytes() {
        let ser = &Serializer::new();
        let result = ser.serialize_bytes(b"hello world");
        assert_eq!(
            result,
            Ok(Value::String(
                "%68%65%6C%6C%6F%20%77%6F%72%6C%64".to_string()
            ))
        );
    }

    #[test]
    fn test_serialize_none() {
        let ser = &Serializer::new();
        let result = ser.serialize_none();
        assert_eq!(result, Ok(Value::None()));
    }

    #[test]
    fn test_serialize_some() {
        let ser = &Serializer::new();
        let result = ser.serialize_some(&42);
        assert_eq!(result, Ok(Value::String("42".to_string())));
    }

    #[test]
    fn test_serialize_unit() {
        let ser = &Serializer::new();
        let result = ser.serialize_unit();
        assert_eq!(result, Ok(Value::None()));
    }

    #[test]
    fn test_serialize_unit_struct() {
        let ser = &Serializer::new();
        let result = ser.serialize_unit_struct("Unit");
        assert_eq!(result, Ok(Value::None()));
    }
}
