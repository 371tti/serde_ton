use std::io::{Error, Read, Seek};

/// Read トレイとの実装
/// 必要機能
/// - seek
/// - no copy read
/// えっとシーク位置の指定はこれでよくて
/// RTON自体は必要な長さのデータだけをほしい
/// バッファリングするのがクソなので
/// で、だから必ず指定するlenの長さのデータを受け取るようにしたいけど
/// これはパフォーマンス的によくない
/// なぜよくないのか
/// 糞でかい場合シンプルにメモリを食う
/// 向こう側がバッファリングしている場合は考慮不要
/// なのでサイズは指定しないで、適当にむこうが渡してくるデータ長でうまく処理できるようにするしかなさそう
pub trait Reader: Read + Seek{
    /// 次のバイトを読み込み、シーク位置を1バイト進めます
    /// seekに失敗した場合 seekの位置を戻しません
    fn next(&mut self) -> Result<Option<u8>, Error> {
        let res = self.peek()?;
        if let Some(_) = res {
            // 1バイト進めてみて、範囲外なら元に戻す
            let pos = self.stream_position()?;
            self.seek(std::io::SeekFrom::Current(1))?;
        }
        Ok(res)
    }

    /// 前のバイトを読み込み、シークを現在から負の方向に1バイト進めます
    /// seekに失敗した場合 seekの位置を戻しません
    fn prev(&mut self) -> Result<Option<u8>, Error> {
        if self.stream_position()? == 0 {
            return Ok(None); // すでに先頭なので戻れない
        }
        self.seek(std::io::SeekFrom::Current(-1))?;
        let res = self.peek()?;
        Ok(res)
    }

    /// 現在の位置のバイトを読み込み、シーク位置を戻します
    fn peek(&mut self) -> Result<Option<u8>, Error> {
        let mut buf = [0; 1];
        match self.read(&mut buf) {
            Ok(0) => Ok(None), // EOF
            Ok(_) => {
                Ok(Some(buf[0]))
            },
            Err(e) => Err(Error::new(e.kind(), format!("Read error: {}", e))),
        }
    }

    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_u16(&mut self) -> Result<u16, Error> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    fn read_u32(&mut self) -> Result<u32, Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    
    fn read_u64(&mut self) -> Result<u64, Error> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_le_bytes(buf))
    }
}

pub struct SliceReader<'a> {
    slice: &'a [u8],
    pos: usize,
}

impl<'a> SliceReader<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self { slice, pos: 0 }
    }

    pub fn into_inner(self) -> &'a [u8] {
        self.slice
    }
}

impl Read for SliceReader<'_> {
    /// Readerの実装
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        if self.pos >= self.slice.len() {
            return Ok(0); // EOF
        }
        let bytes_to_read = buf.len().min(self.slice.len() - self.pos);
        buf[..bytes_to_read].copy_from_slice(&self.slice[self.pos..self.pos + bytes_to_read]);
        Ok(bytes_to_read)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        if self.pos + buf.len() > self.slice.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data to read"));
        }
        buf.copy_from_slice(&self.slice[self.pos..self.pos + buf.len()]);
        Ok(())
    }
}

impl Seek for SliceReader<'_> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            std::io::SeekFrom::Start(offset) => offset as usize,
            std::io::SeekFrom::End(offset) => {
                if offset < 0 {
                    self.slice.len().checked_sub(-offset as usize).unwrap_or(0)
                } else {
                    self.slice.len().saturating_add(offset as usize)
                }
            }
            std::io::SeekFrom::Current(offset) => {
                if offset < 0 {
                    self.pos.checked_sub(-offset as usize).unwrap_or(0)
                } else {
                    (self.pos + offset as usize).min(self.slice.len())
                }
            }
        };
        self.pos = new_pos;
        Ok(self.pos as u64)
    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        Ok(self.pos as u64)
    }

    fn rewind(&mut self) -> std::io::Result<()> {
        self.pos = 0;
        Ok(())
    }

    fn seek_relative(&mut self, offset: i64) -> std::io::Result<()> {
        let new_pos = if offset < 0 {
            self.pos.checked_sub(-offset as usize).unwrap_or(0)
        } else {
            (self.pos + offset as usize).min(self.slice.len())
        };
        self.pos = new_pos;
        Ok(())
    }
}

impl Reader for SliceReader<'_> {
    fn next(&mut self) -> Result<Option<u8>, Error> {
        if self.pos >= self.slice.len() {
            return Ok(None); // EOF
        }
        let byte = self.slice[self.pos];
        self.pos += 1;
        Ok(Some(byte))
    }

    fn prev(&mut self) -> Result<Option<u8>, Error> {
        if self.pos == 0 {
            return Ok(None); // EOF
        }
        self.pos -= 1;
        Ok(Some(self.slice[self.pos]))
    }

    fn peek(&mut self) -> Result<Option<u8>, Error> {
        if self.pos >= self.slice.len() {
            return Ok(None); // EOF
        }
        Ok(Some(self.slice[self.pos]))
    }
}

pub struct VecReader<'a> {
    vec: &'a Vec<u8>,
    pos: usize,
}

impl<'a> VecReader<'a> {
    pub fn new(vec: &'a Vec<u8>) -> Self {
        Self { vec, pos: 0 }
    }

    pub fn into_inner(self) -> &'a Vec<u8> {
        self.vec
    }
}

impl Read for VecReader<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        if self.pos >= self.vec.len() {
            return Ok(0); // EOF
        }
        let bytes_to_read = buf.len().min(self.vec.len() - self.pos);
        buf[..bytes_to_read].copy_from_slice(&self.vec[self.pos..self.pos + bytes_to_read]);
        Ok(bytes_to_read)
    }

    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        if self.pos + buf.len() > self.vec.len() {
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Not enough data to read"));
        }
        buf.copy_from_slice(&self.vec[self.pos..self.pos + buf.len()]);
        Ok(())
    }
}

impl Seek for VecReader<'_> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        let new_pos = match pos {
            std::io::SeekFrom::Start(offset) => offset as usize,
            std::io::SeekFrom::End(offset) => {
                if offset < 0 {
                    self.vec.len().checked_sub(-offset as usize).unwrap_or(0)
                } else {
                    self.vec.len().saturating_add(offset as usize)
                }
            }
            std::io::SeekFrom::Current(offset) => {
                if offset < 0 {
                    self.pos.checked_sub(-offset as usize).unwrap_or(0)
                } else {
                    (self.pos + offset as usize).min(self.vec.len())
                }
            }
        };
        self.pos = new_pos;
        Ok(self.pos as u64)
    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        Ok(self.pos as u64)
    }

    fn rewind(&mut self) -> std::io::Result<()> {
        self.pos = 0;
        Ok(())
    }

    fn seek_relative(&mut self, offset: i64) -> std::io::Result<()> {
        let new_pos = if offset < 0 {
            self.pos.checked_sub(-offset as usize).unwrap_or(0)
        } else {
            (self.pos + offset as usize).min(self.vec.len())
        };
        self.pos = new_pos;
        Ok(())
    }
}

impl Reader for VecReader<'_> {
    fn next(&mut self) -> Result<Option<u8>, Error> {
        if self.pos >= self.vec.len() {
            return Ok(None); // EOF
        }
        let byte = self.vec[self.pos];
        self.pos += 1;
        Ok(Some(byte))
    }

    fn prev(&mut self) -> Result<Option<u8>, Error> {
        if self.pos == 0 {
            return Ok(None); // EOF
        }
        self.pos -= 1;
        Ok(Some(self.vec[self.pos]))
    }

    fn peek(&mut self) -> Result<Option<u8>, Error> {
        if self.pos >= self.vec.len() {
            return Ok(None); // EOF
        }
        Ok(Some(self.vec[self.pos]))
    }
}

pub struct IOReader<R>
where R: Read + Seek,
{
    reader: R,
}

impl<R> IOReader<R>
where R: Read + Seek,
{
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn into_inner(self) -> R {
        self.reader
    }
}

impl<R> Read for IOReader<R>
where R: Read + Seek,
{
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.reader.read(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.reader.read_exact(buf)
    }      
}

impl<R> Seek for IOReader<R>
where R: Read + Seek,
{
    #[inline]
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.reader.seek(pos)
    }

    #[inline]
    fn stream_position(&mut self) -> std::io::Result<u64> {
        self.reader.stream_position()
    }

    #[inline]
    fn rewind(&mut self) -> std::io::Result<()> {
        self.reader.rewind()
    }

    #[inline]
    fn seek_relative(&mut self, offset: i64) -> std::io::Result<()> {
        self.reader.seek_relative(offset)
    }   
}

impl <R> Reader for IOReader<R>
where R: Read + Seek,
{
    fn next(&mut self) -> Result<Option<u8>, Error> {
        let mut buf = [0; 1];
        match self.reader.read(&mut buf) {
            Ok(0) => Ok(None), // EOF
            Ok(_) => {
                self.reader.seek(std::io::SeekFrom::Current(1))?;
                Ok(Some(buf[0]))
            },
            Err(e) => Err(Error::new(e.kind(), format!("Read error: {}", e))),
        }
    }

    fn prev(&mut self) -> Result<Option<u8>, Error> {
        let pos = self.reader.stream_position()?;
        if pos == 0 {
            return Ok(None); // すでに先頭なので戻れない
        }
        self.reader.seek(std::io::SeekFrom::Current(-1))?;
        let mut buf = [0; 1];
        match self.reader.read(&mut buf) {
            Ok(0) => {
                Ok(None)
            },
            Ok(_) => {
                Ok(Some(buf[0]))
            },
            Err(e) => Err(Error::new(e.kind(), format!("Read error: {}", e))),
        }
    }

    fn peek(&mut self) -> Result<Option<u8>, Error> {
        let mut buf = [0; 1];
        match self.reader.read(&mut buf) {
            Ok(0) => Ok(None), // EOF
            Ok(_) => {
                Ok(Some(buf[0]))
            },
            Err(e) => Err(Error::new(e.kind(), format!("Read error: {}", e))),
        }
    }
}