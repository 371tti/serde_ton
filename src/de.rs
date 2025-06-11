use std::{fs::File, io::{self, Read, Seek, SeekFrom}};

use serde::{de::Visitor, Deserialize, Deserializer};

use crate::{error::{Error, ErrorCode}, value::prefix::{prefix, size_prefix}};




/// RTON のデシリアライザ
/// 後ろから読み込む必要がある
/// ファイルとかだとseek位置を動かしてバッファリングすべき
/// バッファリングは外部ですべき - 柔軟性
/// seekトレイとが実装されてる標準型が少ないのが問題
/// 楽に使うために変換できるように
/// 
pub struct ReverseDeserializer<R>
where R: io::Read + io::Seek,
{
    reader: R,
    deep: u64,
}

impl ReverseDeserializer<io::Cursor<Vec<u8>>> 
{
    pub fn from_vector(vec: Vec<u8>) -> Result<Self, io::Error> {
        let reader = io::Cursor::new(vec);
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.reader.into_inner()
    }
}

impl<'a> ReverseDeserializer<io::Cursor<&'a [u8]>> 
{
    pub fn from_slice(slice: &'a [u8]) -> Result<Self, io::Error> {
        let reader = io::Cursor::new(slice);
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> &'a [u8] {
        self.reader.into_inner()
    }
}

impl ReverseDeserializer<File> 
{
    pub fn from_file(file: File) -> Result<Self,io:: Error> {
        let reader = file;
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> File {
        self.reader
    }
}

impl<R> ReverseDeserializer<R>
where R: io::Read + io::Seek,
{
    pub fn new(reader: R) -> Result<Self, io::Error> {
        Ok(Self { reader, deep: 0 })
    }

    pub fn now_pos(&mut self) -> Result<u64, io::Error> {
        self.reader.stream_position()
    }

    pub fn rev_seek(&mut self, pos: i64) -> Result<u64, io::Error> {
        self.reader.seek(io::SeekFrom::End(pos))
    }

    pub fn get_size(reader: &mut R) -> io::Result<u64> {
        let current = reader.stream_position()?;
        let end = reader.seek(io::SeekFrom::End(0))?;
        reader.seek(io::SeekFrom::Start(current))?;
        Ok(end)
    }

    /// ヘッダーを読み込む
    /// # warnings
    /// seekを-9 byte分進めるので、データの開始位置のseekは手動で合わせる必要がある
    pub fn read_header(&mut self) -> Result<(u64, u8), io::Error> {
        // ヘッダーを読み込む処理
        self.reader.seek(io::SeekFrom::Current(-9))?; // ヘッダーのサイズ分だけ後ろにシーク
        // let pos = self.reader.stream_position()?; // pos は現在使用されていないためコメントアウトまたは削除
        let mut header_buf = [0u8; 9]; // ヘッダーのサイズ (値8バイト + プレフィックス1バイト)
        self.reader.read_exact(&mut header_buf)?;

        let prefix_byte = header_buf[8];
        // size_prefix::MASK が現在のスコープで定義されていると仮定します
        // 例: const MASK: u8 = 0x03;
        let size_indicator = prefix_byte & size_prefix::MASK;

        let value: u64;
        let mut value_bytes_le = [0u8; 8]; // u64 を表現するための8バイト配列 (リトルエンディアン)

        match size_indicator {
            0 => { // サイズが1バイトの場合 (header_buf[7] から取得)
                value_bytes_le[0] = header_buf[7];
                //残りのバイトは既に0で初期化されている
                value = u64::from_le_bytes(value_bytes_le);
            }
            1 => { // サイズが2バイトの場合 (header_buf[6..8] から取得)
                value_bytes_le[0..2].copy_from_slice(&header_buf[6..8]);
                //残りのバイトは既に0で初期化されている
                value = u64::from_le_bytes(value_bytes_le);
            }
            2 => { // サイズが4バイトの場合 (header_buf[4..8] から取得)
                value_bytes_le[0..4].copy_from_slice(&header_buf[4..8]);
                //残りのバイトは既に0で初期化されている
                value = u64::from_le_bytes(value_bytes_le);
            }
            3 => { // サイズが8バイトの場合 (header_buf[0..8] から取得)
                value_bytes_le.copy_from_slice(&header_buf[0..8]);
                value = u64::from_le_bytes(value_bytes_le);
            }
            _ => {
                // Error 型が io::Error から変換可能であるか、
                // または io::Error と互換性があると仮定します。
                // 必要に応じて .into() を追加してください。
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid size prefix"));
            }
        }
        Ok((value, prefix_byte))
    }
}

impl<'de, R> Deserializer<'de> for ReverseDeserializer<R>
where R: io::Read + io::Seek,
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