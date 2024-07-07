use crate::{TestContext, WriteClient};
use backend_core::{
  error::{ErrorExistingEntity, ProblemDetail, ProblemDetailDef},
  models::{journal, FIELD_NAME},
};
use serde_json::json;
use std::collections::HashSet;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait JournalClient:
  WriteClient<Presentation = journal::Aggregate, Query = journal::Query, Command = journal::Command>
{
}

impl<T> JournalClient for T where
  T: WriteClient<
    Presentation = journal::Aggregate,
    Query = journal::Query,
    Command = journal::Command,
  >
{
}

pub async fn test_create_journal(ctx: TestContext) -> anyhow::Result<()> {
  let command = journal::CommandCreate {
    name: "New Name".to_string(),
    description: "New Desc".to_string(),
    unit: "New Unit".to_string(),
    tags: HashSet::from_iter(["Tag 1".to_string(), "Tag 2".to_string()]),
  };
  let ids = ctx.journal_client.handle_command(journal::Command::Create(command.clone())).await?;
  let ids = ids.into_iter().filter_map(|id| id.parse::<Uuid>().ok()).collect::<HashSet<_>>();
  let result = ctx
    .journal_client
    .find_all(Some(journal::Query { id: ids, ..Default::default() }), None)
    .await?;

  assert_eq!(result.len(), 1);
  if let Some(journal::Aggregate { name, description, unit, tags, .. }) = result.iter().next() {
    assert_eq!(name, &command.name);
    assert_eq!(description, &command.description);
    assert_eq!(unit, &command.unit);
    assert_eq!(tags, &command.tags);
  }

  Ok(())
}

pub async fn test_create_journal_conflict_name(ctx: TestContext) -> anyhow::Result<()> {
  // setup
  let command = journal::CommandCreate {
    name: "New Name".to_string(),
    description: "New Desc".to_string(),
    unit: "New Unit".to_string(),
    tags: HashSet::from_iter(["Tag 1".to_string(), "Tag 2".to_string()]),
  };

  ctx.journal_client.handle_command(journal::Command::Create(command.clone())).await?;

  if let Err(ProblemDetailDef { typ, title, status, extra, .. }) =
    ctx.journal_client.handle_command(journal::Command::Create(command.clone())).await
  {
    assert_eq!(typ, ErrorExistingEntity::typ());
    assert_eq!(title, ErrorExistingEntity::title());
    assert_eq!(status, ErrorExistingEntity::status());
    assert_eq!(typ, ErrorExistingEntity::typ());
    assert_eq!(
      extra,
      json!({
        "entity": journal::TYPE,
        "values": [[FIELD_NAME, command.name]],
      })
    );
  } else {
    panic!("Error[{}] should be thrown", ErrorExistingEntity::typ());
  }

  Ok(())
}
