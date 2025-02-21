
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// Undefined 型
    /// 値が未定義であることを示す型
    /// type size: 0 byte
    /// 
    /// 値についての定義があいまいな場合に使用されます。
    /// 値が存在しない場合や、値が不正な場合に使用されます。
    Undefined,

    /// Null 型
    /// 値がnullであることを示す型
    /// type size: 0 byte
    /// 
    /// 値が存在しないことを示すために使用されます。
    Null,

    /// Boolean 型
    /// 値が真偽値であることを示す型
    /// type size: 0 byte
    /// 
    /// 値が真または偽であることを示すために使用されます。
    /// 型サイズが0バイトなのはサイズが固定であるため、
    /// BoolTypeにおいてはsize フィールドに値を書き込むようにするためです。
    Bool(bool),

    /// Integer 型
    /// 値が整数であることを示す型
    /// type size: 1..16 byte
    /// 
    /// i8, i16, i32, i64, i128 などの整数型を表現できます。
    Int(Int),

    /// Unsigned Integer 型
    /// 値が符号なし整数であることを示す型
    /// type size: 1..16 byte
    /// 
    /// u8, u16, u32, u64, u128 などの符号なし整数型を表現できます。
    UInt(UInt),

    /// Float 型
    /// 値が浮動小数点数であることを示す型
    /// type size: 2..8 byte
    /// 
    /// f16, f32, f64 などの浮動小数点数型を表現できます。
    Float(Float),

    /// String 型
    /// 値が文字列であることを示す型
    /// type size: n byte
    /// 
    /// 文字列のエンコーディングはUTF-8です。
    String(String),

    /// Bytes 型
    /// 値がバイト列であることを示す型
    /// type size: n byte
    /// 
    Bytes(Vec<u8>),

    /// UUID 型
    /// 値がUUIDであることを示す型
    /// type size: 16 byte
    /// 
    UUID(Uuid),

    /// Date and Time Types
    /// 値が日付または時刻であることを示す型
    /// type size: n byte
    /// 
    /// ISO8601 equivalent に準拠しています。
    /// UTF-8 エンコーディングを使用します。
    DateTime(DateTime<Utc>),

    /// Timestamp 型
    /// 値がタイムスタンプであることを示す型
    /// type size: 8 byte
    /// 
    /// POSIX タイムスタンプを使用します。
    /// Secconds-percentage の形式で表現されます。
    Timestamp(i64),

    /// Duration 型
    /// 値が期間であることを示す型
    /// type size: 8 byte
    /// 
    /// 時間の長さを表現します。
    /// nanoseconds で表現されます。
    Duration(Duration),


    /// Array 型
    /// 値が配列であることを示す型
    Array(Vec<Value>),

    /// Object 型
    /// 値がオブジェクトであることを示す型
    Object(HashMap<Value, Value>),

    /// Wrapped JSON 型
    /// 値が JSON であることを示す型
    WrappedJSON(serde_json::Value),

    /// Meta 型
    /// 値がメタデータであることを示す型
    Meta(Box<Value>),
}