mod journal;
mod utils;

use migration::{Migrator, MigratorTrait};
use std::sync::Arc;
use tonic::transport::{server::Routes, Server};
use tonic_reflection::server::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let db = backend_core::init(".env").await?;
  let db = Arc::new(db);
  Migrator::up(db.as_ref(), None).await?;

  let addr = "[::1]:50051".parse()?;
  let reflection = Builder::configure();
  let routes = Routes::default();
  let (reflection, routes) = crate::journal::init(reflection, routes, db);

  let reflection = reflection.build().unwrap();
  Server::builder().add_routes(routes).add_service(reflection).serve(addr).await?;

  Ok(())
}
