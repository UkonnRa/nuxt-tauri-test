pub mod journal;

use backend_core::error::ProblemDetailDef;
use sea_orm::DatabaseConnection;
use std::{collections::HashSet, sync::Arc};

pub type Result<T> = std::result::Result<T, ProblemDetailDef>;

#[async_trait::async_trait]
pub trait ReadClient {
  type Presentation;

  type Query;

  async fn find_all(
    &self,
    query: Option<Self::Query>,
    limit: Option<u64>,
  ) -> crate::Result<HashSet<Self::Presentation>>;

  async fn find_one(&self, query: Option<Self::Query>)
    -> crate::Result<Option<Self::Presentation>>;
}

#[async_trait::async_trait]
pub trait WriteClient: ReadClient {
  type Command;

  async fn handle_command(&self, command: Self::Command) -> crate::Result<HashSet<String>>;
}

pub struct TestContext {
  pub db: Arc<DatabaseConnection>,
  pub journal_client: Arc<Box<dyn journal::JournalClient>>,
}
