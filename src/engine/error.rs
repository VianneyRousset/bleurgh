use boa_engine::JsError;
use std::convert::From;
use std::string::FromUtf16Error;

#[derive(Debug)]
pub enum EngineError {
    JsError(JsError),
    FromUtf16Error(FromUtf16Error),
}

impl From<JsError> for EngineError {
    fn from(error: JsError) -> Self {
        Self::JsError(error)
    }
}

impl From<FromUtf16Error> for EngineError {
    fn from(error: FromUtf16Error) -> Self {
        Self::FromUtf16Error(error)
    }
}
