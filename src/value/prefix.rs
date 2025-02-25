pub mod prefix {
    pub const UNDEFINED:        u8 = 0b111111_00; // 0xFC
    pub const NULL:             u8 = 0b000000_00; // 0x00
    pub const BOOL:             u8 = 0b000001_00; // 0x04 ~ 0x05
    pub const INT:              u8 = 0b000010_00; // 0x08 ~ 0x0B
    pub const UINT:             u8 = 0b000011_00; // 0x0C ~ 0x0F
    pub const FLOAT:            u8 = 0b000100_00; // 0x10 ~ 0x13
    pub const STRING:           u8 = 0b000101_00; // 0x14 ~ 0x17
    pub const BYTES:            u8 = 0b000110_00; // 0x18 ~ 0x1B
    pub const UUID:             u8 = 0b000111_00; // 0x1C ~ 0x1F
    pub const DATETIME:         u8 = 0b001000_00; // 0x20 ~ 0x23
    pub const TIMESTAMP:        u8 = 0b001001_00; // 0x24 ~ 0x27
    pub const DURATION:         u8 = 0b001010_00; // 0x28 ~ 0x2B
    pub const ARRAY:            u8 = 0b001011_00; // 0x2C ~ 0x2F
    pub const OBJECT:           u8 = 0b001100_00; // 0x30 ~ 0x33
    pub const WRAPPED_JSON:     u8 = 0b001101_00; // 0x34 ~ 0x37

    pub const META:             u8 = 0b001110_00; // 0x38 ~ 0x3B
    pub const PADDING:          u8 = 0b001111_00; // 0x3C ~ 0x3F
}

pub mod prefix_pua_utf8 {
    // 私用領域 (PUA)
    pub const UNDEFINED:        &str = "$undefined"; // 私用領域 (開始)
    pub const NULL:             &str = "$null";      // NULL (カスタム)
    pub const BOOL:             &str = "$bool";      // Boolean
    pub const INT:              &str = "$int";       // Integer
    pub const UINT:             &str = "$uint";      // Unsigned Integer
    pub const FLOAT:            &str = "$float";     // Floating point
    pub const STRING:           &str = "$string";    // String
    pub const BYTES:            &str = "$bytes";     // Bytes
    pub const UUID:             &str = "$uuid";      // UUID
    pub const DATETIME:         &str = "$datetime";  // DateTime
    pub const TIMESTAMP:        &str = "$timestamp"; // Timestamp
    pub const DURATION:         &str = "$duration";  // Duration
    pub const ARRAY:            &str = "$array";     // Array
    pub const OBJECT:           &str = "$object";    // Object
    pub const WRAPPED_JSON:     &str = "$wrapped_json"; // Wrapped JSON

    pub const META:             &str = "$meta";      // Meta
    pub const PADDING:          &str = "$padding";   // Padding
}
