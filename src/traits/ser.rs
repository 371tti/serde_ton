
use chrono::{DateTime, Duration, FixedOffset, Utc};
use serde::{ser::Error, Serialize, Serializer};
use half::f16;
use uuid::Uuid;

use crate::value::value::Value;

pub trait ExtendedSerializer: Sized + Serializer {
    type ExtendSerializeSeq: ExtendSerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    type ExtendSerializeMap: ExtendSerializeMap<Ok = Self::Ok, Error = Self::Error>;
    fn serialize_f16(self, v: f16) -> Result<Self::Ok, Self::Error>;
    fn serialize_uuid(self, v: &Uuid) -> Result<Self::Ok, Self::Error>;
    fn serialize_datetime<Tz>(self, v: &DateTime<Tz>) -> Result<Self::Ok, Self::Error>
    where
        Tz: chrono::TimeZone + ?Sized;
    fn serialize_timestamp(self, v: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_duration(self, v: &Duration) -> Result<Self::Ok, Self::Error>;
    fn serialize_wrapped_json(
        self,
        v: &serde_json::Value,
    ) -> Result<Self::Ok, Self::Error>;
    fn serialize_meta(self, v: &Box<Value>) -> Result<Self::Ok, Self::Error>;
    fn serialize_padding(self, v: usize) -> Result<Self::Ok, Self::Error>;

    fn ex_serialize_seq(self, _len: Option<usize>) -> Result<Self::ExtendSerializeSeq, Self::Error>;
    fn ex_serialize_map(self, _len: Option<usize>) -> Result<Self::ExtendSerializeMap, Self::Error>;
}

pub trait ExtendSerialize{
    fn ex_serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ExtendedSerializer;
}

pub trait ExtendSerializeSeq {
    type Ok;
    type Error: Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait ExtendSerializeTuple {
    type Ok;
    type Error: Error;
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait ExtendSerializeTupleStruct {
    type Ok;
    type Error: Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait ExtendSerializeTupleVariant {
    type Ok;
    type Error: Error;
    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait ExtendSerializeMap {
    type Ok;
    type Error: Error;
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize;
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize;
    fn serialize_entry<K, V>(&mut self, key: &K, value: &V) -> Result<(), Self::Error>
    where
        K: ?Sized + ExtendSerialize,
        V: ?Sized + ExtendSerialize,
    {
        r#try!(self.serialize_key(key));
        self.serialize_value(value)
    }
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait ExtendSerializeStruct {
    type Ok;
    type Error: Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize;
    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let _ = key;
        Ok(())
    }
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

pub trait ExtendSerializeStructVariant {
    type Ok;
    type Error: Error;
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + ExtendSerialize;
    #[inline]
    fn skip_field(&mut self, key: &'static str) -> Result<(), Self::Error> {
        let _ = key;
        Ok(())
    }
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

fn iterator_len_hint<I>(iter: &I) -> Option<usize>
where
    I: Iterator,
{
    match iter.size_hint() {
        (lo, Some(hi)) if lo == hi => Some(lo),
        _ => None,
    }
}
