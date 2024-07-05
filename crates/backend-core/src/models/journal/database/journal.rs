use super::journal_tag;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "journals")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub created_date: chrono::DateTime<chrono::Utc>,
  pub version: u64,
  #[sea_orm(unique, indexed)]
  pub name: String,
  pub description: String,
  #[sea_orm(indexed)]
  pub unit: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(has_many = "journal_tag::Entity")]
  Tags,
}

impl Related<journal_tag::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Tags.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
