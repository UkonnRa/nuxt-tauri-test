use super::{
  command::{self, CommandCreate},
  database::{
    journal::{self, ActiveModel, Column, Model},
    journal_tag,
  },
  query, Command, Entity, Query,
};
use crate::models::{CommandHandler, ReadAggregate, WriteAggregate};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use sea_orm::{
  sea_query::{BinOper, Expr, OnConflict},
  ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, QueryFilter, QuerySelect,
};
use serde::{Deserialize, Serialize};
use std::{
  collections::{HashMap, HashSet},
  hash::Hash,
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Aggregate {
  pub id: Uuid,
  pub created_date: DateTime<Utc>,
  pub version: usize,
  pub name: String,
  pub description: String,
  pub unit: String,
  pub tags: HashSet<String>,
}

impl Hash for Aggregate {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.id.hash(state);
    self.created_date.hash(state);
    self.version.hash(state);
    self.name.hash(state);
    self.description.hash(state);
    self.unit.hash(state);
    self.tags.iter().sorted().cloned().collect::<Vec<_>>().hash(state);
  }
}

impl Default for Aggregate {
  fn default() -> Self {
    Self {
      id: Uuid::now_v7(),
      created_date: Utc::now(),
      version: 0,
      name: Default::default(),
      description: Default::default(),
      unit: Default::default(),
      tags: Default::default(),
    }
  }
}

#[async_trait::async_trait]
impl ReadAggregate for Aggregate {
  type Id = Uuid;

  type Query = query::Query;

  fn id(&self) -> Uuid {
    self.id
  }

  async fn find_all(
    db: &impl ConnectionTrait,
    query: Option<Self::Query>,
    limit: Option<u64>,
  ) -> crate::Result<HashSet<Self>> {
    let select =
      if let Some(query) = query { Entity::find().filter(query) } else { Entity::find() };
    let models = select.limit(limit).all(db).await?;
    Self::from_models(db, models).await
  }
}

#[async_trait::async_trait]
impl WriteAggregate for Aggregate {
  type Model = journal::Model;

  fn created_date(&self) -> DateTime<Utc> {
    self.created_date
  }

  fn version(&self) -> usize {
    self.version
  }

  async fn from_models(
    db: &impl ConnectionTrait,
    models: impl IntoIterator<Item = Self::Model> + Send,
  ) -> crate::Result<HashSet<Self>> {
    let mut aggregates = Vec::new();
    let mut ids = HashSet::<Uuid>::new();

    for model in models {
      aggregates.push(Aggregate {
        id: model.id,
        created_date: model.created_date,
        version: model.version as usize,
        name: model.name,
        description: model.description,
        unit: model.unit,
        tags: HashSet::default(),
      });
      ids.insert(model.id);
    }

    let tags = journal_tag::Entity::find()
      .filter(journal_tag::Column::JournalId.is_in(ids))
      .all(db)
      .await?
      .into_iter()
      .into_group_map_by(|tag| tag.journal_id)
      .into_iter()
      .map(|(k, v)| (k, v.into_iter().map(|m| m.tag).collect::<HashSet<_>>()))
      .collect::<HashMap<_, _>>();

    Ok(
      aggregates
        .into_iter()
        .map(|root| Self { tags: tags.get(&root.id).cloned().unwrap_or_default(), ..root })
        .collect(),
    )
  }

  async fn save(
    db: &impl ConnectionTrait,
    aggregates: impl IntoIterator<Item = Self> + Send,
  ) -> crate::Result<HashSet<Self::Id>> {
    let aggregates: Vec<Aggregate> = aggregates.into_iter().collect();
    if aggregates.is_empty() {
      return Ok(HashSet::default());
    }

    let mut model_ids = HashSet::new();
    let mut models: Vec<ActiveModel> = vec![];
    let mut tags: Vec<journal_tag::ActiveModel> = vec![];

    for ref aggregate in aggregates {
      model_ids.insert(aggregate.id);
      models.push(
        Model {
          id: aggregate.id,
          created_date: aggregate.created_date,
          version: aggregate.version as u64 + 1,
          name: aggregate.name.to_string(),
          description: aggregate.description.to_string(),
          unit: aggregate.unit.to_string(),
        }
        .into_active_model(),
      );
      for tag in &aggregate.tags {
        tags.push(
          journal_tag::Model { journal_id: aggregate.id, tag: tag.to_string() }.into_active_model(),
        );
      }
    }

    journal_tag::Entity::delete_many()
      .filter(journal_tag::Column::JournalId.is_in(model_ids.clone()))
      .exec(db)
      .await?;

    // Update unique column name to temp value
    Entity::update_many()
      .col_expr(
        Column::Name,
        Expr::col((Entity, Column::Name)).binary(BinOper::Custom("||"), Expr::current_timestamp()),
      )
      .filter(Column::Id.is_in(model_ids.clone()))
      .exec(db)
      .await?;

    let mut on_conflict = OnConflict::column(Column::Id);
    on_conflict.update_columns([Column::Name, Column::Description, Column::Unit]);
    Entity::insert_many(models).on_conflict(on_conflict).exec(db).await?;

    if !tags.is_empty() {
      journal_tag::Entity::insert_many(tags).exec(db).await?;
    }

    Ok(model_ids)
  }

  async fn delete(
    db: &impl ConnectionTrait,
    ids: impl IntoIterator<Item = Self::Id> + Send,
  ) -> crate::Result<()> {
    Entity::delete_many().filter(Column::Id.is_in(ids)).exec(db).await?;
    Ok(())
  }
}

#[async_trait::async_trait]
impl CommandHandler for Aggregate {
  type Command = command::Command;

  async fn handle(
    db: &impl ConnectionTrait,
    command: Self::Command,
  ) -> crate::Result<HashSet<Self::Id>> {
    match command {
      Command::Create(command) => Self::create(db, vec![command]).await,
      _ => todo!(),
    }
  }
}

impl Aggregate {
  async fn create(
    db: &impl ConnectionTrait,
    commands: Vec<CommandCreate>,
  ) -> crate::Result<HashSet<Uuid>> {
    if commands.is_empty() {
      return Ok(HashSet::default());
    }

    let mut existing_names = HashSet::new();
    let mut commands_map = HashMap::new();

    for command in commands {
      existing_names.insert(command.name.clone());
      commands_map.insert(command.name.clone(), command);
    }

    let existings =
      Self::find_all(db, Some(Query { name: existing_names, ..Default::default() }), None).await?;

    // if !existings.is_empty() {
    //   let existing_names = existings.iter().map(|model| model.name.clone()).sorted().join(", ");

    //   return Err(crate::Error::ExistingEntity(ErrorExistingEntity {
    //     entity: TYPE.to_string(),
    //     values: vec![(FIELD_NAME.to_string(), existing_names)],
    //   }));
    // }

    let roots: Vec<_> = commands_map
      .into_values()
      .map(|command| {
        let aggregate = Aggregate {
          name: command.name,
          unit: command.unit,
          description: command.description,
          tags: command.tags,
          ..Default::default()
        };

        aggregate.validate().map(|_| aggregate)
      })
      .try_collect()?;
    Self::save(db, roots).await
  }
}
