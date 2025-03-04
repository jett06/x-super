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
    #[error("The path to the given executable was not found! Error: {0:#?}")]
    ExecutablePathNotFound(#[from] WhichError),
    #[error("Your operating system is unsupported!")]
    UnsupportedOS,
    #[cfg(target_os = "linux")]
    #[error("Couldn't load the given ini file! Error: {0:#?}")]
    IniLoadFailure(#[from] IniError),
    #[error("Failed to elevate to superuser!")]
    ElevationFailed,
    #[error("The given elevation handler name (`{0}`) is unrecognized!")]
    UnrecognizedElevationHandlerName(String),
}

pub type Result<T> = StdResult<T, Error>;
