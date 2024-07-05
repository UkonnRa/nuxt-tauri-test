use http::StatusCode;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};

pub trait ProblemDetail {
  fn typ() -> &'static str;

  fn title() -> &'static str;

  fn status() -> StatusCode;

  fn detail(&self) -> String;
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum Error {
  #[error("{}", .0.detail())]
  Internal(ErrorInternal),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorInternal {
  pub message: String,
}

impl ProblemDetail for ErrorInternal {
  fn typ() -> &'static str {
    "urn:white-rabbit:error:internal"
  }

  fn title() -> &'static str {
    "Internal Error"
  }

  fn status() -> StatusCode {
    StatusCode::BAD_REQUEST
  }

  fn detail(&self) -> String {
    format!("Internal Error: {}", self.message)
  }
}

impl<E> From<E> for ErrorInternal
where
  E: std::error::Error,
{
  fn from(err: E) -> Self {
    Self { message: err.to_string() }
  }
}

impl From<ErrorInternal> for Error {
  fn from(value: ErrorInternal) -> Self {
    Error::Internal(value)
  }
}

impl From<DbErr> for Error {
  fn from(value: DbErr) -> Self {
    ErrorInternal::from(value).into()
  }
}

pub type Result<T> = std::result::Result<T, Error>;
