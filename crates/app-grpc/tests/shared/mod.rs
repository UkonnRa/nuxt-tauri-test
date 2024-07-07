mod journal;

use backend_core::error::ProblemDetailDef;
use futures::Future;
use migration::{Migrator, MigratorTrait};
use prost_types::{value, ListValue, Value};
use std::{net::TcpListener, sync::Arc, time::Duration};
use test_suite::{journal::JournalClient, TestContext};
use tokio::time;
use tonic::transport::Error as TonicError;
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

pub(crate) async fn backoff_connect<F, C, R>(connect_fn: F) -> Result<C, TonicError>
where
  F: Fn() -> R,
  R: Future<Output = Result<C, TonicError>>,
{
  let mut count = 0;
  loop {
    count += 1;
    match connect_fn().await {
      Ok(result) => return Ok(result),
      Err(_) if count < 5 => {
        time::sleep(Duration::from_millis(100 * (2 << count))).await;
      }
      Err(error) => return Err(error),
    }
  }
}

pub async fn init() -> anyhow::Result<TestContext> {
  let api_url = {
    let listener = TcpListener::bind("[::1]:0").unwrap();
    let port: u16 = listener.local_addr().unwrap().port();
    format!("[::1]:{port}")
  };

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
