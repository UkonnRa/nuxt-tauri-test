use backend_core::models::journal;
use chrono::DateTime;
use futures::lock::Mutex;
use pb::{journal_service_client::JournalServiceClient, FindAllRequest, HandleCommandRequest};
use prost_types::Timestamp;
use sea_orm::DatabaseConnection;
use std::{collections::HashSet, sync::Arc};
use test_suite::{ReadClient, WriteClient};
use tonic::transport::Channel;

use super::{encode_strings, map_err};

pub(crate) mod pb {
  tonic::include_proto!("whiterabbit.journal.v1");
}

impl From<pb::Journal> for journal::Aggregate {
  fn from(value: pb::Journal) -> Self {
    journal::Aggregate {
      id: value.id.parse().unwrap(),
      created_date: value
        .created_date
        .and_then(|Timestamp { seconds, nanos }| DateTime::from_timestamp(seconds, nanos as u32))
        .unwrap_or_default(),
      version: value.version as usize,
      name: value.name,
      description: value.description,
      unit: value.unit,
      tags: HashSet::from_iter(value.tags),
    }
  }
}

impl From<journal::Query> for pb::JournalQuery {
  fn from(value: journal::Query) -> Self {
    Self {
      id: value.id.into_iter().map(|id| id.to_string()).collect(),
      name: Vec::from_iter(value.name),
      unit: value.unit,
      tags: Vec::from_iter(value.tags),
      full_text: value.full_text,
    }
  }
}

impl From<journal::CommandCreate> for pb::JournalCommandCreate {
  fn from(
    journal::CommandCreate { name, description, unit, tags }: journal::CommandCreate,
  ) -> Self {
    Self { name, description, unit, tags: Vec::from_iter(tags) }
  }
}

impl From<journal::CommandUpdate> for pb::JournalCommandUpdate {
  fn from(
    journal::CommandUpdate { id, name, description, unit, tags }: journal::CommandUpdate,
  ) -> Self {
    Self { id: id.to_string(), name, description, unit, tags: tags.map(encode_strings) }
  }
}

impl From<journal::Command> for pb::journal_command::Command {
  fn from(value: journal::Command) -> Self {
    match value {
      journal::Command::Create(command) => pb::journal_command::Command::Create(command.into()),
      journal::Command::Update(command) => pb::journal_command::Command::Update(command.into()),
      journal::Command::Delete(journal::CommandDelete { id }) => {
        pb::journal_command::Command::Delete(pb::JournalCommandDelete {
          id: id.into_iter().map(|v| v.to_string()).collect(),
        })
      }
      journal::Command::Batch(journal::CommandBatch { create, update, delete }) => {
        pb::journal_command::Command::Batch(pb::JournalCommandBatch {
          create: create.into_iter().map(|c| c.into()).collect(),
          update: update.into_iter().map(|c| c.into()).collect(),
          delete: delete.into_iter().map(|v| v.to_string()).collect(),
        })
      }
    }
  }
}

pub struct JournalClientImpl {
  _db: Arc<DatabaseConnection>,
  grpc_client: Mutex<JournalServiceClient<Channel>>,
}

impl JournalClientImpl {
  pub async fn create(db: Arc<DatabaseConnection>, api_url: impl ToString) -> anyhow::Result<Self> {
    Ok(Self {
      _db: db,
      grpc_client: Mutex::new(
        super::backoff_connect(|| {
          JournalServiceClient::connect(format!("http://{}", api_url.to_string()))
        })
        .await?,
      ),
    })
  }
}

#[async_trait::async_trait]
impl ReadClient for JournalClientImpl {
  type Presentation = journal::Aggregate;

  type Query = journal::Query;

  async fn find_all(
    &self,
    query: Option<Self::Query>,
    _limit: Option<u64>,
  ) -> test_suite::Result<HashSet<Self::Presentation>> {
    match self
      .grpc_client
      .lock()
      .await
      .find_all(FindAllRequest { query: query.map(|q| q.into()) })
      .await
    {
      Ok(resp) => Ok(resp.into_inner().values.into_iter().map(|proto| proto.into()).collect()),
      Err(status) => Err(map_err(status)),
    }
  }

  async fn find_one(
    &self,
    query: Option<Self::Query>,
  ) -> test_suite::Result<Option<Self::Presentation>> {
    Ok(self.find_all(query, None).await?.into_iter().next())
  }
}

#[async_trait::async_trait]
impl WriteClient for JournalClientImpl {
  type Command = journal::Command;

  async fn handle_command(&self, command: Self::Command) -> test_suite::Result<HashSet<String>> {
    match self
      .grpc_client
      .lock()
      .await
      .handle_command(HandleCommandRequest {
        command: Some(pb::JournalCommand { command: Some(command.into()) }),
      })
      .await
    {
      Ok(resp) => Ok(HashSet::from_iter(resp.into_inner().values)),
      Err(status) => Err(map_err(status)),
    }
  }
}
