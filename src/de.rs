use serde::de;

use crate::error::Error;





pub struct Deserializer<R>
{
    reader: R,
    back_buffer: Vec<u8>,
    flat_buffer: Vec<u8>,
    size: usize,
    pos: usize,
    deep: usize,
    reverse: bool,
}

impl<'de, R> Deserializer<R>
where R: Read<'de>, 
{
    fn new(reader: R) -> Self {
        Deserializer {
            reader,
            back_buffer: Vec::new(),
            flat_buffer: Vec::new(),
            size: 0,
            pos: 0,
            deep: 0,
            reverse: false,
        }
    }




    fn read_header(&mut self) -> Result<(TonTypes, usize/*body_size*/), Error> {

    }
}

impl<'de, R: Read<'de>> de::Deserializer<'de> for &mut Deserializer<R> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        
    }
}
pub trait Read<'de> {
    fn pos(&self) -> usize;
    fn set_pos(&mut self, pos: usize);
    fn next(&mut self) -> Option<u8>;
    fn next_chunk(&mut self, size_hint: usize) -> (&[u8], usize);
}

pub enum TonTypes {
    Undefined,
    None,
    Bool,
    UIntU8,
    UIntU16,
    UIntU32,
    UIntU64,
    IntI8,
    IntI16,
    IntI32,
    IntI64,
    Float16,
    Float32,
    Float64,
    String,
    Bytes,
    UUID,
    DateTime,
    Timestamp,
    Duration,
    Array,
    Object,
    WrappedJSON,
    Meta,
}