use std::hash::Hasher;

use serde::{Deserialize, Serialize};
use half::f16;

use crate::traits::{ExtendSerialize, ExtendedSerializer};

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Int {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
}

#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum UInt {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

#[derive(Deserialize, Debug, Clone, PartialOrd)]
pub enum Float {
    F16(f16),
    F32(f32),
    F64(f64),
}

impl ExtendSerialize for Float {
    fn ex_serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ExtendedSerializer,
    {
        match self {
            Float::F16(v) => serializer.serialize_f16(*v),
            Float::F32(v) => serializer.serialize_f32(*v),
            Float::F64(v) => serializer.serialize_f64(*v),
        }
    }
    
}

impl Eq for Float {}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Float::F16(a), Float::F16(b)) => a.to_f32().partial_cmp(&b.to_f32()).unwrap(),
            (Float::F32(a), Float::F32(b)) => a.partial_cmp(b).unwrap(),
            (Float::F64(a), Float::F64(b)) => a.partial_cmp(b).unwrap(),
            _ => std::cmp::Ordering::Less,
        }
    }
}

impl std::hash::Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Float::F16(v) => (v.to_f32() as i32).hash(state),
            Float::F32(v) => (*v as i32).hash(state),
            Float::F64(v) => (*v as i64).hash(state),
        }
    }    
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Float::F16(a), Float::F16(b)) => a.to_f32() == b.to_f32(),
            (Float::F32(a), Float::F32(b)) => a == b,
            (Float::F64(a), Float::F64(b)) => a == b,
            _ => false,
        }
    }
}