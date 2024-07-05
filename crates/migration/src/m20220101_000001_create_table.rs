use backend_core::models::{
  journal::database, MAX_DESCRIPTION_LENGTH, MAX_NAME_LENGTH, MAX_SHORT_TEXT_LENGTH,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

impl Migration {
  async fn create_table_journals(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let table = Table::create()
      .table(database::journal::Entity)
      .col(ColumnDef::new(database::journal::Column::Id).uuid().primary_key().not_null())
      .col(ColumnDef::new(database::journal::Column::CreatedDate).date_time().not_null())
      .col(ColumnDef::new(database::journal::Column::Version).integer().not_null())
      .col(
        ColumnDef::new(database::journal::Column::Name)
          .string_len(MAX_NAME_LENGTH as u32)
          .not_null()
          .unique_key(),
      )
      .col(
        ColumnDef::new(database::journal::Column::Description)
          .string_len(MAX_DESCRIPTION_LENGTH as u32)
          .not_null(),
      )
      .col(
        ColumnDef::new(database::journal::Column::Unit)
          .string_len(MAX_SHORT_TEXT_LENGTH as u32)
          .not_null(),
      )
      .to_owned();
    manager.create_table(table).await?;

    Ok(())
  }

  async fn create_table_journal_tags(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    let table = Table::create()
      .table(database::journal_tag::Entity)
      .col(ColumnDef::new(database::journal_tag::Column::JournalId).uuid().not_null())
      .col(
        ColumnDef::new(database::journal_tag::Column::Tag)
          .string_len(MAX_SHORT_TEXT_LENGTH as u32)
          .not_null(),
      )
      .primary_key(
        Index::create()
          .name("pk-journal_tags")
          .col(database::journal_tag::Column::JournalId)
          .col(database::journal_tag::Column::Tag)
          .primary(),
      )
      .foreign_key(
        ForeignKeyCreateStatement::new()
          .name("fk-journal_tags-journal_id")
          .from_tbl(database::journal_tag::Entity)
          .from_col(database::journal_tag::Column::JournalId)
          .to_tbl(database::journal::Entity)
          .to_col(database::journal::Column::Id)
          .on_delete(ForeignKeyAction::Cascade)
          .on_update(ForeignKeyAction::Cascade),
      )
      .to_owned();
    manager.create_table(table).await?;

    Ok(())
  }

  // async fn create_table_accounts(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
  //   let table = Table::create()
  //     .table(account::Entity)
  //     .col(ColumnDef::new(account::Column::Id).uuid().primary_key().not_null())
  //     .col(ColumnDef::new(account::Column::JournalId).uuid().not_null())
  //     .col(ColumnDef::new(account::Column::Name).string_len(MAX_NAME_LENGTH as u32).not_null())
  //     .col(
  //       ColumnDef::new(account::Column::Description)
  //         .string_len(MAX_DESCRIPTION_LENGTH as u32)
  //         .not_null(),
  //     )
  //     .col(
  //       ColumnDef::new(account::Column::Unit).string_len(MAX_SHORT_TEXT_LENGTH as u32).not_null(),
  //     )
  //     .col(ColumnDef::new(account::Column::Typ).string_len(1).not_null())
  //     .foreign_key(
  //       ForeignKeyCreateStatement::new()
  //         .name("fk-accounts-journal_id")
  //         .from_tbl(account::Entity)
  //         .from_col(account::Column::JournalId)
  //         .to_tbl(journal::Entity)
  //         .to_col(journal::Column::Id)
  //         .on_delete(ForeignKeyAction::Cascade)
  //         .on_update(ForeignKeyAction::Cascade),
  //     )
  //     .to_owned();
  //   manager.create_table(table).await?;

  //   let index = Index::create()
  //     .name("idx-accounts-journal_id-name")
  //     .table(account::Entity)
  //     .col(account::Column::JournalId)
  //     .col(account::Column::Name)
  //     .unique()
  //     .to_owned();
  //   manager.create_index(index).await?;

  //   let index = Index::create()
  //     .name("idx-accounts-name")
  //     .table(account::Entity)
  //     .col(account::Column::Name)
  //     .to_owned();
  //   manager.create_index(index).await?;

  //   let index = Index::create()
  //     .name("idx-accounts-unit")
  //     .table(account::Entity)
  //     .col(account::Column::Unit)
  //     .to_owned();
  //   manager.create_index(index).await?;

  //   let index = Index::create()
  //     .name("idx-accounts-type")
  //     .table(account::Entity)
  //     .col(account::Column::Typ)
  //     .to_owned();
  //   manager.create_index(index).await?;

  //   Ok(())
  // }

  // async fn create_table_account_tags(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
  //   let table = Table::create()
  //     .table(account_tag::Entity)
  //     .col(ColumnDef::new(account_tag::Column::AccountId).uuid().not_null())
  //     .col(
  //       ColumnDef::new(account_tag::Column::Tag)
  //         .string_len(MAX_SHORT_TEXT_LENGTH as u32)
  //         .not_null(),
  //     )
  //     .primary_key(
  //       Index::create()
  //         .name("pk-account_tags")
  //         .col(account_tag::Column::AccountId)
  //         .col(account_tag::Column::Tag)
  //         .primary(),
  //     )
  //     .foreign_key(
  //       ForeignKeyCreateStatement::new()
  //         .name("fk-account_tags-account_id")
  //         .from_tbl(account_tag::Entity)
  //         .from_col(account_tag::Column::AccountId)
  //         .to_tbl(account::Entity)
  //         .to_col(account::Column::Id)
  //         .on_delete(ForeignKeyAction::Cascade)
  //         .on_update(ForeignKeyAction::Cascade),
  //     )
  //     .to_owned();
  //   manager.create_table(table).await?;

  //   Ok(())
  // }

  // async fn create_table_entries(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
  //   let table = Table::create()
  //     .table(entry::Entity)
  //     .col(ColumnDef::new(entry::Column::Id).uuid().primary_key().not_null())
  //     .col(ColumnDef::new(entry::Column::JournalId).uuid().not_null())
  //     .col(ColumnDef::new(entry::Column::Name).string_len(MAX_NAME_LENGTH as u32).not_null())
  //     .col(
  //       ColumnDef::new(entry::Column::Description)
  //         .string_len(MAX_DESCRIPTION_LENGTH as u32)
  //         .not_null(),
  //     )
  //     .col(ColumnDef::new(entry::Column::Typ).string_len(1).not_null())
  //     .col(ColumnDef::new(entry::Column::Date).date().not_null())
  //     .foreign_key(
  //       ForeignKeyCreateStatement::new()
  //         .name("fk-entries-journal_id")
  //         .from_tbl(entry::Entity)
  //         .from_col(entry::Column::JournalId)
  //         .to_tbl(journal::Entity)
  //         .to_col(journal::Column::Id)
  //         .on_delete(ForeignKeyAction::Cascade)
  //         .on_update(ForeignKeyAction::Cascade),
  //     )
  //     .to_owned();
  //   manager.create_table(table).await?;

  //   let index = Index::create()
  //     .name("idx-entries-journal_id-name")
  //     .table(entry::Entity)
  //     .col(entry::Column::JournalId)
  //     .col(entry::Column::Name)
  //     .unique()
  //     .to_owned();
  //   manager.create_index(index).await?;

  //   let index = Index::create()
  //     .name("idx-entries-name")
  //     .table(entry::Entity)
  //     .col(entry::Column::Name)
  //     .to_owned();
  //   manager.create_index(index).await?;

  //   let index = Index::create()
  //     .name("idx-entries-type")
  //     .table(entry::Entity)
  //     .col(entry::Column::Typ)
  //     .to_owned();
  //   manager.create_index(index).await?;

  //   let index = Index::create()
  //     .name("idx-entries-date")
  //     .table(entry::Entity)
  //     .col(entry::Column::Date)
  //     .to_owned();
  //   manager.create_index(index).await?;

  //   Ok(())
  // }

  // async fn create_table_entry_items(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
  //   let table = Table::create()
  //     .table(entry_item::Entity)
  //     .col(ColumnDef::new(entry_item::Column::EntryId).uuid().not_null())
  //     .col(ColumnDef::new(entry_item::Column::AccountId).uuid().not_null())
  //     .col(ColumnDef::new(entry_item::Column::Amount).decimal().not_null())
  //     .col(ColumnDef::new(entry_item::Column::Price).decimal().not_null())
  //     .primary_key(
  //       Index::create()
  //         .name("pk-entry_items")
  //         .col(entry_item::Column::EntryId)
  //         .col(entry_item::Column::AccountId)
  //         .primary(),
  //     )
  //     .foreign_key(
  //       ForeignKeyCreateStatement::new()
  //         .name("fk-entry_items-entry_id")
  //         .from_tbl(entry_item::Entity)
  //         .from_col(entry_item::Column::EntryId)
  //         .to_tbl(entry::Entity)
  //         .to_col(entry::Column::Id)
  //         .on_delete(ForeignKeyAction::Cascade)
  //         .on_update(ForeignKeyAction::Cascade),
  //     )
  //     .foreign_key(
  //       ForeignKeyCreateStatement::new()
  //         .name("fk-entry_items-account_id")
  //         .from_tbl(entry_item::Entity)
  //         .from_col(entry_item::Column::AccountId)
  //         .to_tbl(account::Entity)
  //         .to_col(account::Column::Id)
  //         .on_delete(ForeignKeyAction::Cascade)
  //         .on_update(ForeignKeyAction::Cascade),
  //     )
  //     .to_owned();
  //   manager.create_table(table).await?;

  //   Ok(())
  // }

  // async fn create_table_entry_tags(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
  //   let table = Table::create()
  //     .table(entry_tag::Entity)
  //     .col(ColumnDef::new(entry_tag::Column::EntryId).uuid().not_null())
  //     .col(
  //       ColumnDef::new(entry_tag::Column::Tag).string_len(MAX_SHORT_TEXT_LENGTH as u32).not_null(),
  //     )
  //     .primary_key(
  //       Index::create()
  //         .name("pk-entry_tags")
  //         .col(entry_tag::Column::EntryId)
  //         .col(entry_tag::Column::Tag)
  //         .primary(),
  //     )
  //     .foreign_key(
  //       ForeignKeyCreateStatement::new()
  //         .name("fk-entry_tags-entry_id")
  //         .from_tbl(entry_tag::Entity)
  //         .from_col(entry_tag::Column::EntryId)
  //         .to_tbl(entry::Entity)
  //         .to_col(entry::Column::Id)
  //         .on_delete(ForeignKeyAction::Cascade)
  //         .on_update(ForeignKeyAction::Cascade),
  //     )
  //     .to_owned();
  //   manager.create_table(table).await?;

  //   Ok(())
  // }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    Migration::create_table_journals(manager).await?;
    Migration::create_table_journal_tags(manager).await?;

    // Migration::create_table_accounts(manager).await?;
    // Migration::create_table_account_tags(manager).await?;

    // Migration::create_table_entries(manager).await?;
    // Migration::create_table_entry_items(manager).await?;
    // Migration::create_table_entry_tags(manager).await?;

    Ok(())
  }
}
