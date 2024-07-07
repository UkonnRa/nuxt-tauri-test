use std::fmt::{Display, Formatter};

use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct ProblemDetailDef {
  #[serde(rename = "type")]
  pub typ: String,
  pub title: String,
  pub status: u16,
  pub detail: String,
  #[serde(flatten)]
  pub extra: Value,
}

impl Display for ProblemDetailDef {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.detail)
  }
}

impl std::error::Error for ProblemDetailDef {}

pub trait ProblemDetail:
  Into<ProblemDetailDef> + Serialize + for<'a> Deserialize<'a> + Sync + Send
{
  fn typ() -> &'static str;

  fn title() -> &'static str;

  fn status() -> StatusCode;

  fn detail(&self) -> String;
}

impl<E> From<E> for ProblemDetailDef
where
  E: ProblemDetail,
{
  fn from(value: E) -> Self {
    ProblemDetailDef {
      typ: E::typ().to_string(),
      title: E::title().to_string(),
      status: E::status().as_u16(),
      detail: value.detail().to_string(),
      extra: serde_json::to_value(value).unwrap(),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::error::{
    ErrorExistingEntity, ErrorInternal, ErrorNotFound, ErrorOutOfRange, ErrorRequiredField,
  };
  use crate::models::{journal, FIELD_ID, FIELD_NAME, MIN_NAME_LENGTH};

  #[test]
  fn test_serde() -> anyhow::Result<()> {
    let errors = vec![
      crate::Error::NotFound(ErrorNotFound {
        entity: journal::TYPE.to_string(),
        values: vec![
          (FIELD_ID.to_string(), "ID1".to_string()),
          (FIELD_NAME.to_string(), "Journal 1".to_string()),
        ],
      }),
      crate::Error::ExistingEntity(ErrorExistingEntity {
        entity: journal::TYPE.to_string(),
        values: vec![
          (FIELD_ID.to_string(), "ID2".to_string()),
          (FIELD_NAME.to_string(), "Journal 2".to_string()),
        ],
      }),
      crate::Error::OutOfRange(ErrorOutOfRange {
        entity: journal::TYPE.to_string(),
        field: FIELD_NAME.to_string(),
        start: Some(MIN_NAME_LENGTH.to_string()),
        end: None,
      }),
      crate::Error::RequiredField(ErrorRequiredField {
        entity: journal::TYPE.to_string(),
        field: FIELD_NAME.to_string(),
      }),
      crate::Error::Internal(ErrorInternal { message: "Invalid DB Connection".to_string() }),
    ];

    for err in errors {
      let serded = serde_json::to_string_pretty(&err)?;
      println!("Serded: {}", serded);

      let deserded: crate::Error = serde_json::from_str(&serded)?;
      println!("Desered Err: {:#?}", deserded);

      assert_eq!(err, deserded);
    }

    Ok(())
  }
}
