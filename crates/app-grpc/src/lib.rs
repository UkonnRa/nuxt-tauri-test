mod journal;
mod utils;

use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::transport::{server::Routes, Server};
use tonic_reflection::server::Builder;

pub async fn serve(db: Arc<DatabaseConnection>, api_url: impl ToString) -> anyhow::Result<()> {
  log::info!("GRPC Server starts on {}", api_url.to_string());

  let reflection = Builder::configure();
  let routes = Routes::default();
  let (reflection, routes) = crate::journal::init(reflection, routes, db);

  let reflection = reflection.build().unwrap();
  Server::builder()
    .add_routes(routes)
    .add_service(reflection)
    .serve(api_url.to_string().parse()?)
    .await?;

  Ok(())
}
