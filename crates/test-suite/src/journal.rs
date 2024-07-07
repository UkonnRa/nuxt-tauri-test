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

pub async fn test_update_journal(ctx: TestContext) -> anyhow::Result<()> {
  // setup
  let command = journal::CommandCreate {
    name: "New Name".to_string(),
    description: "New Desc".to_string(),
    unit: "New Unit".to_string(),
    tags: HashSet::from_iter(["Tag 1".to_string(), "Tag 2".to_string()]),
  };

  let created_id: Uuid = ctx
    .journal_client
    .handle_command(journal::Command::Create(command.clone()))
    .await?
    .into_iter()
    .next()
    .unwrap()
    .parse()
    .unwrap();

  let update_command = journal::CommandUpdate {
    id: created_id,
    name: "Updated Name".to_string(),
    description: None,
    unit: Default::default(),
    tags: Some(HashSet::from_iter(["Tag 4".to_string(), "Tag 6".to_string()])),
  };

  ctx.journal_client.handle_command(journal::Command::Update(update_command.clone())).await?;
  let result = ctx
    .journal_client
    .find_one(Some(journal::Query { id: HashSet::from_iter([created_id]), ..Default::default() }))
    .await?
    .unwrap();

  assert_eq!(result.id, created_id);
  assert_eq!(result.name, update_command.name);
  assert_eq!(result.description, command.description);
  assert_eq!(result.unit, command.unit);
  assert_eq!(result.tags, update_command.tags.unwrap());

  Ok(())
}
