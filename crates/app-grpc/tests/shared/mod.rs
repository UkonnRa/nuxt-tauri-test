mod journal;

use backend_core::error::ProblemDetailDef;
use migration::{Migrator, MigratorTrait};
use prost_types::{value, ListValue, Value};
use rand::RngCore;
use std::sync::Arc;
use test_suite::{journal::JournalClient, TestContext};
use tonic::Status;

pub(crate) fn map_err(status: Status) -> ProblemDetailDef {
  serde_json::from_slice(status.details()).unwrap()
}

pub(crate) fn encode_strings(value: impl IntoIterator<Item = impl ToString>) -> ListValue {
  ListValue {
    values: value
      .into_iter()
      .map(|s| Value { kind: Some(value::Kind::StringValue(s.to_string())) })
      .collect(),
  }
}

pub async fn init() -> anyhow::Result<TestContext> {
  let port: u32 = (rand::thread_rng().next_u32() % 1_0000) + 5_0000;
  let api_url = format!("[::1]:{port}");

  let db = Arc::new(backend_core::init(".test.env").await?);
  Migrator::up(db.as_ref(), None).await?;

  let db_ref = db.clone();
  let api_url_ref = api_url.clone();
  tokio::spawn(async move {
    app_grpc::serve(db_ref, api_url_ref).await.unwrap();
  });

  let journal_client: Arc<Box<dyn JournalClient>> =
    Arc::new(Box::new(journal::JournalClientImpl::create(db.clone(), api_url).await?));
  Ok(TestContext { db, journal_client })
}
