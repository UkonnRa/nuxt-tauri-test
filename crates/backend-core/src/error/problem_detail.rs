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

pub trait ProblemDetail: Into<ProblemDetailDef> + Serialize + Sync + Send {
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

  use serde_json::json;

  use crate::error::{
    ErrorExistingEntity, ErrorInternal, ErrorNotFound, ErrorOutOfRange, ErrorRequiredField,
  };
  use crate::models::{journal, FIELD_ID, FIELD_NAME, MIN_NAME_LENGTH};

  #[test]
  fn test_serde() -> anyhow::Result<()> {
    let errors = vec![
      (
        crate::Error::NotFound(ErrorNotFound {
          entity: journal::TYPE.to_string(),
          values: vec![
            (FIELD_ID.to_string(), "ID1".to_string()),
            (FIELD_NAME.to_string(), "Journal 1".to_string()),
          ],
        }),
        json!({
          "type": "urn:white-rabbit:error:not-found",
          "title": "Entity Not Found",
          "status": 404,
          "detail": "Entity[Journal, id = ID1, name = Journal 1] not found",
          "entity": "Journal",
          "values": [
            ["id", "ID1"],
            ["name", "Journal 1"],
          ]
        }),
      ),
      (
        crate::Error::ExistingEntity(ErrorExistingEntity {
          entity: journal::TYPE.to_string(),
          values: vec![
            (FIELD_ID.to_string(), "ID2".to_string()),
            (FIELD_NAME.to_string(), "Journal 2".to_string()),
          ],
        }),
        json!({
          "type": "urn:white-rabbit:error:existing-entity",
          "title": "Entity Already Exists",
          "status": 400,
          "detail": "Entity[Journal, id = ID2, name = Journal 2] already exists",
          "entity": "Journal",
          "values": [
            ["id", "ID2"],
            ["name", "Journal 2"],
          ]
        }),
      ),
      (
        crate::Error::OutOfRange(ErrorOutOfRange {
          entity: journal::TYPE.to_string(),
          field: FIELD_NAME.to_string(),
          start: Some(MIN_NAME_LENGTH.to_string()),
          end: None,
        }),
        json!({
          "type": "urn:white-rabbit:error:out-of-range",
          "title": "Value Out of Range",
          "status": 400,
          "detail": "Field[name] of Entity[Journal] should in Range[start = 6, end = ]",
          "entity": "Journal",
          "field": "name",
          "start": "6",
          "end": null,
        }),
      ),
      (
        crate::Error::RequiredField(ErrorRequiredField {
          entity: journal::TYPE.to_string(),
          field: FIELD_NAME.to_string(),
        }),
        json!({
          "type": "urn:white-rabbit:error:required-field",
          "title": "Required Field",
          "status": 400,
          "detail": "Field[name] of Entity[Journal] is required",
          "entity": "Journal",
          "field": "name",
        }),
      ),
      (
        crate::Error::Internal(ErrorInternal { message: "Invalid DB Connection".to_string() }),
        json!({
          "type": "urn:white-rabbit:error:internal",
          "title": "Internal Error",
          "status": 500,
          "detail": "Internal Error: Invalid DB Connection",
          "message": "Invalid DB Connection",
        }),
      ),
    ];

    // let errors = vec![
    //   crate::Error::NotFound(ErrorNotFound {
    //     entity: journal::TYPE.to_string(),
    //     values: vec![
    //       (FIELD_ID.to_string(), "ID1".to_string()),
    //       (FIELD_NAME.to_string(), "Journal 1".to_string()),
    //     ],
    //   }),
    //   crate::Error::ExistingEntity(ErrorExistingEntity {
    //     entity: journal::TYPE.to_string(),
    //     values: vec![
    //       (FIELD_ID.to_string(), "ID2".to_string()),
    //       (FIELD_NAME.to_string(), "Journal 2".to_string()),
    //     ],
    //   }),
    //   crate::Error::OutOfRange(ErrorOutOfRange {
    //     entity: journal::TYPE.to_string(),
    //     field: FIELD_NAME.to_string(),
    //     start: Some(MIN_NAME_LENGTH.to_string()),
    //     end: None,
    //   }),
    //   crate::Error::RequiredField(ErrorRequiredField {
    //     entity: journal::TYPE.to_string(),
    //     field: FIELD_NAME.to_string(),
    //   }),
    //   crate::Error::Internal(ErrorInternal { message: "Invalid DB Connection".to_string() }),
    // ];

    for (err, detail) in errors {
      let serded = serde_json::to_value(&err)?;
      println!("Serded: {}", serded);

      assert_eq!(serded, detail);
    }

    Ok(())
  }
}
