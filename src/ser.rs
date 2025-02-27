use std::io::Write;

use serde::{ser, Serialize, Serializer};

use crate::{error::Error, value::prefix::prefix};

const SIZE_PREFIX_1BYTE: u8 = 0b000000_00;
const SIZE_PREFIX_2BYTE: u8 = 0b000000_01;
const SIZE_PREFIX_4BYTE: u8 = 0b000000_10;
const SIZE_PREFIX_8BYTE: u8 = 0b000000_11;
/// Reverse TON シリアライザー
/// 
/// cp 8bit 単位で逆順にストリームでシリアライズします
pub struct ReverseSerializer<W>
where
    W: Write,
{
    writer: W,
    buffer: Vec<u8>,
    size: usize,
    deep: usize,
}

impl<W> ReverseSerializer<W>
where
    W: Write,
{
    /// 新しいReverseSerializerを作る
    /// 
    /// writer: W
    /// 
    /// return: ReverseSerializer
    #[inline]
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            size: 0,
            buffer: Vec::with_capacity(256/*default capacity*/),
            deep: 0,
        }
    }

    #[inline]
    pub fn with_capacity(writer: W, capacity: usize) -> Self {
        Self {
            writer,
            size: 0,
            buffer: Vec::with_capacity(capacity),
            deep: 0,
        }
    }

    /// バッファに書き込む
    /// 
    /// バッファのキャパがいっぱいになるとflashします
    /// 深さが0の場合、必ずすべてを書ききります
    /// 
    /// iterator: I
    /// 
    /// return: Result<(), Error>
    #[inline]
    fn write_iter<'a, I>(&mut self, mut iterator: I) -> Result<(), Error>
    where
        I: Iterator<Item = &'a u8>,
    {
        let cp = self.buffer.capacity();
        let mut rem_cp = cp - self.buffer.len();
        'outer: loop {
            for _ in 0..rem_cp {
                if let Some(b) = iterator.next() {
                    self.buffer.push(*b);
                } else {
                    break 'outer;
                }
            }
            self.flash()?;
            rem_cp = cp;
        }
        if self.deep == 0 {
            self.flash()?;
        }

        Ok(())
    }

    /// バッファの内容をフラッシュする
    #[inline]
    fn flash(&mut self) -> Result<(), Error> {
        self.writer.write_all(&self.buffer).map_err(Error::io)?;
        self.buffer.truncate(0);
        Ok(())
    }

    /// シリアライズしたサイズを取得する
    /// 
    /// return: u64
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }

    /// writerを取り出す
    /// 
    /// return: &W
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<'a, W> ser::Serializer for &'a mut ReverseSerializer<W> 
where 
    W: Write,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    #[inline]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        // <header: <prefix: 6bit, value: 1bit>
        let value = [prefix::BOOL | (v as u8).to_be()];
        self.write_iter(value.iter())?;
        self.size += 1;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 2] = [0; 2];
        buf[0] = v.to_le() as u8;
        buf[1] = prefix::INT | SIZE_PREFIX_1BYTE;
        self.write_iter(buf.iter())?;
        self.size += 2;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 3] = [0; 3];
        buf[0..2].copy_from_slice(&v.to_le_bytes());
        buf[2] = prefix::INT | SIZE_PREFIX_2BYTE;
        self.write_iter(buf.iter())?;
        self.size += 3;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 5] = [0; 5];
        buf[0..4].copy_from_slice(&v.to_le_bytes());
        buf[4] = prefix::INT | SIZE_PREFIX_4BYTE;
        self.write_iter(buf.iter())?;
        self.size += 5;
        Ok(())
    }
    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0..8].copy_from_slice(&v.to_le_bytes());
        buf[8] = prefix::INT | SIZE_PREFIX_8BYTE;
        self.write_iter(buf.iter())?;
        self.size += 9;
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let mut buf= [0u8; 2];
        buf[0] = v.to_le();
        buf[1] = prefix::UINT | SIZE_PREFIX_1BYTE;
        self.write_iter(buf.iter())?;
        self.size += 2;
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 3] = [0; 3];
        buf[0..2].copy_from_slice(&v.to_le_bytes());
        buf[2] = prefix::UINT | SIZE_PREFIX_2BYTE;
        self.write_iter(buf.iter())?;
        self.size += 3;
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 5] = [0; 5];
        buf[0..4].copy_from_slice(&v.to_le_bytes());
        buf[4] = prefix::UINT | SIZE_PREFIX_4BYTE;
        self.write_iter(buf.iter())?;
        self.size += 5;
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0..8].copy_from_slice(&v.to_le_bytes());
        buf[8] = prefix::UINT | SIZE_PREFIX_8BYTE;
        self.write_iter(buf.iter())?;
        self.size += 9;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 5] = [0; 5];
        buf[0..4].copy_from_slice(&v.to_le_bytes());
        buf[4] = prefix::FLOAT | SIZE_PREFIX_4BYTE;
        self.write_iter(buf.iter())?;
        self.size += 5;
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0..8].copy_from_slice(&v.to_le_bytes());
        buf[8] = prefix::FLOAT | SIZE_PREFIX_8BYTE;
        self.write_iter(buf.iter())?;
        self.size += 9;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let bytes = v.as_bytes();
        let size = bytes.len();
        let (header, header_size) = generate_header(prefix::STRING, size);
        // 文字列データを逆順に格納
        let value = bytes.iter().chain(header[..header_size].iter().rev());
        self.write_iter(value)?;
        self.size += size + header_size;
        Ok(())
    }
    
    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let size = v.len();
        let (header, header_size) = generate_header(prefix::BYTES, size);
        // バイトデータを逆順に格納
        let value = v.iter().chain(header[..header_size].iter().rev());
        self.write_iter(value)?;
        self.size += size + header_size;
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let value = [prefix::NULL];
        self.write_iter(value.iter())?;
        self.size += 1;
        Ok(())
    }


    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_none()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + ser::Serialize {
        let start_pos = self.size;
        self.serialize_some(value)?;
        self.serialize_str(variant)?;
        let (header, header_size) = generate_header(prefix::OBJECT, self.size - start_pos);
        self.write_iter(header[..header_size].iter().rev())?;
        self.size += header_size;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Compound::with_variant(self, variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound::new(self))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Compound::with_variant(self, variant))
    }
}

pub struct Compound<'a, W>
where
    W: Write,
{
    ser: &'a mut ReverseSerializer<W>,
    start_pos: usize,
    variant_name: Option<&'static str>,
}

impl<'a, W> Compound<'a, W>
where 
    W: Write,
{
    #[inline]
    pub fn new(ser: &'a mut ReverseSerializer<W>) -> Self {
        let start_pos = ser.size;
        // ネストの深さを増やす
        ser.deep += 1;
        Self {
            ser,
            start_pos,
            variant_name: None,
        }
    }

    #[inline]
    pub fn with_variant(ser: &'a mut ReverseSerializer<W>, variant_name: &'static str) -> Self {
        let start_pos = ser.size;
        // ネストの深さを増やす
        ser.deep += 2;
        Self {
            ser,
            start_pos,
            variant_name: Some(variant_name),
        }
    }
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;


    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // ネストを抜ける
        self.ser.deep -= 1;
        // シーケンスの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_header(prefix::ARRAY, seq_size);
        // ヘッダを書き込み
        self.ser.write_iter(header[..header_size].iter().rev())?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ser::SerializeTuple for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // ネストを抜ける
        self.ser.deep -= 1;
        // シーケンスの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_header(prefix::ARRAY, seq_size);
        // ヘッダを書き込み
        self.ser.write_iter(header[..header_size].iter().rev())?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ser::SerializeTupleStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // ネストを抜ける
        self.ser.deep -= 1;
        // シーケンスの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_header(prefix::ARRAY, seq_size);
        // ヘッダを書き込み
        self.ser.write_iter(header[..header_size].iter().rev())?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ser::SerializeTupleVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // ネストを抜ける(seq と map 分)
        self.ser.deep -= 2;
        // シーケンスの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // seqヘッダを生成
        let (array_header, array_header_size) = generate_header(prefix::ARRAY, seq_size);
        // seqヘッダを書き込み
        let array_header_iter = array_header[..array_header_size].iter();
        self.ser.write_iter(array_header_iter.rev())?;
        self.ser.size += array_header_size;
        // mapのkeyをシリアライズ
        self.ser.serialize_str(self.variant_name.unwrap())?;
        // マップの合計サイズを計算
        let map_size = self.ser.size - self.start_pos;
        // mapヘッダを生成
        let (object_header, object_header_size) = generate_header(prefix::OBJECT, map_size);
        // ヘッダを書き込み
        let object_header_iter = object_header[..object_header_size].iter();
        self.ser.write_iter(object_header_iter.rev())?;
        self.ser.size += object_header_size;
        Ok(())
    }
}

impl <'a, W> ser::SerializeMap for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
        where
            K: ?Sized + ser::Serialize,
            V: ?Sized + ser::Serialize, {
        
        // 逆順のため、valueを先にシリアライズ
        value.serialize(&mut *self.ser)?;
        key.serialize(&mut *self.ser)?;

        Ok(())
    }

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        key.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // ネストを抜ける
        self.ser.deep -= 1;
        // Mapの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_header(prefix::OBJECT, seq_size);
        // ヘッダを書き込み
        self.ser.write_iter(header[..header_size].iter().rev())?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ser::SerializeStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        // 逆順のため、valueを先にシリアライズ
        value.serialize(&mut *self.ser)?;
        key.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // ネストを抜ける
        self.ser.deep -= 1;
        // Structの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_header(prefix::OBJECT, seq_size);
        // ヘッダを書き込み
        self.ser.write_iter(header[..header_size].iter().rev())?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ser::SerializeStructVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        // 逆順のため、valueを先にシリアライズ
        value.serialize(&mut *self.ser)?;
        key.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // ネストを抜ける(map と map 分)
        self.ser.deep -= 2;
        // structの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // structのヘッダを生成
        let (header, header_size) = generate_header(prefix::OBJECT, seq_size);
        // structヘッダを書き込み
        self.ser.write_iter(header[..header_size].iter().rev())?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        // mapのkeyをシリアライズ
        self.ser.serialize_str(self.variant_name.unwrap())?;
        // outer_structの合計サイズを計算
        let outer_struct_size = self.ser.size - self.start_pos;
        // outer_structのヘッダを生成
        let (outer_struct_header, outer_struct_header_size) = generate_header(prefix::OBJECT, outer_struct_size);
        // outer_structヘッダを書き込み
        self.ser.write_iter(outer_struct_header[..outer_struct_header_size].iter().rev())?;
        // ヘッダ分のサイズを加算
        self.ser.size += outer_struct_header_size;
        Ok(())
    }
}

/// 反転したheaderを生成する
/// 
/// prefix: u8 // 型情報
/// size_of_byte: u64 // データ本体のサイズ
/// 
/// return: ([u8; 9], u8) // header, headerのサイズ
#[inline]
pub fn generate_header(prefix: u8, size_of_byte: usize) -> ([u8; 9], usize) {
    match size_of_byte {
        s if s <= u8::MAX as usize => {
            let mut buf = [0; 9];
            buf[0] = prefix | SIZE_PREFIX_1BYTE;
            buf[1] = s.to_le() as u8;
            (buf, 2)
        },
        s if s <= u16::MAX as usize => {
            let mut buf = [0; 9];
            buf[0] = prefix | SIZE_PREFIX_2BYTE;
            buf[1..3].copy_from_slice(&(s as u16).to_le_bytes());
            (buf, 3)
        },
        s if s <= u32::MAX as usize => {
            let mut buf = [0; 9];
            buf[0] = prefix | SIZE_PREFIX_4BYTE;
            buf[1..5].copy_from_slice(&(s as u32).to_le_bytes());
            (buf, 5)
        },
        s => {
            let mut buf = [0; 9];
            buf[0] = prefix | SIZE_PREFIX_8BYTE;
            buf[1..9].copy_from_slice(&(s as u64).to_le_bytes());
            (buf, 9)
        },
    }
}
#[cfg(test)]
mod ser_tests {
    use serde::ser::SerializeSeq;
use std::collections::HashMap;

    use super::*;
    // Test ReverseSerializer using an in-memory Vec<u8>
    #[test]
    fn test_serialize_bool() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_bool(true).unwrap();
            // Flush any remaining buffered bytes
            serializer.flash().unwrap();
        }
        // For bool, we write one byte: [prefix::BOOL | (true as u8)]
        let expected = vec![1 | prefix::BOOL];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_u8() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_u8(42).unwrap();
            serializer.flash().unwrap();
        }
        // For u8, we write two bytes: [42, prefix::UINT | SIZE_PREFIX_1BYTE]
        let expected = vec![42, prefix::UINT | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_u16() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_u16(42).unwrap();
            serializer.flash().unwrap();
        }
        // For u16, we write three bytes: [42, 0, prefix::UINT | SIZE_PREFIX_2BYTE]
        let expected = vec![42, 0, prefix::UINT | SIZE_PREFIX_2BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_u32() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_u32(42).unwrap();
            serializer.flash().unwrap();
        }
        // For u32, we write five bytes: [42, 0, 0, 0, prefix::UINT | SIZE_PREFIX_4BYTE]
        let expected = vec![42, 0, 0, 0, prefix::UINT | SIZE_PREFIX_4BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_u64() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_u64(42).unwrap();
            serializer.flash().unwrap();
        }
        // For u64, we write nine bytes: [42, 0, 0, 0, 0, 0, 0, 0, prefix::UINT | SIZE_PREFIX_8BYTE]
        let expected = vec![42, 0, 0, 0, 0, 0, 0, 0, prefix::UINT | SIZE_PREFIX_8BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_i8() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_i8(-42).unwrap();
            serializer.flash().unwrap();
        }
        // For i8, we write two bytes: [-42, prefix::INT | SIZE_PREFIX_1BYTE]
        let expected = vec![-42i8.to_le() as u8, prefix::INT | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_i16() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_i16(-42).unwrap();
            serializer.flash().unwrap();
        }
        // For i16, we write three bytes: [-42 bytes in little endian, prefix::INT | SIZE_PREFIX_2BYTE]
        let i16_bytes = (-42i16).to_le_bytes();
        let expected = vec![i16_bytes[0], i16_bytes[1], prefix::INT | SIZE_PREFIX_2BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_i32() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_i32(-42).unwrap();
            serializer.flash().unwrap();
        }
        // For i32, we write five bytes: [-42 bytes in little endian, prefix::INT | SIZE_PREFIX_4BYTE]
        let i32_bytes = (-42i32).to_le_bytes();
        let expected = vec![i32_bytes[0], i32_bytes[1], i32_bytes[2], i32_bytes[3], prefix::INT | SIZE_PREFIX_4BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_i64() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_i64(-42).unwrap();
            serializer.flash().unwrap();
        }
        // For i64, we write nine bytes: [-42 bytes in little endian, prefix::INT | SIZE_PREFIX_8BYTE]
        let i64_bytes = (-42i64).to_le_bytes();
        let expected = vec![i64_bytes[0], i64_bytes[1], i64_bytes[2], i64_bytes[3], i64_bytes[4], i64_bytes[5], i64_bytes[6], i64_bytes[7], prefix::INT | SIZE_PREFIX_8BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_f32() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_f32(42.0).unwrap();
            serializer.flash().unwrap();
        }
        // For f32, we write five bytes: [42.0 bytes in little endian, prefix::FLOAT | SIZE_PREFIX_4BYTE]
        let f32_bytes = 42.0f32.to_le_bytes();
        let expected = vec![f32_bytes[0], f32_bytes[1], f32_bytes[2], f32_bytes[3], prefix::FLOAT | SIZE_PREFIX_4BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_f64() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_f64(42.0).unwrap();
            serializer.flash().unwrap();
        }
        // For f64, we write nine bytes: [42.0 bytes in little endian, prefix::FLOAT | SIZE_PREFIX_8BYTE]
        let f64_bytes = 42.0f64.to_le_bytes();
        let expected = vec![f64_bytes[0], f64_bytes[1], f64_bytes[2], f64_bytes[3], f64_bytes[4], f64_bytes[5], f64_bytes[6], f64_bytes[7], prefix::FLOAT | SIZE_PREFIX_8BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_char() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_char('A').unwrap();
            serializer.flash().unwrap();
        }
        // For char, we write six bytes: [prefix::STRING | SIZE_PREFIX_1BYTE, 4, 'A' bytes in little endian]
        let expected = vec![prefix::STRING | SIZE_PREFIX_1BYTE, 1, b'A'];
        out.reverse();
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_str() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_str("Hello, world!").unwrap();
            serializer.flash().unwrap();
        }
      
        let expected = vec![b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13, prefix::STRING | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_bytes() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_bytes(b"Hello, world!").unwrap();
            serializer.flash().unwrap();
        }
      
        let expected = vec![b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13, prefix::BYTES | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_none() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_none().unwrap();
            serializer.flash().unwrap();
        }
        // For none, we write one byte: [prefix::NULL]
        let expected = vec![prefix::NULL];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_some() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_some(&42u8).unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![42, prefix::UINT | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_unit() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_unit().unwrap();
            serializer.flash().unwrap();
        }
        // For unit, we write one byte: [prefix::NULL]
        let expected = vec![prefix::NULL];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_unit_struct() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_unit_struct("Unit").unwrap();
            serializer.flash().unwrap();
        }
        // For unit struct, we write one byte: [prefix::NULL]
        let expected = vec![prefix::NULL];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_unit_variant() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_unit_variant("Unit", 0, "Variant").unwrap();
            serializer.flash().unwrap();
        }

        let expected = vec![b'V', b'a', b'r', b'i', b'a', b'n', b't', 7, prefix::STRING | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_newtype_struct() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_newtype_struct("Newtype", &42u8).unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![42, prefix::UINT | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_newtype_variant() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            serializer.serialize_newtype_variant("Newtype", 0, "Variant", &42u8).unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![42, prefix::UINT | SIZE_PREFIX_1BYTE, b'V', b'a', b'r', b'i', b'a', b'n', b't', 7, prefix::STRING | SIZE_PREFIX_1BYTE, 11, prefix::OBJECT | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_seq() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            let mut seq = serializer.serialize_seq(None).unwrap();
            seq.serialize_element(&"Hello, world!").unwrap();
            seq.serialize_element(&42u8).unwrap();
            seq.end().unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13, prefix::STRING | SIZE_PREFIX_1BYTE, 42, prefix::UINT | SIZE_PREFIX_1BYTE, 17, prefix::ARRAY | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_tuple() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            let mut tuple = serializer.serialize_tuple(2).unwrap();
            tuple.serialize_element(&"Hello, world!").unwrap();
            tuple.serialize_element(&42u8).unwrap();
            tuple.end().unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13, prefix::STRING | SIZE_PREFIX_1BYTE, 42, prefix::UINT | SIZE_PREFIX_1BYTE, 17, prefix::ARRAY | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_tuple_struct() {
        let mut out = Vec::new();
        {
            let mut serializer = ReverseSerializer::new(&mut out);
            let mut tuple = serializer.serialize_tuple_struct("Tuple", 2).unwrap();
            tuple.serialize_element(&"Hello, world!").unwrap();
            tuple.serialize_element(&42u8).unwrap();
            tuple.end().unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13, prefix::STRING | SIZE_PREFIX_1BYTE, 42, prefix::UINT | SIZE_PREFIX_1BYTE, 17, prefix::ARRAY | SIZE_PREFIX_1BYTE];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_tuple_variant() {
        #[derive(Serialize)]
        enum TestEnum {
            TupleVariant(&'static str, u8),
        }

        let mut out = Vec::new();
        {
            let value = TestEnum::TupleVariant("Hello, world!", 42);
            let mut serializer = ReverseSerializer::new(&mut out);
            value.serialize(&mut serializer).unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![
            b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13,
            prefix::STRING | SIZE_PREFIX_1BYTE,
            42, prefix::UINT | SIZE_PREFIX_1BYTE,
            17, prefix::ARRAY | SIZE_PREFIX_1BYTE,
            b'T', b'u', b'p', b'l', b'e', b'V', b'a', b'r', b'i', b'a', b'n', b't', 12, prefix::STRING | SIZE_PREFIX_1BYTE,
            33, prefix::OBJECT | SIZE_PREFIX_1BYTE,
        ];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_map() {
        let mut out = Vec::new();
        {
            let mut map = HashMap::new();
            map.insert("Hello, world!", 42u8);
            let mut serializer = ReverseSerializer::new(&mut out);
            map.serialize(&mut serializer).unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![
            42, prefix::UINT | SIZE_PREFIX_1BYTE,
            b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13, 
            prefix::STRING | SIZE_PREFIX_1BYTE,
            17, prefix::OBJECT | SIZE_PREFIX_1BYTE,
        ];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_struct() {
        #[derive(Serialize)]
        struct TestStruct {
            field1: &'static str,
            field2: u8,
        }

        let mut out = Vec::new();
        {
            let value = TestStruct {
                field1: "Hello, world!",
                field2: 42,
            };
            let mut serializer = ReverseSerializer::new(&mut out);
            value.serialize(&mut serializer).unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![
            b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13,
            prefix::STRING | SIZE_PREFIX_1BYTE,
            b'f', b'i', b'e', b'l', b'd', b'1', 6, prefix::STRING | SIZE_PREFIX_1BYTE,
            42, prefix::UINT | SIZE_PREFIX_1BYTE,
            b'f', b'i', b'e', b'l', b'd', b'2', 6, prefix::STRING | SIZE_PREFIX_1BYTE,
            33, prefix::OBJECT | SIZE_PREFIX_1BYTE,            
        ];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_serialize_struct_variant() {
        #[derive(Serialize)]
        enum TestEnum {
            StructVariant {
                field1: &'static str,
                field2: u8,
            },
        }

        let mut out = Vec::new();
        {
            let value = TestEnum::StructVariant {
                field1: "Hello, world!",
                field2: 42,
            };
            let mut serializer = ReverseSerializer::new(&mut out);
            value.serialize(&mut serializer).unwrap();
            serializer.flash().unwrap();
        }
        let expected = vec![
            b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 13,
            prefix::STRING | SIZE_PREFIX_1BYTE,
            b'f', b'i', b'e', b'l', b'd', b'1', 6, prefix::STRING | SIZE_PREFIX_1BYTE,
            42, prefix::UINT | SIZE_PREFIX_1BYTE,
            b'f', b'i', b'e', b'l', b'd', b'2', 6, prefix::STRING | SIZE_PREFIX_1BYTE,
            33, prefix::OBJECT | SIZE_PREFIX_1BYTE,
            b'S', b't', b'r', b'u', b'c', b't', b'V', b'a', b'r', b'i', b'a', b'n', b't', 13, prefix::STRING | SIZE_PREFIX_1BYTE,
            50, prefix::OBJECT | SIZE_PREFIX_1BYTE,
        ];
        assert_eq!(out, expected);
    }

    #[test]
    fn test_generate_header() {
        let (header, header_size) = generate_header(prefix::UINT, 42);
        let expected = vec![prefix::UINT | SIZE_PREFIX_1BYTE, 42];
        assert_eq!(header[..header_size].to_vec(), expected);

        let (header, header_size) = generate_header(prefix::UINT, u8::MAX as usize + 1);
        let expected = vec![prefix::UINT | SIZE_PREFIX_2BYTE, 0, 1];
        assert_eq!(header[..header_size].to_vec(), expected);

        let (header, header_size) = generate_header(prefix::UINT, u16::MAX as usize + 1);
        let expected = vec![prefix::UINT | SIZE_PREFIX_4BYTE, 0, 0, 1, 0];
        assert_eq!(header[..header_size].to_vec(), expected);

        let (header, header_size) = generate_header(prefix::UINT, u32::MAX as usize + 1);
        let expected = vec![prefix::UINT | SIZE_PREFIX_8BYTE, 0, 0, 0, 0, 1, 0, 0, 0];
        assert_eq!(header[..header_size].to_vec(), expected);
    }
}