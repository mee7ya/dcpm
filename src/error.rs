use std::{io, string};

use thiserror::Error;

use regex;

#[derive(Error, Debug)]
pub enum DCPMError {
    #[error("IOError")]
    IOError(#[from] io::Error),
    #[error("ShellError")]
    ShellError(String),
    #[error("DockerError")]
    DockerError(String),
    #[error("UTF8Error")]
    UTF8Error(#[from] string::FromUtf8Error),
    #[error("RegexError")]
    RegexError(#[from] regex::Error),
    #[error("FileIOError")]
    FileIOError(String),
    #[error("MapError")]
    MapError(String),
}
