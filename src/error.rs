use std::{fmt, result};

use failure::{Backtrace, Context, Fail};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    ctx: Context<ErrorKind>,
}

impl Error {
    /// Return the kind of this error.
    pub fn kind(&self) -> &ErrorKind {
        self.ctx.get_context()
    }

    pub fn bad_request(errors: Vec<String>) -> Self {
        ErrorKind::BadRequest { errors }.into()
    }

    pub fn unexpected_status(status_code: reqwest::StatusCode) -> Self {
        ErrorKind::UnexpectedStatus { status_code }.into()
    }

    pub fn rate_limit_exceeded() -> Self {
        ErrorKind::RateLimitExceeded.into()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.ctx.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.ctx.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ctx.fmt(f)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Fail)]
pub enum ErrorKind {
    #[fail(display = "bad request: {:#?}", errors)]
    BadRequest { errors: Vec<String> },

    #[fail(
        display = "unexpected status code received from NYT: {:#?}",
        status_code
    )]
    UnexpectedStatus { status_code: reqwest::StatusCode },

    #[fail(display = "rate limit exceeded, please wait to try again")]
    RateLimitExceeded,

    #[fail(display = "serde error: {}", error)]
    SerdeError { error: String },

    #[fail(display = "reqwest error: {}", error)]
    ReqwestError { error: String },
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error::from(Context::new(kind))
    }
}
impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { ctx }
    }
}

/// map from serde errors
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::from(Context::new(ErrorKind::SerdeError {
            error: format!("{}", error),
        }))
    }
}

/// map from reqwest errors
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Error::from(Context::new(ErrorKind::ReqwestError {
            error: error.to_string(),
        }))
    }
}
