use std::io::Write;

use serde::{ser, Serialize};

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
    size: u64,
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
        }
    }

    /// シリアライズしたサイズを取得する
    /// 
    /// return: u64
    #[inline]
    pub fn size(&self) -> u64 {
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
        self.writer.write(&[prefix::BOOL | v as u8]).map_err(|e| Error::io(e))?;
        self.size += 1;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let bytes = v.to_le_bytes();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::INT | SIZE_PREFIX_1BYTE]).map_err(|e| Error::io(e))?;
        self.size += 2;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let mut bytes = v.to_le_bytes();
        bytes.reverse();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::INT | SIZE_PREFIX_2BYTE]).map_err(|e| Error::io(e))?;
        self.size += 3;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut bytes = v.to_le_bytes();
        bytes.reverse();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::INT | SIZE_PREFIX_4BYTE]).map_err(|e| Error::io(e))?;
        self.size += 5;
        Ok(())
    }

    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut bytes = v.to_le_bytes();
        bytes.reverse();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::INT | SIZE_PREFIX_8BYTE]).map_err(|e| Error::io(e))?;
        self.size += 9;
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let bytes = v.to_le_bytes();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::UINT | SIZE_PREFIX_1BYTE]).map_err(|e| Error::io(e))?;
        self.size += 2;
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let mut bytes = v.to_le_bytes();
        bytes.reverse();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::UINT | SIZE_PREFIX_2BYTE]).map_err(|e| Error::io(e))?;
        self.size += 3;
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let mut bytes = v.to_le_bytes();
        bytes.reverse();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::UINT | SIZE_PREFIX_4BYTE]).map_err(|e| Error::io(e))?;
        self.size += 5;
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut bytes = v.to_le_bytes();
        bytes.reverse();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::UINT | SIZE_PREFIX_8BYTE]).map_err(|e| Error::io(e))?;
        self.size += 9;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let mut bytes = v.to_le_bytes();
        bytes.reverse();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::FLOAT | SIZE_PREFIX_4BYTE]).map_err(|e| Error::io(e))?;
        self.size += 5;
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let mut bytes = v.to_le_bytes();
        bytes.reverse();
        self.writer.write(&bytes).map_err(|e| Error::io(e))?;
        self.writer.write(&[prefix::FLOAT | SIZE_PREFIX_8BYTE]).map_err(|e| Error::io(e))?;
        self.size += 9;
        Ok(())
    }

    #[inline]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        let bytes = v.encode_utf8(&mut buf);
        self.serialize_str(&bytes)
    }

    #[inline]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let bytes = v.as_bytes();
        let len = bytes.len();
        let (header, header_size) = generate_reverse_header(prefix::STRING, len as u64);
        for &byte in bytes.iter().rev() {
            self.writer.write(&[byte]).map_err(|e| Error::io(e))?;
        }
        self.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        self.size += len as u64 + header_size as u64;
        Ok(())
    }

    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let len = v.len();
        let (header, header_size) = generate_reverse_header(prefix::BYTES, len as u64);
        for &byte in v.iter().rev() {
            self.writer.write(&[byte]).map_err(|e| Error::io(e))?;
        }
        self.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        self.size += len as u64 + header_size as u64;
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.writer.write(&[prefix::NULL]).map_err(|e| Error::io(e))?;
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
        let (header, header_size) = generate_reverse_header(prefix::OBJECT, self.size - start_pos);
        self.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
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
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let start_pos = self.size;
        self.serialize_tuple(len)?;
        self.serialize_str(variant)?;
        let (header, header_size) = generate_reverse_header(prefix::OBJECT, self.size - start_pos);
        self.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        Ok(Compound::new(self))
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
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let start_pos = self.size;
        self.serialize_struct(name, len)?;
        self.serialize_str(variant)?;
        let (header, header_size) = generate_reverse_header(prefix::OBJECT, self.size - start_pos);
        self.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        Ok(Compound::new(self))
    }
}

pub struct Compound<'a, W>
where
    W: Write,
{
    ser: &'a mut ReverseSerializer<W>,
    start_pos: u64,
}

impl<'a, W> Compound<'a, W>
where 
    W: Write,
{
    pub fn new(ser: &'a mut ReverseSerializer<W>) -> Self {
        let start_pos = ser.size;
        Self {
            ser,
            start_pos,
        }
    }
}

impl<'a, W> ser::SerializeSeq for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // シーケンスの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_reverse_header(prefix::ARRAY, seq_size);
        // ヘッダを書き込み
        self.ser.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size as u64;
        Ok(())
    }
}

impl <'a, W> ser::SerializeTuple for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // シーケンスの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_reverse_header(prefix::ARRAY, seq_size);
        // ヘッダを書き込み
        self.ser.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size as u64;
        Ok(())
    }
}

impl <'a, W> ser::SerializeTupleStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // シーケンスの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_reverse_header(prefix::ARRAY, seq_size);
        // ヘッダを書き込み
        self.ser.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size as u64;
        Ok(())
    }
}

/// ダミー実装
/// 呼び出されるが動作する必要ない
impl <'a, W> ser::SerializeTupleVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();

    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ser::Serialize {
        // value.serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        // // シーケンスの合計サイズを計算
        // let seq_size = self.ser.size - self.start_pos;
        // // ヘッダを生成
        // let (header, header_size) = generate_reverse_header(prefix::ARRAY, seq_size);
        // // ヘッダを書き込み
        // self.ser.writer.write(&header[..header_size as usize]);
        // // ヘッダ分のサイズを加算
        // self.ser.size += header_size as u64;
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
        // Mapの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_reverse_header(prefix::OBJECT, seq_size);
        // ヘッダを書き込み
        self.ser.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size as u64;
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
        // Structの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_reverse_header(prefix::OBJECT, seq_size);
        // ヘッダを書き込み
        self.ser.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size as u64;
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
        // Structの合計サイズを計算
        let seq_size = self.ser.size - self.start_pos;
        // ヘッダを生成
        let (header, header_size) = generate_reverse_header(prefix::OBJECT, seq_size);
        // ヘッダを書き込み
        self.ser.writer.write(&header[..header_size as usize]).map_err(|e| Error::io(e))?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size as u64;
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
pub fn generate_reverse_header(prefix: u8, size_of_byte: u64) -> ([u8; 9], u8) {
    match size_of_byte {
        s if s <= u8::MAX as u64 => {
            let v = s.to_le_bytes();
            (
                [
                    v[0], 
                    prefix | SIZE_PREFIX_1BYTE, 
                    0, 0, 0, 0, 0, 0, 0 // padding
                ], 
                2
            )
        },
        s if s <= u16::MAX as u64 => {
            let v = s.to_le_bytes();
            (
                [
                    v[1], 
                    v[0], 
                    prefix | SIZE_PREFIX_2BYTE, 
                    0, 0, 0, 0, 0, 0 // padding
                ], 
                3
            )
        },
        s if s <= u32::MAX as u64 => {
            let v = s.to_le_bytes();
            (
                [
                    v[3], 
                    v[2], 
                    v[1], 
                    v[0], 
                    prefix | SIZE_PREFIX_4BYTE, 
                    0, 0, 0, 0 // padding
                ], 
                5
            )
        },
        s => {
            let v = s.to_le_bytes();
            (
                [
                    v[7], 
                    v[6], 
                    v[5], 
                    v[4], 
                    v[3], 
                    v[2], 
                    v[1], 
                    v[0], 
                    prefix | SIZE_PREFIX_8BYTE
                ], 
                9
            )
        },
    }
}