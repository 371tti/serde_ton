use std::f32::consts::E;

use serde::de;

use crate::{error::{Error, ErrorCode}, value::prefix::prefix};

use crate::value::prefix::size_prefix::{SIZE_PREFIX_1BYTE, SIZE_PREFIX_2BYTE, SIZE_PREFIX_4BYTE, SIZE_PREFIX_8BYTE};



pub struct Deserializer<R>
{
    reader: R,
    front_buffer: Vec<u8>,
    back_buffer: Vec<u8>,
    standard_pos: usize,
    pos: usize,
    deep: usize,
    reverse: bool,
}

impl<'de, R> Deserializer<R>
where R: Read<'de>, 
{
    fn new(reader: R, pos: usize) -> Self {
        Deserializer {
            reader,
            front_buffer: Vec::new(),
            back_buffer: Vec::new(),
            standard_pos: pos,
            pos: pos,
            deep: 0,
            reverse: false,
        }
    }

    fn read_byte(&mut self, pos: usize) -> Result<u8, Error> {
        if self.standard_pos <= pos && pos < self.standard_pos + self.front_buffer.len() {
            let local_pos = pos - self.standard_pos;
            Ok(self.front_buffer[local_pos])
        } else {
            self.standard_pos = self.reader.get_chunk(&mut self.front_buffer, pos);
            let local_pos = pos - self.standard_pos;
            match self.front_buffer.get(local_pos) {
                Some(byte) => Ok(*byte),
                None => Err(Error::syntax(ErrorCode::NotFoundTarget, 0)),
            }
        }
    }

    fn read_header(&mut self) -> Result<(TonTypes, usize/*body_size*/), Error> {
        const MASK: u8 = 0b111111_00;
        let prefix = self.read_byte(self.pos)?;
        let ton_type = match prefix | MASK {
            prefix::UNDEFINED => return Ok((TonTypes::Undefined, 0)),
            prefix::NONE => return Ok((TonTypes::None, 0)),
            prefix::BOOL => return Ok((TonTypes::Bool, 0)),
            prefix::UINT => {

            }
            prefix::INT => {

            }
            prefix::FLOAT => {

            }
            prefix::STRING => {

            }
            prefix::BYTES => {

            }
            prefix::UUID => return Ok((TonTypes::UUID, 16)),
            prefix::DATETIME => {

            }
            prefix::TIMESTAMP => return Ok((TonTypes::Timestamp, 8)),
            prefix::DURATION => return Ok((TonTypes::Duration, 8)),
            prefix::ARRAY => {

            }
            prefix::OBJECT => {

            }
            prefix::WRAPPED_JSON => {

            }
            prefix::META => {

            }
            _ => return Err(Error::syntax(ErrorCode::InvalidType, self.pos)),
        };

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
    fn get_chunk(&mut self, buffer: &mut Vec<u8>, pos: usize) -> usize/* chunk_head_position */;
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