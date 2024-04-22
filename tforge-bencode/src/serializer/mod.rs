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
