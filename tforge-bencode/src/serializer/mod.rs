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

impl<'a, 'ser: 'a, W: BencodeWriter> serde::ser::Serializer for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = SerializeMap<'a, 'ser, W>;
    type SerializeStruct = SerializeMap<'a, 'ser, W>;
    type SerializeStructVariant = SerializeMap<'a, 'ser, W>;

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
        Ok(())
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
        Ok(())
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
        Ok(())
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
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + serde::ser::Serialize>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self> {
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
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap::new(self, len.unwrap_or(0)))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(SerializeMap::new(self, len))
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeSeq for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<()> {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeTuple for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<()> {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeTupleStruct for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<()> {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeTupleVariant for &'a mut Serializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<()> {
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

pub struct SerializeMap<'a, 'ser: 'a, W: BencodeWriter> {
    ser: &'a mut Serializer<'ser, W>,
    entries: Vec<(Vec<u8>, Vec<u8>)>,
    cur_key: Option<Vec<u8>>,
}

impl<'a, 'ser, W: BencodeWriter> SerializeMap<'a, 'ser, W> {
    pub fn new(ser: &'a mut Serializer<'ser, W>, len: usize) -> SerializeMap<'a, 'ser, W> {
        SerializeMap {
            ser,
            entries: Vec::with_capacity(len),
            cur_key: None,
        }
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeMap for SerializeMap<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + serde::ser::Serialize>(&mut self, key: &T) -> Result<()> {
        Ok(())
    }

    fn serialize_value<T: ?Sized + serde::ser::Serialize>(&mut self, value: &T) -> Result<()> {
        Ok(())
    }

    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<()>
    where
        K: ?Sized + serde::ser::Serialize,
        V: ?Sized + serde::ser::Serialize,
    {
        Ok(())
    }

    fn end(mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeStruct for SerializeMap<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        Ok(())
    }

    fn end(mut self) -> Result<()> {
        Ok(())
    }
}

impl<'a, 'ser, W: BencodeWriter> serde::ser::SerializeStructVariant for SerializeMap<'a, 'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + serde::ser::Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        Ok(())
    }

    fn end(mut self) -> Result<()> {
        Ok(())
    }
}
