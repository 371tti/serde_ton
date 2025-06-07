

use serde::ser::SerializeSeq;
use serde_ton::ser::{generate_header, ReverseSerializer};
use serde_ton::traits::{ExtendSerialize, ExtendedSerializer};
use serde_ton::value::prefix::prefix::DATETIME;
use std::io::Write;

use serde::{ser, Serialize, Serializer};

use serde_ton::{error::Error, value::prefix::prefix};
use serde_ton::value::prefix::size_prefix::{SIZE_PREFIX_1BYTE, SIZE_PREFIX_2BYTE, SIZE_PREFIX_4BYTE, SIZE_PREFIX_8BYTE};

use std::collections::HashMap;

// Test ReverseSerializer using an in-memory Vec<u8>
#[test]
fn test_serialize_bool() {
    let mut out = Vec::new();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_bool(true).unwrap();
        // Flush any remaining buffered bytes
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
    }
    // For none, we write one byte: [prefix::NONE]
    let expected = vec![prefix::NONE];
    assert_eq!(out, expected);
}

#[test]
fn test_serialize_some() {
    let mut out = Vec::new();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_some(&42u8).unwrap();
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
    }
    // For unit, we write one byte: [prefix::NONE]
    let expected = vec![prefix::NONE];
    assert_eq!(out, expected);
}

#[test]
fn test_serialize_unit_struct() {
    let mut out = Vec::new();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_unit_struct("Unit").unwrap();
    }
    // For unit struct, we write one byte: [prefix::NONE]
    let expected = vec![prefix::NONE];
    assert_eq!(out, expected);
}

#[test]
fn test_serialize_unit_variant() {
    let mut out = Vec::new();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_unit_variant("Unit", 0, "Variant").unwrap();
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
    }
    // display 
    // ["Helllo, world!", 13, STRING | SIZE_PREFIX_1BYTE,
    //  "field1", 6, STRING | SIZE_PREFIX_1BYTE,
    //  `42`, UINT | SIZE_PREFIX_1BYTE,
    //  "field2", 6, STRING | SIZE_PREFIX_1BYTE,
    //  `33`, OBJECT | SIZE_PREFIX_1BYTE]
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
fn test_serialize_f16() {
    let mut out = Vec::new();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_f16(half::f16::from_f32(42.0f32)).unwrap();
    }
    // For f16, we write five bytes: [42.0 bytes in little endian, prefix::FLOAT | SIZE_PREFIX_4BYTE]
    let f16_bytes = half::f16::from_f32(42.0f32).to_le_bytes();
    let expected = vec![f16_bytes[0], f16_bytes[1], prefix::FLOAT | SIZE_PREFIX_2BYTE];
    assert_eq!(out, expected);
}

#[test]
fn test_serialize_uuid() {
    let mut out = Vec::new();
    let uuid = uuid::Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_uuid(&uuid).unwrap();
    }
    // For UUID, we write 17 bytes: [UUID bytes in little endian, prefix::UUID | SIZE_PREFIX_1BYTE]
    let uuid_bytes = uuid.as_bytes();
    let mut expected = Vec::with_capacity(17);
    expected.extend_from_slice(uuid_bytes);
    expected.push(prefix::UUID | SIZE_PREFIX_1BYTE);
}

#[test]
fn test_serialize_datetime() {
    let mut out = Vec::new();
    let datetime = chrono::Utc::now();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_datetime(&datetime).unwrap();
    }
    // For DateTime, we write 17 bytes: [DateTime bytes in little endian, prefix::DATETIME | SIZE_PREFIX_1BYTE]
    let datetime_bytes = datetime.to_rfc3339().as_bytes().to_vec();
    let len = datetime_bytes.len();
    let (header, header_size) = generate_header(DATETIME, len);
    let value = datetime_bytes.iter().chain(header[..header_size].iter()).cloned().collect::<Vec<_>>();
    assert_eq!(out, value);
}

#[test]
fn test_serialize_timestamp() {
    let mut out = Vec::new();
    let timestamp = chrono::Utc::now().timestamp();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_timestamp(timestamp).unwrap();
    }
    // For Timestamp, we write 9 bytes: [Timestamp bytes in little endian, prefix::TIMESTAMP | SIZE_PREFIX_1BYTE]
    let timestamp_bytes = timestamp.to_le_bytes();
    let expected = vec![timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3], timestamp_bytes[4], timestamp_bytes[5], timestamp_bytes[6], timestamp_bytes[7], prefix::TIMESTAMP | SIZE_PREFIX_8BYTE];
    assert_eq!(out, expected);
}

#[test]
fn test_serialize_duration() {
    let mut out = Vec::new();
    let duration = chrono::Duration::seconds(42);
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_duration(&duration).unwrap();
    }
    // For Duration, we write 9 bytes: [Duration bytes in little endian, prefix::DURATION | SIZE_PREFIX_1BYTE]
    let duration_bytes = duration.num_nanoseconds().unwrap().to_le_bytes();
    let expected = vec![duration_bytes[0], duration_bytes[1], duration_bytes[2], duration_bytes[3], duration_bytes[4], duration_bytes[5], duration_bytes[6], duration_bytes[7], prefix::DURATION | SIZE_PREFIX_8BYTE];
    assert_eq!(out, expected);
}

#[test]
fn test_serialize_wrapped_json() {
    let mut out = Vec::new();
    let json_value = serde_json::json!({"key": "value"});
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_wrapped_json(&json_value).unwrap();
    }
    // For Wrapped JSON, we write the JSON bytes followed by the prefix
    let json_bytes = serde_json::to_vec(&json_value).unwrap();
    let len = json_bytes.len();
    let (header, header_size) = generate_header(prefix::WRAPPED_JSON, len);
    let value = json_bytes.iter().chain(header[..header_size].iter()).cloned().collect::<Vec<_>>();
    assert_eq!(out, value);
}

#[test]
fn test_serialize_meta() {
    let mut out = Vec::new();
    let meta_value = serde_ton::value::value::Value::Meta(
        Box::new(serde_ton::value::value::Value::String("meta_value".to_string()))
    );
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        meta_value.ex_serialize(&mut serializer).unwrap();
    }
    // For Meta, we write the meta value bytes followed by the prefix
    let expected = vec![b'm', b'e', b't', b'a', b'_', b'v', b'a', b'l', b'u', b'e', 10, prefix::STRING | SIZE_PREFIX_1BYTE, 12, prefix::META | SIZE_PREFIX_1BYTE];
    assert_eq!(out, expected);
}

#[test]
fn test_serialize_padding() {
    let mut out = Vec::new();
    {
        let mut serializer = ReverseSerializer::new(&mut out);
        serializer.serialize_padding(5).unwrap();
    }
    // For padding, we write 5 bytes of zero followed by the prefix
    let expected = vec![0, 0, 0, 0, 0, 5, prefix::PADDING | SIZE_PREFIX_1BYTE];
    assert_eq!(out, expected);
}




#[test]
fn test_generate_header() {
    let (header, header_size) = generate_header(prefix::UINT, 42);
    let expected = vec![42, prefix::UINT | SIZE_PREFIX_1BYTE];
    assert_eq!(header[..header_size].to_vec(), expected);

    let (header, header_size) = generate_header(prefix::UINT, u8::MAX as usize + 1);
    let expected = vec![0, 1, prefix::UINT | SIZE_PREFIX_2BYTE];
    assert_eq!(header[..header_size].to_vec(), expected);

    let (header, header_size) = generate_header(prefix::UINT, u16::MAX as usize + 1);
    let expected = vec![0, 0, 1, 0, prefix::UINT | SIZE_PREFIX_4BYTE];
    assert_eq!(header[..header_size].to_vec(), expected);

    let (header, header_size) = generate_header(prefix::UINT, u32::MAX as usize + 1);
    let expected = vec![0, 0, 0, 0, 1, 0, 0, 0, prefix::UINT | SIZE_PREFIX_8BYTE];
    assert_eq!(header[..header_size].to_vec(), expected);
}
