use std::{io, string};

use thiserror::Error;

use regex;

#[derive(Error, Debug)]
pub enum DCPMError {
    #[error("ShellError")]
    ShellError(#[from] io::Error),
    #[error("DockerError")]
    DockerError(String),
    #[error("UTF8Error")]
    UTF8Error(#[from] string::FromUtf8Error),
    #[error("RegexError")]
    RegexError(#[from] regex::Error),
    #[error("ParseIntError")]
    ParseIntError(String),
}
