use std::{fs::File, io::{self, Error}};




/// RTON のデシリアライザ
/// 後ろから読み込む必要がある
/// ファイルとかだとseek位置を動かしてバッファリングすべき
/// バッファリングは外部ですべき - 柔軟性
/// seekトレイとが実装されてる標準型が少ないのが問題
/// 楽に使うために変換できるように
/// 
pub struct ReverseDeserializer<R>
where R: io::Read + io::Seek,
{
    reader: R,
    deep: u64,
}

impl ReverseDeserializer<io::Cursor<Vec<u8>>> 
{
    pub fn from_vector(vec: Vec<u8>) -> Result<Self, Error> {
        let reader = io::Cursor::new(vec);
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.reader.into_inner()
    }
}

impl<'a> ReverseDeserializer<io::Cursor<&'a [u8]>> 
{
    pub fn from_slice(slice: &'a [u8]) -> Result<Self, Error> {
        let reader = io::Cursor::new(slice);
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> &'a [u8] {
        self.reader.into_inner()
    }
}

impl ReverseDeserializer<File> 
{
    pub fn from_file(file: File) -> Result<Self, Error> {
        let reader = file;
        Ok(Self { reader, deep: 0 })
    }

    pub fn into_inner(self) -> File {
        self.reader
    }
}

impl<R> ReverseDeserializer<R>
where R: io::Read + io::Seek,
{
    pub fn new(reader: R) -> Result<Self, Error> {
        Ok(Self { reader, deep: 0 })
    }

    pub fn now_pos(&mut self) -> Result<u64, Error> {
        self.reader.stream_position()
    }

    pub fn get_size(reader: &mut R) -> io::Result<u64> {
        let current = reader.stream_position()?;
        let end = reader.seek(io::SeekFrom::End(0))?;
        reader.seek(io::SeekFrom::Start(current))?;
        Ok(end)
    }

    pub fn read_header(&mut self) -> Result<(u64, u8), Error> {
        // ここでヘッダーを読み込む処理を実装する
        let pos = self.reader.stream_position()?;
        Ok((0, 0)) // 仮の実装
    }
}