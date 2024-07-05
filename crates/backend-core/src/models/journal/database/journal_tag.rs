use super::journal;
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, Hash, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "journal_tags")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub journal_id: Uuid,
  #[sea_orm(primary_key)]
  pub tag: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "journal::Entity",
    from = "Column::JournalId",
    to = "journal::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
  )]
  Journal,
}

impl Related<journal::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Journal.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
