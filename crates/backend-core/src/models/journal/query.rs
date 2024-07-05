use super::database::{journal, journal_tag};
use sea_orm::{
  sea_query::{Cond, Expr, Func, IntoCondition},
  ColumnTrait, Condition, EntityTrait, QueryFilter, QuerySelect, QueryTrait,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Query {
  #[serde(default)]
  pub id: HashSet<Uuid>,
  #[serde(default)]
  pub name: HashSet<String>,
  #[serde(default)]
  pub unit: String,
  #[serde(default)]
  pub tags: HashSet<String>,
  #[serde(default)]
  pub full_text: String,
}

impl IntoCondition for Query {
  fn into_condition(self) -> Condition {
    let mut cond = Cond::all();

    if !self.id.is_empty() {
      cond = cond.add(journal::Column::Id.is_in(self.id));
    }

    let name: HashSet<String> = self
      .name
      .into_iter()
      .map(|name| name.trim().to_string())
      .filter(|name| !name.is_empty())
      .collect();
    if !name.is_empty() {
      cond = cond.add(journal::Column::Name.is_in(name));
    }

    let unit = self.unit.trim().to_string();
    if !unit.is_empty() {
      cond = cond.add(journal::Column::Unit.eq(unit));
    }

    let tags = self
      .tags
      .iter()
      .filter_map(|tag| match tag.trim() {
        "" => None,
        val => Some(val.to_string()),
      })
      .collect::<HashSet<_>>();
    if !tags.is_empty() {
      cond = cond.add(
        journal::Column::Id.in_subquery(
          journal_tag::Entity::find()
            .select_only()
            .distinct()
            .column(journal_tag::Column::JournalId)
            .filter(
              Expr::expr(Expr::col((journal_tag::Entity, journal_tag::Column::Tag))).is_in(tags),
            )
            .into_query(),
        ),
      );
    }

    let keyword = self.full_text.trim().to_lowercase();
    if !keyword.is_empty() {
      let keyword = format!("%{}%", keyword);
      let sub_cond = Cond::any()
        .add(
          Expr::expr(Func::lower(Expr::col((journal::Entity, journal::Column::Name))))
            .like(keyword.clone()),
        )
        .add(
          Expr::expr(Func::lower(Expr::col((journal::Entity, journal::Column::Description))))
            .like(keyword.clone()),
        )
        .add(
          journal::Column::Id.in_subquery(
            journal_tag::Entity::find()
              .select_only()
              .distinct()
              .column(journal_tag::Column::JournalId)
              .filter(
                Expr::expr(Func::lower(Expr::col((journal_tag::Entity, journal_tag::Column::Tag))))
                  .like(keyword.clone()),
              )
              .into_query(),
          ),
        );

      cond = cond.add(sub_cond);
    }

    cond
  }
}

#[cfg(test)]
mod tests {
  use crate::models::journal;
  use sea_orm::{DbBackend, EntityTrait, QueryFilter, QueryTrait};
  use std::collections::HashSet;
  use uuid::uuid;

  #[test]
  fn test_query() -> anyhow::Result<()> {
    let query = journal::Query {
      id: HashSet::from_iter([uuid!("50a1b556-b99d-4ae0-bfba-d117f9a958de")]),
      name: HashSet::from_iter(["Name 1".to_string(), "".to_string(), "  ".to_string()]),
      unit: "Unit 1".to_string(),
      full_text: "Keyword  ".to_string(),
      ..Default::default()
    };

    assert_eq!(
      [r#"SELECT "journals"."id", "journals"."created_date", "journals"."version", "journals"."name", "journals"."description", "journals"."unit" FROM "journals""#,
        r#"WHERE "journals"."id" IN ('50a1b556-b99d-4ae0-bfba-d117f9a958de')"#,
        r#"AND "journals"."name" IN ('Name 1') AND "journals"."unit" = 'Unit 1'"#,
        r#"AND (LOWER("journals"."name") LIKE '%keyword%' OR LOWER("journals"."description") LIKE '%keyword%'"#,
        r#"OR "journals"."id" IN (SELECT DISTINCT "journal_tags"."journal_id" FROM "journal_tags" WHERE LOWER("journal_tags"."tag") LIKE '%keyword%'))"#].join(" "),
      journal::Entity::find().filter(query).build(DbBackend::Sqlite).to_string()
    );

    Ok(())
  }
}
