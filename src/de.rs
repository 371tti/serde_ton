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

    pub fn now_pos(&mut self) -> Result<u64, io::Error> {
        self.reader.stream_position()
    }

    pub fn get_size(reader: &mut R) -> io::Result<u64> {
        let current = reader.stream_position()?;
        let end = reader.seek(io::SeekFrom::End(0))?;
        reader.seek(io::SeekFrom::Start(current))?;
        Ok(end)
    }

    pub fn next(&mut self) -> Result<u8, io::Error> {
        if let Some(i) = self.reader.peek()? {
            Ok(i)
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more data to read"))
        }
    }

    pub fn prev(&mut self) -> Result<u8, io::Error> {
        if let Some(i) = self.reader.prev()? {
            Ok(i)
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more data to read"))
        }
    }

    pub fn peek(&mut self) -> Result<u8, io::Error> {
        if let Some(i) = self.reader.peek()? {
            Ok(i)
        } else {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more data to read"))
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