use crate::{
    error::{Error, Result},
    writer::BencodeWriter,
};

pub struct Serializer<'ser, W: BencodeWriter> {
    writer: &'ser mut W,
}

impl<'ser, W: BencodeWriter> Serializer<'ser, W> {
    pub fn from_writer(writer: &'ser mut W) -> Self {
        Serializer { writer }
    }
}

pub fn from_writer<W: BencodeWriter>(writer: &'_ mut W) -> Serializer<'_, W> {
    Serializer::from_writer(writer)
}

impl<'a, 'ser: 'a, W: BencodeWriter> serde::ser::Serializer for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        self.writer.write_signed_integer(value)
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        self.writer.write_unsigned_integer(value)
    }

    fn serialize_f32(self, _value: f32) -> Result<()> {
        Err(Error::from_unsupported_type::<f32>())
    }

    fn serialize_f64(self, _value: f64) -> Result<()> {
        Err(Error::from_unsupported_type::<f64>())
    }

    fn serialize_char(self, value: char) -> Result<()> {
        let mut buffer = [0; 4];
        self.serialize_bytes(value.encode_utf8(&mut buffer).as_bytes())
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        self.serialize_bytes(value.as_bytes())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.writer.write_bytes(value)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized + serde::ser::Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + serde::ser::Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> {
        self.writer.write_dict_start()?;
        self.serialize_bytes(variant.as_bytes())?;
        value.serialize(&mut *self)?;
        self.writer.write_end()?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + serde::ser::Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self> {
        self.writer.write_list_start()?;
        Ok(self)
    }

    fn serialize_tuple(self, size: usize) -> Result<Self> {
        self.serialize_seq(Some(size))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.writer.write_dict_start()?;
        self.serialize_bytes(variant.as_bytes())?;
        self.writer.write_list_start()?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.writer.write_dict_start()?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(None)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!("struct_variant")
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeSeq for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.writer.write_end()?;
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeTuple for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + serde::ser::Serialize>(&mut self, _value: &T) -> Result<()> {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeTupleStruct for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(&mut self, _value: &T) -> Result<()> {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeTupleVariant for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(&mut self, _value: &T) -> Result<()> {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeMap for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + serde::ser::Serialize>(&mut self, key: &T) -> Result<()> {
        key.serialize(&mut **self)?;
        Ok(())
    }

    fn serialize_value<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: ?Sized + serde::ser::Serialize,
        V: ?Sized + serde::ser::Serialize,
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.writer.write_end()?;
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeStruct for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        serde::ser::SerializeMap::serialize_entry(self, key, value)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.writer.write_end()?;
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeStructVariant
    for &'a mut Serializer<'ser, W>
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()> {
        unimplemented!("struct_variant field")
    }

    fn end(self) -> Result<()> {
        unimplemented!("struct_variant end")
    }
}

#[cfg(test)]
mod tests {
    use crate::serializer::Serializer;
    use serde::Serialize;
    use std::io::Cursor;

    #[test]
    fn test_serialize_signed_integer() {
        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        (-42).serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"i-42e");
    }

    #[test]
    fn test_serialize_unsigned_integer() {
        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        42.serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"i42e");
    }

    #[test]
    fn test_serialize_string() {
        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        "hello".serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"5:hello");
    }

    #[test]
    fn test_serialize_string_list() {
        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        vec!["hello", "world"].serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"l5:hello5:worlde");
    }

    #[test]
    fn test_serialize_i64_list() {
        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        vec![42, -42].serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"li42ei-42ee");
    }

    #[test]
    fn test_serialize_map() {
        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        let mut map = std::collections::BTreeMap::new();
        map.insert("hello", "world");
        map.serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"d5:hello5:worlde");
    }

    #[test]
    fn test_serialize_struct() {
        #[derive(Serialize)]
        struct Test {
            hello: String,
            world: String,
        }

        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        let test = Test {
            hello: "world".to_string(),
            world: "hello".to_string(),
        };
        test.serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"d5:hello5:world5:world5:helloe");
    }

    #[test]
    fn test_serialize_option_string() {
        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        let value: Option<&str> = Some("hello");
        value.serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"5:hello");
    }

    #[test]
    fn test_serialize_option_none() {
        let mut cursor = Cursor::new(Vec::new());
        let mut serializer = Serializer::from_writer(&mut cursor);

        let value: Option<&str> = None;
        value.serialize(&mut serializer).unwrap();
        assert_eq!(cursor.into_inner(), b"");
    }
}
