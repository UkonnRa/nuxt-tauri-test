use backend_core::models::{journal, CommandHandler, ReadAggregate};
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use std::{collections::HashSet, sync::Arc};
use test_suite::{journal::JournalClient, ReadClient, TestContext, WriteClient};

pub struct JournalClientImpl {
  db: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
impl ReadClient for JournalClientImpl {
  type Presentation = journal::Aggregate;

  type Query = journal::Query;

  async fn find_all(
    &self,
    query: Option<Self::Query>,
    limit: Option<u64>,
  ) -> test_suite::Result<HashSet<Self::Presentation>> {
    Ok(journal::Aggregate::find_all(self.db.as_ref(), query, limit).await?)
  }

  async fn find_one(
    &self,
    query: Option<Self::Query>,
  ) -> test_suite::Result<Option<Self::Presentation>> {
    Ok(journal::Aggregate::find_one(self.db.as_ref(), query).await?)
  }
}

#[async_trait::async_trait]
impl WriteClient for JournalClientImpl {
  type Command = journal::Command;

  async fn handle_command(&self, command: Self::Command) -> test_suite::Result<HashSet<String>> {
    Ok(
      journal::Aggregate::handle_command(self.db.as_ref(), command)
        .await?
        .into_iter()
        .map(|id| id.to_string())
        .collect(),
    )
  }
}

pub async fn init() -> anyhow::Result<TestContext> {
  let db = Arc::new(backend_core::init(".test.env").await?);
  Migrator::up(db.as_ref(), None).await?;

  let journal_client: Arc<Box<dyn JournalClient>> =
    Arc::new(Box::new(JournalClientImpl { db: db.clone() }));
  Ok(TestContext { db, journal_client })
}
