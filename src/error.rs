use std::convert::From;
use std::option::NoneError;
use std::num::ParseFloatError;
use reqwest;
use json;

#[derive(Debug)]
pub enum Error {
    IO(reqwest::Error),
    Parse(json::Error),
    Api(String),
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::Api("json incomplete".to_string())
    }
}

impl From<ParseFloatError> for Error {
    fn from(_: ParseFloatError) -> Self {
        Error::Api("float parse error".to_string())
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
