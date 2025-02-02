#[cfg(target_os = "linux")]
use ini::Error as IniError;
use std::{
    io::Error as IOError,
    result::Result as StdResult,
    string::FromUtf8Error,
};
use thiserror::Error;
use which::Error as WhichError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Getting the command output failed! Error: {0:#?}")]
    CommandOutputGet(#[from] IOError),
    #[error("Parsing the command output as UTF-8 failed! Error: {0:#?}")]
    CommandOutputParse(#[from] FromUtf8Error),
    #[error("Path to your OS' package manager was not found! Error: {0:#?}")]
    PackageManagerPathNotFound(#[from] WhichError),
    #[error("Your operating system is unsupported!")]
    UnsupportedOS,
    #[error("Couldn't load the given ini file! Error: {0:#?}")]
    IniLoadFailure(#[from] IniError),
}

pub type Result<T> = StdResult<T, Error>;
