use crate::utils::{decode_strings, decode_uuid, decode_uuids, map_err};
use backend_core::models::{journal, CommandHandler, ReadAggregate};
use itertools::Itertools;
use pb::journal_service_server::{JournalService, JournalServiceServer};
use pb::{
  journal_command, FindAllRequest, FindAllResponse, FindByIdRequest, FindByIdResponse,
  HandleCommandRequest, HandleCommandResponse, Journal, JournalCommand, JournalCommandBatch,
  JournalCommandCreate, JournalCommandDelete, JournalCommandUpdate, JournalQuery,
};
use sea_orm::DatabaseConnection;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::SystemTime;
use tonic::codec::CompressionEncoding;
use tonic::service::Routes;
use tonic::{Code, Request, Response, Status};
use tonic_reflection::server::Builder;
use uuid::Uuid;

pub(crate) mod pb {
  tonic::include_proto!("whiterabbit.journal.v1");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("journal_descriptor");
}

impl prost::Name for Journal {
  const PACKAGE: &'static str = "whiterabbit.journal.v1";
  const NAME: &'static str = "Journal";
}

// Model -> Proto

impl From<HashSet<journal::Aggregate>> for FindAllResponse {
  fn from(results: HashSet<journal::Aggregate>) -> Self {
    Self { values: results.into_iter().map(|model| model.into()).collect() }
  }
}

impl From<journal::Aggregate> for Journal {
  fn from(value: journal::Aggregate) -> Self {
    Self {
      id: value.id.to_string(),
      created_date: Some(SystemTime::from(value.created_date).into()),
      version: value.version as i64,
      name: value.name,
      description: value.description,
      unit: value.unit,
      tags: Vec::from_iter(value.tags),
    }
  }
}

impl From<journal::Aggregate> for FindByIdResponse {
  fn from(value: journal::Aggregate) -> Self {
    Self { value: Some(value.into()) }
  }
}

// Proto -> Model

impl TryFrom<JournalQuery> for journal::Query {
  type Error = Status;

  fn try_from(value: JournalQuery) -> Result<Self, Self::Error> {
    println!("Journal Query Proto: {value:?}");
    Ok(Self {
      id: decode_uuids(value.id)?,
      name: HashSet::from_iter(value.name),
      unit: value.unit,
      tags: value.tags.iter().cloned().collect(),
      full_text: value.full_text,
    })
  }
}

impl From<JournalCommandCreate> for journal::CommandCreate {
  fn from(value: JournalCommandCreate) -> Self {
    journal::CommandCreate {
      name: value.name,
      description: value.description,
      unit: value.unit,
      tags: HashSet::from_iter(value.tags),
    }
  }
}

impl TryFrom<JournalCommandUpdate> for journal::CommandUpdate {
  type Error = Status;

  fn try_from(value: JournalCommandUpdate) -> Result<Self, Self::Error> {
    Ok(journal::CommandUpdate {
      id: decode_uuid(value.id)?,
      name: value.name,
      description: value.description,
      unit: value.unit,
      tags: value.tags.map(decode_strings),
    })
  }
}

impl TryFrom<JournalCommandDelete> for journal::CommandDelete {
  type Error = Status;

  fn try_from(value: JournalCommandDelete) -> Result<Self, Self::Error> {
    Ok(journal::CommandDelete { id: decode_uuids(value.id)? })
  }
}

impl TryFrom<JournalCommandBatch> for journal::CommandBatch {
  type Error = Status;

  fn try_from(value: JournalCommandBatch) -> Result<Self, Self::Error> {
    Ok(journal::CommandBatch {
      create: value.create.into_iter().map(|c| c.into()).collect(),
      update: value.update.into_iter().map(|c| c.try_into()).try_collect()?,
      delete: decode_uuids(value.delete)?,
    })
  }
}

impl TryFrom<journal_command::Command> for journal::Command {
  type Error = Status;

  fn try_from(value: journal_command::Command) -> Result<Self, Self::Error> {
    Ok(match value {
      journal_command::Command::Create(command) => journal::Command::Create(command.into()),
      journal_command::Command::Update(command) => journal::Command::Update(command.try_into()?),
      journal_command::Command::Delete(command) => journal::Command::Delete(command.try_into()?),
      journal_command::Command::Batch(command) => journal::Command::Batch(command.try_into()?),
    })
  }
}

#[derive(Debug)]
pub struct JournalServiceImpl {
  pub db: Arc<DatabaseConnection>,
}

#[tonic::async_trait]
impl JournalService for JournalServiceImpl {
  async fn find_all(
    &self,
    request: Request<FindAllRequest>,
  ) -> Result<Response<FindAllResponse>, Status> {
    let query = request.get_ref().query.clone().unwrap_or_default();
    let results = journal::Aggregate::find_all(self.db.as_ref(), Some(query.try_into()?), None)
      .await
      .map_err(map_err)?;

    Ok(Response::new(results.into()))
  }

  async fn find_by_id(
    &self,
    request: Request<FindByIdRequest>,
  ) -> Result<Response<FindByIdResponse>, Status> {
    let id: Uuid =
      request.get_ref().id.parse().map_err(|_| Status::new(Code::Internal, "Invalid UUID"))?;

    let model = journal::Aggregate::find_one(
      self.db.as_ref(),
      Some(journal::Query { id: HashSet::from_iter([id]), ..Default::default() }),
    )
    .await
    .map_err(map_err)?
    .map(|model| model.into());

    Ok(Response::new(FindByIdResponse { value: model }))
  }

  async fn handle_command(
    &self,
    request: Request<HandleCommandRequest>,
  ) -> Result<Response<HandleCommandResponse>, Status> {
    log::info!("Journal Command: {:#?}", request);
    if let Some(JournalCommand { command: Some(command) }) = request.get_ref().command.clone() {
      let command: journal::Command = command.try_into()?;
      let values = journal::Aggregate::handle_command(self.db.as_ref(), command)
        .await
        .map_err(map_err)?
        .into_iter()
        .map(|model| model.into())
        .collect();
      Ok(Response::new(HandleCommandResponse { values }))
    } else {
      Err(Status::invalid_argument("Command field should not be empty"))
    }
  }
}

pub(crate) fn init(
  reflection_builder: Builder,
  routes: Routes,
  db: Arc<DatabaseConnection>,
) -> (Builder, Routes) {
  let service = JournalServiceServer::new(JournalServiceImpl { db })
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

  (
    reflection_builder.register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET),
    routes.add_service(service),
  )
}
