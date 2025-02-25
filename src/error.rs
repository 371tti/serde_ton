use core::fmt;
use std::{fmt::{Debug, Display}, io, result};
use std::str::FromStr;

use serde::{de, ser};

pub struct Error {
    err: Box<ErrorImpl>,
}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn pos(&self) -> usize {
        self.err.pos
    }

    pub fn classify(&self) -> Category {
        match &self.err.code {
            ErrorCode::Message(_) => Category::InvalidType,
            ErrorCode::Io(_) => Category::Io,
        }
    }

    pub fn is_io(&self) -> bool {
        self.classify() == Category::Io
    }

    pub fn is_syntax(&self) -> bool {
        self.classify() == Category::Syntax
    }

    pub fn is_type(&self) -> bool {
        self.classify() == Category::InvalidType
    }

    pub fn is_eof(&self) -> bool {
        self.classify() == Category::Eof
    }

    pub fn is_unknown_format(&self) -> bool {
        self.classify() == Category::UnknownFormat
    }

}

pub struct ErrorImpl {
    code: ErrorCode,
    pos: usize,
}

pub(crate) enum ErrorCode {
    Message(String),
    Io(io::Error),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// IOエラー
    Io,

    /// 不正なフォーマット
    Syntax,

    /// 不正な型
    InvalidType,

    /// 不明なフォーマット
    /// 
    /// 基本のフォーマットには従ってるがprefixなどが不明な場合
    UnknownFormat,

    /// 終端エラー
    /// 
    /// ファイルの終端に達した場合
    Eof,
}

impl Error {
    #[cold]
    pub(crate) fn syntax(code: ErrorCode, pos: usize) -> Self {
        Self {
            err: Box::new(ErrorImpl {
                code,
                pos,
            }),
        }
    }

    #[cold]
    pub(crate) fn io(error: io::Error) -> Self {
        Error {
            err: Box::new(ErrorImpl {
                code: ErrorCode::Io(error),
                pos: 0,
            }),
        }
    }

    #[cold]
    pub(crate) fn fix_position<F>(self, f: F) -> Self
    where
        F: FnOnce(ErrorCode) -> Error,
    {
        if self.err.pos == 0 {
            f(self.err.code)
        } else {
            self
        }
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Message(msg) => f.write_str(msg),
            ErrorCode::Io(err) => Display::fmt(err, f),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {{ code: {:?}, pos: {} }}",
            self.err.code.to_string(),
            self.err.pos
        )
    }
}

impl serde::de::StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&*self.err, f)
    }
}

impl Display for ErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {  
        if self.pos == 0 {
            Display::fmt(&self.code, f)
        } else {
            write!(f, "{} at position {}", self.code, self.pos)
        }
    }
}

impl de::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Self {
        make_error(msg.to_string())
    }
}

impl ser::Error for Error {
    #[cold]
    fn custom<T: Display>(msg: T) -> Self {
        make_error(msg.to_string())
    }
}

fn make_error(mut msg: String) -> Error {
    let pos = parse_pos(&mut msg).unwrap_or(0);
    Error {
        err: Box::new(ErrorImpl {
            code: ErrorCode::Message(msg),
            pos,
        }),
    }
}

fn parse_pos(msg: &mut String) -> Option<usize> {
    let start_of_suffix = match msg.rfind(" at pos ") {
        Some(index) => index,
        None => return None, // " at pos " が見つからなければ解析できない
    };

    // " at pos " の直後にある数値を探す
    let start_of_pos = start_of_suffix + " at pos ".len();
    let mut end_of_pos = start_of_pos;

    while msg[end_of_pos..].starts_with(|c: char| c.is_ascii_digit()) {
        end_of_pos += 1;
    }

    // 数値を `usize` に変換
    let pos = match usize::from_str(&msg[start_of_pos..end_of_pos]) {
        Ok(pos) => pos,
        Err(_) => return None, // 数値のパースに失敗したら None を返す
    };

    // `msg` から " at pos X" を削除
    msg.truncate(start_of_suffix);
    Some(pos)
}