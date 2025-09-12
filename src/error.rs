use std::{io, num::ParseIntError, string};

use thiserror::Error;

use regex;

#[derive(Error, Debug)]
pub enum DCPMError {
    #[error("IOError")]
    IOError(#[from] io::Error),
    #[error("DockerError")]
    DockerError(String),
    #[error("UTF8Error")]
    UTF8Error(#[from] string::FromUtf8Error),
    #[error("RegexError")]
    RegexError(#[from] regex::Error),
    #[error("ParseIntError")]
    ParseIntError(#[from] ParseIntError),
    #[error("FileIOError")]
    FileIOError(String),
    #[error("ReadError")]
    ReadError,
    #[error("MapError")]
    MapError(String)
}
