mod problem_detail;

pub use problem_detail::*;

use http::StatusCode;
use itertools::Itertools;
use sea_orm::DbErr;
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum Error {
  #[error("{}", .0.detail())]
  NotFound(ErrorNotFound),

  #[error("{}", .0.detail())]
  ExistingEntity(ErrorExistingEntity),

  #[error("{}", .0.detail())]
  OutOfRange(ErrorOutOfRange),

  #[error("{}", .0.detail())]
  RequiredField(ErrorRequiredField),

  #[error("{}", .0.detail())]
  Internal(ErrorInternal),
}

impl From<Error> for ProblemDetailDef {
  fn from(value: Error) -> Self {
    match value {
      Error::NotFound(err) => ProblemDetailDef::from(err.clone()),
      Error::ExistingEntity(err) => ProblemDetailDef::from(err.clone()),
      Error::OutOfRange(err) => ProblemDetailDef::from(err.clone()),
      Error::RequiredField(err) => ProblemDetailDef::from(err.clone()),
      Error::Internal(err) => ProblemDetailDef::from(err.clone()),
    }
  }
}

impl Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    ProblemDetailDef::from(self.clone()).serialize(serializer)
  }
}

impl<'a> Deserialize<'a> for Error {
  fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
  where
    D: Deserializer<'a>,
  {
    let def = ProblemDetailDef::deserialize(deserializer)?;

    if def.typ == ErrorNotFound::typ() {
      Ok(Error::NotFound(serde_json::from_value(def.extra).unwrap()))
    } else if def.typ == ErrorExistingEntity::typ() {
      Ok(Error::ExistingEntity(serde_json::from_value(def.extra).unwrap()))
    } else if def.typ == ErrorOutOfRange::typ() {
      Ok(Error::OutOfRange(serde_json::from_value(def.extra).unwrap()))
    } else if def.typ == ErrorRequiredField::typ() {
      Ok(Error::RequiredField(serde_json::from_value(def.extra).unwrap()))
    } else if def.typ == ErrorInternal::typ() {
      Ok(Error::Internal(serde_json::from_value(def.extra).unwrap()))
    } else {
      Err(SerdeError::custom(format!("Invalid Error type: {}", def.typ)))
    }
  }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorNotFound {
  pub entity: String,
  pub values: Vec<(String, String)>,
}

impl ProblemDetail for ErrorNotFound {
  fn typ() -> &'static str {
    "urn:white-rabbit:error:not-found"
  }

  fn title() -> &'static str {
    "Entity Not Found"
  }

  fn status() -> StatusCode {
    StatusCode::NOT_FOUND
  }

  fn detail(&self) -> String {
    format!(
      "Entity[{}, {}] not found",
      self.entity,
      self.values.iter().map(|(f, v)| format!("{} = {}", f, v)).join(", ")
    )
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorExistingEntity {
  pub entity: String,
  pub values: Vec<(String, String)>,
}

impl ProblemDetail for ErrorExistingEntity {
  fn typ() -> &'static str {
    "urn:white-rabbit:error:existing-entity"
  }

  fn title() -> &'static str {
    "Entity Already Exists"
  }

  fn status() -> StatusCode {
    StatusCode::BAD_REQUEST
  }

  fn detail(&self) -> String {
    format!(
      "Entity[{}, {}] already exists",
      self.entity,
      self.values.iter().map(|(f, v)| format!("{} = {}", f, v)).join(", ")
    )
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorOutOfRange {
  pub entity: String,
  pub field: String,
  pub start: Option<String>,
  pub end: Option<String>,
}

impl ProblemDetail for ErrorOutOfRange {
  fn typ() -> &'static str {
    "urn:white-rabbit:error:out-of-range"
  }

  fn title() -> &'static str {
    "Value Out of Range"
  }

  fn status() -> StatusCode {
    StatusCode::BAD_REQUEST
  }

  fn detail(&self) -> String {
    format!(
      "Field[{}] of Entity[{}] should in Range[start = {:?}, end = {:?}]",
      self.field, self.entity, self.start, self.end,
    )
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ErrorRequiredField {
  pub entity: String,
  pub field: String,
}

impl ProblemDetail for ErrorRequiredField {
  fn typ() -> &'static str {
    "urn:white-rabbit:error:required-field"
  }

  fn title() -> &'static str {
    "Required Field"
  }

  fn status() -> StatusCode {
    StatusCode::BAD_REQUEST
  }

  fn detail(&self) -> String {
    format!("Field[{}] of Entity[{}] is required", self.field, self.entity,)
  }
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
