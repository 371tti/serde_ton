use std::{f32::consts::E, io::{self, BufReader}};
use serde_cbor::de::Read;
use serde::{de, forward_to_deserialize_any};

use serde_cbor::de::Deserializer;

use crate::{error::{Error, ErrorCode}, traits::reader::Read, value::prefix::prefix};

use crate::value::prefix::size_prefix::{SIZE_PREFIX_1BYTE, SIZE_PREFIX_2BYTE, SIZE_PREFIX_4BYTE, SIZE_PREFIX_8BYTE};


/// RTON のデシリアライザ
/// 後ろから読み込む必要がある
/// ファイルとかだとseek位置を動かしてバッファリングすべき
/// バッファリングは外部ですべき - 柔軟性
/// seekトレイとが実装されてる標準型が少ないのが問題
/// 楽に使うために変換できるように
/// 
pub struct ReverseDeserializer<R>
{
    reader: io::Cursor<R>,
    deep: usize,
}

impl ReverseDeserializer<Vec<u8>>
{
    pub fn from_vector(vec: Vec<u8>) -> Result<Self, Error> {
        let reader = io::Cursor::new(vec);
        Ok(Self { reader, deep: 0 })
    }
}

impl<R> ReverseDeserializer<R>
{
    pub fn new(reader: R) -> Result<Self, Error> {
        let reader = io::Cursor::new(reader);
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> R {
        self.reader.into_inner()
    }
}

impl<R> ReverseDeserializer<R> {
    pub fn read_header() -> Result<(size, prefix_val), Error> {

    }
}

impl<'de, 'a, R> de::Deserializer<'de> for &'a mut ReverseDeserializer<R>
where
    R: io::Read + io::Seek,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de> {
        if self.eof() {
            return visitor.visit_none();
        }

    }
    
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = visitor;
        Err(de::Error::custom("i128 is not supported"))
    }
    
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let _ = visitor;
        Err(de::Error::custom("u128 is not supported"))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes byte_buf option unit
        unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
    
    fn is_human_readable(&self) -> bool {
        true
    }
}
