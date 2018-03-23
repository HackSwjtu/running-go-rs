use std::io;
use std::convert::From;
use std::option::NoneError;
use std::num::{ParseFloatError, ParseIntError};
use time;
use serde_ini;
use reqwest;
use json;

#[derive(Debug)]
pub enum Error {
    IO(reqwest::Error),
    Parse(json::Error),
    Api(String),
    Format(String),
    Config(String),
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::Format("invalid format".into())
    }
}

impl From<ParseFloatError> for Error {
    fn from(error: ParseFloatError) -> Self {
        Error::Format(format!("{}", error))
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::Format(format!("{}", error))
    }
}

impl From<time::ParseError> for Error {
    fn from(error: time::ParseError) -> Self {
        Error::Format(format!("{}", error))
    }
}
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::IO(error)
    }
}

impl From<json::Error> for Error {
    fn from(error: json::Error) -> Self {
        Error::Parse(error)
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::Config(format!("{}", error))
    }
}

impl From<serde_ini::de::Error> for Error {
    fn from(error: serde_ini::de::Error) -> Self {
        Error::Config(format!("{}", error))
    }
}

impl From<serde_ini::ser::Error> for Error {
    fn from(error: serde_ini::ser::Error) -> Self {
        Error::Config(format!("{}", error))
    }
}
