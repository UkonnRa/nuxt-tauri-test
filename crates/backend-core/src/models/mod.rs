use std::{collections::HashSet, hash::Hash};

use chrono::{DateTime, Utc};
use sea_orm::{sea_query::IntoCondition, ConnectionTrait};
use serde::{Deserialize, Serialize};

pub mod journal;

#[async_trait::async_trait]
pub trait ReadAggregate: Sized + Hash + Default {
  type Id: ToString;

  type Query: Send;

  fn id(&self) -> Self::Id;

  async fn find_all(
    db: &impl ConnectionTrait,
    query: Option<Self::Query>,
    limit: Option<u64>,
  ) -> crate::Result<HashSet<Self>>;

  async fn find_one(
    db: &impl ConnectionTrait,
    query: Option<Self::Query>,
  ) -> crate::Result<Option<Self>> {
    Ok(Self::find_all(db, query, Some(1)).await?.into_iter().next())
  }

  fn validate(&self) -> crate::Result<()> {
    Ok(())
  }
}

#[async_trait::async_trait]
pub trait WriteAggregate: ReadAggregate<Query: IntoCondition> {
  type Model;

  fn created_date(&self) -> DateTime<Utc>;

  fn version(&self) -> usize;

  async fn from_models(
    db: &impl ConnectionTrait,
    models: impl IntoIterator<Item = Self::Model> + Send,
  ) -> crate::Result<HashSet<Self>>;

  async fn save(
    db: &impl ConnectionTrait,
    aggregates: impl IntoIterator<Item = Self> + Send,
  ) -> crate::Result<HashSet<Self::Id>>;

  async fn delete(
    db: &impl ConnectionTrait,
    ids: impl IntoIterator<Item = Self::Id> + Send,
  ) -> crate::Result<()>;
}

#[async_trait::async_trait]
pub trait Presentation: Sized + Serialize + for<'a> Deserialize<'a> {
  type A: ReadAggregate;

  async fn from_aggregates(
    db: &impl ConnectionTrait,
    aggregates: impl IntoIterator<Item = Self::A> + Send,
  ) -> crate::Result<HashSet<Self>>;
}

#[async_trait::async_trait]
pub trait CommandHandler: WriteAggregate {
  type Command: Send + Sync;

  async fn handle(
    db: &impl ConnectionTrait,
    command: Self::Command,
  ) -> crate::Result<HashSet<Self::Id>>;
}
