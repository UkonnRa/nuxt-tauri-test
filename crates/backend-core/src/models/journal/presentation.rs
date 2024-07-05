use std::collections::HashSet;

use sea_orm::ConnectionTrait;

use crate::models::{journal, Presentation};

#[async_trait::async_trait]
impl Presentation for journal::Aggregate {
  type A = journal::Aggregate;

  async fn from_aggregates(
    db: &impl ConnectionTrait,
    aggregates: impl IntoIterator<Item = Self::A> + Send,
  ) -> crate::Result<HashSet<Self>> {
    Ok(aggregates.into_iter().collect())
  }
}