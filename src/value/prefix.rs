pub mod prefix {
    pub const UNDEFINED:        u8 = 0b111111_00;
    pub const NULL:             u8 = 0b000000_00;
    pub const BOOL:             u8 = 0b000001_00;
    pub const INT:              u8 = 0b000010_00;
    pub const UINT:             u8 = 0b000011_00;
    pub const FLOAT:            u8 = 0b000100_00;
    pub const STRING:           u8 = 0b000101_00;
    pub const BYTES:            u8 = 0b000110_00;
    pub const UUID:             u8 = 0b000111_00;
    pub const DATETIME:         u8 = 0b001000_00;
    pub const TIMESTAMP:        u8 = 0b001001_00;
    pub const DURATION:         u8 = 0b001010_00;
    pub const ARRAY:            u8 = 0b001011_00;
    pub const OBJECT:           u8 = 0b001100_00;
    pub const WRAPPED_JSON:     u8 = 0b001101_00;

    pub const META:             u8 = 0b001110_00;
    pub const PADDING:          u8 = 0b001111_00;
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
