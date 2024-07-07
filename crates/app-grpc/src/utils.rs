use backend_core::error::ProblemDetailDef;
use itertools::Itertools;
use prost_types::{value, ListValue};
use std::collections::HashSet;
use tonic::{Code, Status};

pub(crate) fn map_err(value: backend_core::Error) -> Status {
  let value: ProblemDetailDef = value.into();
  let code = match value.status {
    401 => Code::Unauthenticated,
    404 => Code::NotFound,
    _ => Code::Unknown,
  };
  let details = serde_json::to_string(&value).unwrap_or_default();
  Status::with_details(code, value.detail, details.into())
}

pub(crate) fn decode_strings(value: ListValue) -> HashSet<String> {
  value
    .values
    .into_iter()
    .filter_map(|v| if let Some(value::Kind::StringValue(v)) = v.kind { Some(v) } else { None })
    .collect()
}

pub(crate) fn decode_uuid(value: impl ToString) -> Result<uuid::Uuid, Status> {
  value.to_string().parse().map_err(|_e| Status::new(Code::Internal, "Invalid UUID"))
}

pub(crate) fn decode_uuids(
  values: impl IntoIterator<Item = impl ToString>,
) -> Result<HashSet<uuid::Uuid>, Status> {
  values.into_iter().map(decode_uuid).try_collect()
}
