use std::{fs::File, io::{self, Read, Seek, SeekFrom}};

use serde::{de::Visitor, Deserialize, Deserializer};

use crate::{error::{Error, ErrorCode}, traits::reader::{IOReader, Reader, SliceReader, VecReader}, value::prefix::{prefix, size_prefix}};




/// RTON のデシリアライザ
/// 後ろから読み込む必要がある
/// ファイルとかだとseek位置を動かしてバッファリングすべき
/// バッファリングは外部ですべき - 柔軟性
/// seekトレイとが実装されてる標準型が少ないのが問題
/// 楽に使うために変換できるように
/// 
pub struct ReverseDeserializer<R>
where R: Reader,
{
    reader: R,
    deep: u64,
}

impl<'a> ReverseDeserializer<SliceReader<'a>> 
{
    pub fn from_slice(slice: &'a [u8]) -> Result<Self, io::Error> {
        let reader = SliceReader::new(slice);
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> &'a [u8] {
        self.reader.into_inner()
    }
}

impl<'a> ReverseDeserializer<VecReader<'a>> 
{
    pub fn from_vec(vec: &'a Vec<u8>) -> Result<Self, io::Error> {
        let reader = VecReader::new(vec);
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> &'a Vec<u8> {
        self.reader.into_inner()
    }
    
}

impl ReverseDeserializer<IOReader<File>> 
{
    pub fn from_file(file: File) -> Result<Self,io:: Error> {
        let reader = IOReader::new(file);
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> File {
        self.reader.into_inner()
    }
}

impl<R> ReverseDeserializer<R>
where R: Reader,
{
    pub fn new(reader: R) -> Result<Self, io::Error> {
        Ok(Self { reader, deep: 0 })
    }

    fn now_pos(&mut self) -> Result<u64, io::Error> {
        self.reader.stream_position()
    }

    fn get_size(reader: &mut R) -> io::Result<u64> {
        let current = reader.stream_position()?;
        let end = reader.seek(io::SeekFrom::End(0))?;
        reader.seek(io::SeekFrom::Start(current))?;
        Ok(end)
    }

    fn next(&mut self) -> Result<u8, io::Error> {
        if let Some(i) = self.reader.peek()? {
            Ok(i)
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more data to read"))
        }
    }

    fn prev(&mut self) -> Result<u8, io::Error> {
        if let Some(i) = self.reader.prev()? {
            Ok(i)
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more data to read"))
        }
    }

    fn peek(&mut self) -> Result<u8, io::Error> {
        if let Some(i) = self.reader.peek()? {
            Ok(i)
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more data to read"))
        }
    }
}

impl<'de, R> ReverseDeserializer<R>
where
    R: Reader,
{
    fn perse_value<V>(&mut self, visitor: V) -> Result<V::Value, Error>
    where
        V: Visitor<'de>,
    {
        const UNDEFINED: u8 = prefix::UNDEFINED;
        const NONE: u8 = prefix::NONE;
        const BOOLF: u8 = prefix::BOOL | size_prefix::SIZE_PREFIX_1BYTE;
        const BOOLT: u8 = prefix::BOOL | size_prefix::SIZE_PREFIX_2BYTE;
        const UINT8: u8 = prefix::UINT | size_prefix::SIZE_PREFIX_1BYTE;
        const UINT16: u8 = prefix::UINT | size_prefix::SIZE_PREFIX_2BYTE;
        const UINT32: u8 = prefix::UINT | size_prefix::SIZE_PREFIX_4BYTE;
        const UINT64: u8 = prefix::UINT | size_prefix::SIZE_PREFIX_8BYTE;
        const INT8: u8 = prefix::INT | size_prefix::SIZE_PREFIX_1BYTE;
        const INT16: u8 = prefix::INT | size_prefix::SIZE_PREFIX_2BYTE;
        const INT32: u8 = prefix::INT | size_prefix::SIZE_PREFIX_4BYTE;
        const INT64: u8 = prefix::INT | size_prefix::SIZE_PREFIX_8BYTE;
        const FLOAT16: u8 = prefix::FLOAT | size_prefix::SIZE_PREFIX_2BYTE;
        const FLOAT32: u8 = prefix::FLOAT | size_prefix::SIZE_PREFIX_4BYTE;
        const FLOAT64: u8 = prefix::FLOAT | size_prefix::SIZE_PREFIX_8BYTE;
        const STRING8: u8 = prefix::STRING | size_prefix::SIZE_PREFIX_1BYTE;
        const STRING16: u8 = prefix::STRING | size_prefix::SIZE_PREFIX_2BYTE;
        const STRING32: u8 = prefix::STRING | size_prefix::SIZE_PREFIX_4BYTE;
        const STRING64: u8 = prefix::STRING | size_prefix::SIZE_PREFIX_8BYTE;
        const BYTES8: u8 = prefix::BYTES | size_prefix::SIZE_PREFIX_1BYTE;
        const BYTES16: u8 = prefix::BYTES | size_prefix::SIZE_PREFIX_2BYTE;
        const BYTES32: u8 = prefix::BYTES | size_prefix::SIZE_PREFIX_4BYTE;
        const BYTES64: u8 = prefix::BYTES | size_prefix::SIZE_PREFIX_8BYTE;
        const UUID: u8 = prefix::UUID;
        const DATETIME: u8 = prefix::DATETIME;
        const TIMESTAMP: u8 = prefix::TIMESTAMP;
        const DURATION: u8 = prefix::DURATION;
        const ARRAY8: u8 = prefix::ARRAY | size_prefix::SIZE_PREFIX_1BYTE;
        const ARRAY16: u8 = prefix::ARRAY | size_prefix::SIZE_PREFIX_2BYTE;
        const ARRAY32: u8 = prefix::ARRAY | size_prefix::SIZE_PREFIX_4BYTE;
        const ARRAY64: u8 = prefix::ARRAY | size_prefix::SIZE_PREFIX_8BYTE;
        const OBJECT8: u8 = prefix::OBJECT | size_prefix::SIZE_PREFIX_1BYTE;
        const OBJECT16: u8 = prefix::OBJECT | size_prefix::SIZE_PREFIX_2BYTE;
        const OBJECT32: u8 = prefix::OBJECT | size_prefix::SIZE_PREFIX_4BYTE;
        const OBJECT64: u8 = prefix::OBJECT | size_prefix::SIZE_PREFIX_8BYTE;
        const WRAPPED_JSON8: u8 = prefix::WRAPPED_JSON | size_prefix::SIZE_PREFIX_1BYTE;
        const WRAPPED_JSON16: u8 = prefix::WRAPPED_JSON | size_prefix::SIZE_PREFIX_2BYTE;
        const WRAPPED_JSON32: u8 = prefix::WRAPPED_JSON | size_prefix::SIZE_PREFIX_4BYTE;
        const WRAPPED_JSON64: u8 = prefix::WRAPPED_JSON | size_prefix::SIZE_PREFIX_8BYTE;
        const META8: u8 = prefix::META | size_prefix::SIZE_PREFIX_1BYTE;
        const META16: u8 = prefix::META | size_prefix::SIZE_PREFIX_2BYTE;
        const META32: u8 = prefix::META | size_prefix::SIZE_PREFIX_4BYTE;
        const META64: u8 = prefix::META | size_prefix::SIZE_PREFIX_8BYTE;
        const PADDING8: u8 = prefix::PADDING | size_prefix::SIZE_PREFIX_1BYTE;
        const PADDING16: u8 = prefix::PADDING | size_prefix::SIZE_PREFIX_2BYTE;
        const PADDING32: u8 = prefix::PADDING | size_prefix::SIZE_PREFIX_4BYTE;
        const PADDING64: u8 = prefix::PADDING | size_prefix::SIZE_PREFIX_8BYTE;
        let header = self.peek()?;
        match header {
            UNDEFINED => {
                let pos = self.now_pos()?;
                return Err(Error::new(ErrorCode::Other("Cant perse UNDEFINED TYPE".to_string()), pos as usize));
            },
            NONE => return visitor.visit_none(),
            BOOLF => {
                return visitor.visit_bool(false);
            },
            BOOLT => {
                return visitor.visit_bool(true);
            },
            UINT8 => {
                self.reader.seek(io::SeekFrom::Current(-1))?; // 1 byte戻す
                let u8_val = self.reader.read_u8()?;
                return visitor.visit_u8(u8_val);
            },
            UINT16 => {
                self.reader.seek(io::SeekFrom::Current(-2))?; // 2 byte戻す
                let u16_val = self.reader.read_u16()?;
                return visitor.visit_u16(u16_val);
            },
            UINT32 => {
                self.reader.seek(io::SeekFrom::Current(-4))?; // 4 byte戻す
                let u32_val = self.reader.read_u32()?;
                return visitor.visit_u32(u32_val);
            },
            UINT64 => {
                self.reader.seek(io::SeekFrom::Current(-8))?; // 8 byte戻す
                let u64_val = self.reader.read_u64()?;
                return visitor.visit_u64(u64_val);
            },
            INT8 => {
                self.reader.seek(io::SeekFrom::Current(-1))?; // 1 byte戻す
                let as_u8_val = self.reader.read_u8()?;
                return visitor.visit_i8(as_u8_val as i8);
            },
            INT16 => {
                self.reader.seek(io::SeekFrom::Current(-2))?; // 2 byte戻す
                let as_u16_val = self.reader.read_u16()?;
                return visitor.visit_i16(as_u16_val as i16);
            },
            INT32 => {
                self.reader.seek(io::SeekFrom::Current(-4))?; // 4 byte戻す
                let as_u32_val = self.reader.read_u32()?;
                return visitor.visit_i32(as_u32_val as i32);
            },
            INT64 => {
                self.reader.seek(io::SeekFrom::Current(-8))?; // 8 byte戻す
                let as_u64_val = self.reader.read_u64()?;
                return visitor.visit_i64(as_u64_val as i64);
            },
            FLOAT16 => {
                self.reader.seek(io::SeekFrom::Current(-2))?; // 2 byte戻す
                let as_u16_val = self.reader.read_u16()?;
                return visitor.visit_f32(f32::from_bits(as_u16_val as u32));
            },
            FLOAT32 => {
                self.reader.seek(io::SeekFrom::Current(-4))?; // 4 byte戻す
                let as_u32_val = self.reader.read_u32()?;
                return visitor.visit_f32(f32::from_bits(as_u32_val));
            },
            FLOAT64 => {
                self.reader.seek(io::SeekFrom::Current(-8))?; // 8 byte戻す
                let as_u64_val = self.reader.read_u64()?;
                return visitor.visit_f64(f64::from_bits(as_u64_val));
            },
            STRING8 => {},
            STRING16 => {},
            STRING32 => {},
            STRING64 => {},
            BYTES8 => {},
            BYTES16 => {},
            BYTES32 => {},
            BYTES64 => {},
            UUID => {},
            DATETIME => {},
            TIMESTAMP => {},
            DURATION => {},
            ARRAY8 => {},
            ARRAY16 => {},
            ARRAY32 => {},
            ARRAY64 => {},
            OBJECT8 => {},
            OBJECT16 => {},
            OBJECT32 => {},
            OBJECT64 => {},
            WRAPPED_JSON8 => {},
            WRAPPED_JSON16 => {},
            WRAPPED_JSON32 => {},
            WRAPPED_JSON64 => {},
            META8 => {},
            META16 => {},
            META32 => {},
            META64 => {},
            PADDING8 => {},
            PADDING16 => {},
            PADDING32 => {},
            PADDING64 => {},
            _ => {
                let pos = self.now_pos()?;
                Err(Error::new(ErrorCode::InvalidType, pos as usize))
            },
        }
    }

    /// ヘッダーを読み込む
    /// # warnings
    /// seekを-9 byte分進めるので、データの開始位置のseekは手動で合わせる必要がある
    pub fn read_header(&mut self) -> Result<(u64, u8), io::Error> {
        // ヘッダーを読み込む処理
        let header = self.peek()?;
        let size_prefix_val = header & size_prefix::MASK;
        match size_prefix_val {
            0 => {
                let size = self.prev()?;
                Ok((size as u64, header))
            },
            1 => {
                self.reader.seek(io::SeekFrom::Current(-2))?;
                let header = self.reader.read_u16()?;
                Ok((header as u64, header as u8))
            },
            2 => {
                self.reader.seek(io::SeekFrom::Current(-4))?;
                let header = self.reader.read_u32()?;
                Ok((header as u64, header as u8))
            },
            3 => {
                self.reader.seek(io::SeekFrom::Current(-8))?;
                let header = self.reader.read_u64()?;
                Ok((header, header as u8))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid size prefix")),
                
        }
    }
}

impl<'de, R> Deserializer<'de> for ReverseDeserializer<R>
where R: Reader,
{
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        // ここでデシリアライズのロジックを実装
        // 例: visitor.visit_i64(42)
        Err(serde::de::Error::custom("deserialize_any is not implemented"))
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let _ = visitor;
        Err(serde::de::Error::custom("i128 is not supported"))
    }
    
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let _ = visitor;
        Err(serde::de::Error::custom("u128 is not supported"))
    }
    
    fn is_human_readable(&self) -> bool {
        true
    }
    
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de> {
        todo!()
    }
    
    // 他の必要なメソッドを実装
}