mod shared;

#[tokio::test]
async fn test_create_journal() -> anyhow::Result<()> {
  let ctx = shared::init().await?;
  test_suite::journal::test_create_journal(ctx).await?;
  Ok(())
}

#[tokio::test]
async fn test_create_journal_conflict_name() -> anyhow::Result<()> {
  let ctx = shared::init().await?;
  test_suite::journal::test_create_journal_conflict_name(ctx).await?;
  Ok(())
}
