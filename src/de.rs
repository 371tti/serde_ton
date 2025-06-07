use std::f32::consts::E;
use serde_cbor::de::Read;
use serde::de;

use crate::{error::{Error, ErrorCode}, value::prefix::prefix};

use crate::value::prefix::size_prefix::{SIZE_PREFIX_1BYTE, SIZE_PREFIX_2BYTE, SIZE_PREFIX_4BYTE, SIZE_PREFIX_8BYTE};



// pub struct Deserializer<R>
// where
//     R: de::Read,
// {
//     reader: R,
//     front_buffer: Vec<u8>,
//     back_buffer: Vec<u8>,
//     standard_pos: usize,
//     pos: usize,
//     deep: usize,
//     reverse: bool,
// }