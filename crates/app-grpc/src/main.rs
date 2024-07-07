use migration::{Migrator, MigratorTrait};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let db = backend_core::init(".env").await?;
  let db = Arc::new(db);
  Migrator::up(db.as_ref(), None).await?;

  let api_url = env::var("WHITE_RABBIT_API_URL").unwrap();
  app_grpc::serve(db.clone(), api_url).await?;

  Ok(())
}
