use std::io;
use std::io::prelude::*;
use std::convert::From;
use std::option::NoneError;
use std::num::ParseFloatError;
use serde_ini;
use reqwest;
use json;

#[derive(Debug)]
pub enum Error {
    IO(reqwest::Error),
    Parse(json::Error),
    Api(String),
    Config(String),
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::Api("json incomplete".into())
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error::Api("float parse error".into())
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
