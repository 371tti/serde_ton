use std::io::Write;

use serde::{ser, Serialize, Serializer};

use crate::traits::ser::{ExtendSerialize, ExtendSerializeMap, ExtendSerializeSeq, ExtendSerializeStruct, ExtendSerializeStructVariant, ExtendSerializeTuple, ExtendSerializeTupleStruct, ExtendSerializeTupleVariant, ExtendedSerializer};
use crate::value::prefix::self_describe;
use crate::{error::Error, value::prefix::prefix};
use crate::value::prefix::size_prefix::{SIZE_PREFIX_1BYTE, SIZE_PREFIX_2BYTE, SIZE_PREFIX_4BYTE, SIZE_PREFIX_8BYTE};

/// A structure for serializing Rust values to RTON.
pub struct ReverseSerializer<W>
where
    W: Write,
{
    writer: W,
    size: usize,
    deep: usize,
}

impl<W> ReverseSerializer<W>
where
    W: Write,
{
    /// Create a new RTON serializer
    #[inline]
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            size: 0,
            deep: 0,
        }
    }

    /// Unwrap the `Writer` from the `Serializer`.
    #[inline]
    pub fn into_inner(self) -> W {
        self.writer
    }
    
    /// Writes a RTON self-describe tag to the stream.
    ///
    /// Tagging allows a decoder to distinguish different file formats based on their content
    /// without further information.
    #[inline]
    pub fn self_describe(&mut self) -> Result<(), Error> {
        self.write_bytes(&self_describe::TON_V1_REV_TAG)?;
        self.size += self_describe::TON_V1_REV_TAG.len();
        Ok(())
    }
    

    /// Wrap Writer
    #[inline]
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        self.writer.write_all(bytes).map_err(Error::io)?;
        Ok(())
    }

    /// シリアライズしたサイズを取得する
    /// 
    /// return: u64
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
}

/// Implement the `ExtendedSerializer` trait for `ReverseSerializer`.
/// 
/// This trait provides methods for serializing various types, including custom types like `f16`, `Uuid`, and `chrono::DateTime`.
impl<'a, W> ExtendedSerializer for &'a mut ReverseSerializer<W>
where
    W: Write,
{
    type ExtendSerializeSeq = Compound<'a, W>;
    type ExtendSerializeMap = Compound<'a, W>;

    #[inline]
    fn serialize_f16(self, v: half::f16) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 3] = [0; 3];
        buf[0..2].copy_from_slice(&v.to_bits().to_le_bytes());
        buf[2] = prefix::FLOAT | SIZE_PREFIX_2BYTE;
        self.write_bytes(&buf)?;
        self.size += 3;
        Ok(())
    }
    
    #[inline]
    fn serialize_uuid(self, v: &uuid::Uuid) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 17] = [0; 17];
        buf[0..16].copy_from_slice(v.as_bytes());
        buf[16] = prefix::UUID;
        self.write_bytes(&buf)?;
        self.size += 17;
        Ok(())
    }
    
    #[inline]
    fn serialize_datetime<Tz>(self, v: &chrono::DateTime<Tz>) -> Result<Self::Ok, Self::Error>
    where
        Tz: chrono::TimeZone + ?Sized {
        let rfc_str = v.to_rfc3339();
        let bytes = rfc_str.as_bytes();
        let size = bytes.len();
        let (header, header_size) = generate_header(prefix::DATETIME, size);
        // 日時データを逆順に格納
        self.write_bytes(&bytes)?;
        self.write_bytes(&header[..header_size])?;
        self.size += size + header_size;
        Ok(())
    }
    
    #[inline]
    fn serialize_timestamp(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0..8].copy_from_slice(&v.to_le_bytes());
        buf[8] = prefix::TIMESTAMP | SIZE_PREFIX_8BYTE;
        self.write_bytes(&buf)?;
        self.size += 9;
        Ok(())
    }
    
    #[inline]
    fn serialize_duration(self, v: &chrono::Duration) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0..8].copy_from_slice(&v.num_nanoseconds().unwrap_or(0).to_le_bytes());
        buf[8] = prefix::DURATION | SIZE_PREFIX_8BYTE;
        self.write_bytes(&buf)?;
        self.size += 9;
        Ok(())
    }
    
    #[inline]
    fn serialize_wrapped_json(
        self,
        v: &serde_json::Value,
    ) -> Result<Self::Ok, Self::Error> {
        let json_str = v.to_string();
        let bytes = json_str.as_bytes();
        let size = bytes.len();
        let (header, header_size) = generate_header(prefix::WRAPPED_JSON, size);
        // JSONデータを逆順に格納
        self.write_bytes(&bytes)?;
        self.write_bytes(&header[..header_size])?;
        self.size += size + header_size;
        Ok(())
    }
    
    #[inline]
    fn serialize_meta(self, v: &Box<crate::value::value::Value>) -> Result<Self::Ok, Self::Error> {
        let start_pos = self.size;
        v.ex_serialize(&mut *self)?;
        let (header, header_size) = generate_header(prefix::META, self.size - start_pos);
        self.write_bytes(&header[..header_size])?;
        self.size += header_size;
        Ok(())
    }
    
    #[inline]
    fn serialize_padding(self, v: usize) -> Result<Self::Ok, Self::Error> {
        if v == 0 {
            return Ok(());
        }
        let buf = vec![0u8; v];
        let (header, header_size) = generate_header(prefix::PADDING, v);
        // パディングデータを逆順に格納
        self.write_bytes(&buf)?;
        self.write_bytes(&header[..header_size])?;
        self.size += v + header_size;
        Ok(())
    }

    #[inline]
    fn ex_serialize_seq(self, _len: Option<usize>) -> Result<Self::ExtendSerializeSeq, Self::Error> {
        Ok(Compound::new(self))
    }

    #[inline]
    fn ex_serialize_map(self, _len: Option<usize>) -> Result<Self::ExtendSerializeMap, Self::Error> {
        Ok(Compound::new(self))
    }
}

/// Implement the `Serializer` trait for `ReverseSerializer`.
/// 
/// This trait provides methods for serializing various types, including primitive types, strings, and sequences.
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
        self.write_bytes(&value)?;
        self.size += 1;
        Ok(())
    }

    #[inline]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 2] = [0; 2];
        buf[0] = v.to_le() as u8;
        buf[1] = prefix::INT | SIZE_PREFIX_1BYTE;
        self.write_bytes(&buf)?;
        self.size += 2;
        Ok(())
    }

    #[inline]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 3] = [0; 3];
        buf[0..2].copy_from_slice(&v.to_le_bytes());
        buf[2] = prefix::INT | SIZE_PREFIX_2BYTE;
        self.write_bytes(&buf)?;
        self.size += 3;
        Ok(())
    }

    #[inline]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 5] = [0; 5];
        buf[0..4].copy_from_slice(&v.to_le_bytes());
        buf[4] = prefix::INT | SIZE_PREFIX_4BYTE;
        self.write_bytes(&buf)?;
        self.size += 5;
        Ok(())
    }
    #[inline]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0..8].copy_from_slice(&v.to_le_bytes());
        buf[8] = prefix::INT | SIZE_PREFIX_8BYTE;
        self.write_bytes(&buf)?;
        self.size += 9;
        Ok(())
    }

    #[inline]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let mut buf= [0u8; 2];
        buf[0] = v.to_le();
        buf[1] = prefix::UINT | SIZE_PREFIX_1BYTE;
        self.write_bytes(&buf)?;
        self.size += 2;
        Ok(())
    }

    #[inline]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 3] = [0; 3];
        buf[0..2].copy_from_slice(&v.to_le_bytes());
        buf[2] = prefix::UINT | SIZE_PREFIX_2BYTE;
        self.write_bytes(&buf)?;
        self.size += 3;
        Ok(())
    }

    #[inline]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 5] = [0; 5];
        buf[0..4].copy_from_slice(&v.to_le_bytes());
        buf[4] = prefix::UINT | SIZE_PREFIX_4BYTE;
        self.write_bytes(&buf)?;
        self.size += 5;
        Ok(())
    }

    #[inline]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0..8].copy_from_slice(&v.to_le_bytes());
        buf[8] = prefix::UINT | SIZE_PREFIX_8BYTE;
        self.write_bytes(&buf)?;
        self.size += 9;
        Ok(())
    }

    #[inline]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 5] = [0; 5];
        buf[0..4].copy_from_slice(&v.to_le_bytes());
        buf[4] = prefix::FLOAT | SIZE_PREFIX_4BYTE;
        self.write_bytes(&buf)?;
        self.size += 5;
        Ok(())
    }

    #[inline]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let mut buf: [u8; 9] = [0; 9];
        buf[0..8].copy_from_slice(&v.to_le_bytes());
        buf[8] = prefix::FLOAT | SIZE_PREFIX_8BYTE;
        self.write_bytes(&buf)?;
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
        self.write_bytes(bytes)?;
        self.write_bytes(&header[..header_size])?;
        self.size += size + header_size;
        Ok(())
    }
    
    #[inline]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let size = v.len();
        let (header, header_size) = generate_header(prefix::BYTES, size);
        // バイトデータを逆順に格納
        self.write_bytes(v)?;
        self.write_bytes(&header[..header_size])?;
        self.size += size + header_size;
        Ok(())
    }

    #[inline]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let value = [prefix::NONE];
        self.write_bytes(&value)?;
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
        self.write_bytes(&header[..header_size])?;
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
        self.ser.write_bytes(&header[..header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ExtendSerializeSeq for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize {
        value.ex_serialize(&mut *self.ser)?;
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
        self.ser.write_bytes(&header[..header_size])?;
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
        self.ser.write_bytes(&header[..header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ExtendSerializeTuple for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize {
        value.ex_serialize(&mut *self.ser)?;
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
        self.ser.write_bytes(&header[..header_size])?;
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
        self.ser.write_bytes(&header[..header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ExtendSerializeTupleStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize {
        value.ex_serialize(&mut *self.ser)?;
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
        self.ser.write_bytes(&header[..header_size])?;
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
        self.ser.write_bytes(&array_header[..array_header_size])?;
        self.ser.size += array_header_size;
        // mapのkeyをシリアライズ
        self.ser.serialize_str(self.variant_name.unwrap())?;
        // マップの合計サイズを計算
        let map_size = self.ser.size - self.start_pos;
        // mapヘッダを生成
        let (object_header, object_header_size) = generate_header(prefix::OBJECT, map_size);
        // ヘッダを書き込み
        self.ser.write_bytes(&object_header[..object_header_size])?;
        self.ser.size += object_header_size;
        Ok(())
    }
}

impl <'a, W> ExtendSerializeTupleVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize {
        value.ex_serialize(&mut *self.ser)?;
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
        self.ser.write_bytes(&array_header[..array_header_size])?;
        self.ser.size += array_header_size;
        // mapのkeyをシリアライズ
        self.ser.serialize_str(self.variant_name.unwrap())?;
        // マップの合計サイズを計算
        let map_size = self.ser.size - self.start_pos;
        // mapヘッダを生成
        let (object_header, object_header_size) = generate_header(prefix::OBJECT, map_size);
        // ヘッダを書き込み
        self.ser.write_bytes(&object_header[..object_header_size])?;
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
        self.ser.write_bytes(&header[..header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ExtendSerializeMap for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
        where
            K: ?Sized + ExtendSerialize,
            V: ?Sized + ExtendSerialize, {
        
        // 逆順のため、valueを先にシリアライズ
        value.ex_serialize(&mut *self.ser)?;
        key.ex_serialize(&mut *self.ser)?;

        Ok(())
    }

    #[inline]
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize {
        key.ex_serialize(&mut *self.ser)?;
        Ok(())
    }

    #[inline]
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize {
        value.ex_serialize(&mut *self.ser)?;
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
        self.ser.write_bytes(&header[..header_size])?;
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
        self.ser.write_bytes(&header[..header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        Ok(())
    }
}

impl <'a, W> ExtendSerializeStruct for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize {
        // 逆順のため、valueを先にシリアライズ
        value.ex_serialize(&mut *self.ser)?;
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
        self.ser.write_bytes(&header[..header_size])?;
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
        self.ser.write_bytes(&header[..header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        // mapのkeyをシリアライズ
        self.ser.serialize_str(self.variant_name.unwrap())?;
        // outer_structの合計サイズを計算
        let outer_struct_size = self.ser.size - self.start_pos;
        // outer_structのヘッダを生成
        let (outer_struct_header, outer_struct_header_size) = generate_header(prefix::OBJECT, outer_struct_size);
        // outer_structヘッダを書き込み
        self.ser.write_bytes(&outer_struct_header[..outer_struct_header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += outer_struct_header_size;
        Ok(())
    }
}

impl <'a, W> ExtendSerializeStructVariant for Compound<'a, W>
where
    W: Write,
{
    type Ok = ();
    type Error = Error;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize {
        // 逆順のため、valueを先にシリアライズ
        value.ex_serialize(&mut *self.ser)?;
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
        self.ser.write_bytes(&header[..header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += header_size;
        // mapのkeyをシリアライズ
        self.ser.serialize_str(self.variant_name.unwrap())?;
        // outer_structの合計サイズを計算
        let outer_struct_size = self.ser.size - self.start_pos;
        // outer_structのヘッダを生成
        let (outer_struct_header, outer_struct_header_size) = generate_header(prefix::OBJECT, outer_struct_size);
        // outer_structヘッダを書き込み
        self.ser.write_bytes(&outer_struct_header[..outer_struct_header_size])?;
        // ヘッダ分のサイズを加算
        self.ser.size += outer_struct_header_size;
        Ok(())
    }
    
}

/// Generate reverse serialization header.
/// 
/// prefix: u8 // header prefix
/// size_of_byte: usize // data size
/// 
/// return: ([u8; 9], usize) // header, header size
#[inline]
pub fn generate_header(prefix: u8, size_of_byte: usize) -> ([u8; 9], usize) {
    let mut buf = [0u8; 9];

    if size_of_byte <= u8::MAX as usize {
        buf[0] = size_of_byte as u8;
        buf[1] = prefix | SIZE_PREFIX_1BYTE;
        return (buf, 2);
    }

    if size_of_byte <= u16::MAX as usize {
        buf[0..2].copy_from_slice(&(size_of_byte as u16).to_le_bytes());
        buf[2] = prefix | SIZE_PREFIX_2BYTE;
        return (buf, 3);
    }

    if size_of_byte <= u32::MAX as usize {
        buf[0..4].copy_from_slice(&(size_of_byte as u32).to_le_bytes());
        buf[4] = prefix | SIZE_PREFIX_4BYTE;
        return (buf, 5);
    }

    buf[0..8].copy_from_slice(&(size_of_byte as u64).to_le_bytes());
    buf[8] = prefix | SIZE_PREFIX_8BYTE;
    (buf, 9)
}


