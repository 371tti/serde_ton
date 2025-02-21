# Value Enum 使用法

以下のテーブルは各 Variant の用途と補足情報をまとめたものです。

| Variant     | 用途                                                    | 補足情報                                    |
|-------------|---------------------------------------------------------|---------------------------------------------|
| Undefined   | 定義があいまいな場合、不正な状態を示す                  | サイズ: 0 byte                              |
| Null        | 値が存在しない場合を示す                                | サイズ: 0 byte                              |
| Bool        | 真偽値 (true/false) を保持                              | 固定サイズ、BoolType で size 書込必要         |
| Int         | 符号付き整数を表現                                      | サイズ: 1～16 byte（i8, i16, …）              |
| UInt        | 符号なし整数を表現                                      | サイズ: 1～16 byte（u8, u16, …）              |
| Float       | 浮動小数点数を表現                                      | サイズ: 2～8 byte（f16, f32, f64, …）         |
| String      | 文字列 (UTF-8 エンコーディング) を保持                  | サイズ: 可変                                 |
| Bytes       | バイト列を保持                                          | Vec<u8>、サイズ: 可変                        |
| UUID        | 16バイト固定の UUID を保持                              | サイズ: 16 byte                             |
| DateTime    | ISO8601 準拠の日付と時刻を表現                           | DateTime<Utc>、サイズ: 可変                   |
| Timestamp   | POSIX タイムスタンプを表現                              | サイズ: 8 byte                              |
| Duration    | ナノ秒単位の期間を表現                                  | サイズ: 8 byte                              |
| Array       | 複数の Value を保持する配列                              | Vec<Value>、サイズ: 可変                      |
| Object      | キーと値の組を保持するオブジェクト                      | HashMap<Value, Value>、サイズ: 可変          |
| WrappedJSON | serde_json::Value をラップする JSON 型                  | サイズ: 可変                                 |
| Meta        | 他の Value のメタデータとしてラップする                | Box<Value>                                  |

## バイナリ表現

ton の Value は以下のバイナリ構造となっています:

<prefix(型情報)><size-prefix(サイズ範囲情報)><size(実際のデータサイズを表す可変長値)><data(実際データ)>
