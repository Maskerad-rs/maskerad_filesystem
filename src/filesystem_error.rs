// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::error::Error;
use std::fmt;
use std::io::Error as IOError;
use std::env::VarError;

#[derive(Debug)]
pub enum FSErrorKind {
    GameDirectoryError(String),
    CreationError(String),
    IOError(String, IOError),
    EnvironmentError(String, VarError),
    MiscellaneousError(String),
}

impl fmt::Display for FSErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FSErrorKind::GameDirectoryError(ref description) => write!(f, "Game directory error: {}", description),
            &FSErrorKind::CreationError(ref description) => write!(f, "Creation error: {}", description),
            &FSErrorKind::EnvironmentError(ref description, _) => write!(f, "Environment variable error: {}", description),
            &FSErrorKind::IOError(ref description, _) => write!(f, "I/O error: {}", description),
            &FSErrorKind::MiscellaneousError(ref description) => write!(f, "Miscellaneous Error: {}", description),
        }

    }
}

impl Error for FSErrorKind {
    fn description(&self) -> &str {
        match self {
            &FSErrorKind::GameDirectoryError(_) => "GameDirectoryError",
            &FSErrorKind::CreationError(_) => "CreationError",
            &FSErrorKind::EnvironmentError(_, _) => "EnvironmentError",
            &FSErrorKind::IOError(_, _) => "IOError",
            &FSErrorKind::MiscellaneousError(_) => "MiscellaneousError",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &FSErrorKind::GameDirectoryError(_) => None,
            &FSErrorKind::CreationError(_) => None,
            &FSErrorKind::IOError(_, ref cause) => Some(cause),
            &FSErrorKind::EnvironmentError(_, ref cause) => Some(cause),
            &FSErrorKind::MiscellaneousError(_) => None,
        }
    }
}

impl From<IOError> for FSErrorKind {
    fn from(error: IOError) -> Self {
        FSErrorKind::IOError(format!("Error while doing I/O operations"), error)
    }
}

impl From<VarError> for FSErrorKind {
    fn from(error: VarError) -> Self {
        FSErrorKind::EnvironmentError(format!("Error while dealing with environment variable"), error)
    }
}


#[derive(Debug)]
pub struct FileSystemError {
    cause: FSErrorKind,
}

impl FileSystemError {
    pub fn new(cause: FSErrorKind) -> Self {
        FileSystemError {
            cause,
        }
    }
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while dealing with the file system: {}", self.cause)
    }
}

impl Error for FileSystemError {
    fn description(&self) -> &str {
        "FileSystemError"
    }

    fn cause(&self) -> Option<&Error> {
        Some(&self.cause)
    }
}

pub type FileSystemResult<T> = Result<T, FileSystemError>;


impl From<IOError> for FileSystemError {
    fn from(error: IOError) -> Self {
        FileSystemError::new(FSErrorKind::from(error))
    }
}

impl From<VarError> for FileSystemError {
    fn from(error: VarError) -> Self {
        FileSystemError::new(FSErrorKind::from(error))
    }
}
