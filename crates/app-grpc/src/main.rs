mod helloworld;

use tonic_reflection::server::Builder;

use tonic::transport::{server::Routes, Server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse()?;

  let reflection = Builder::configure();

  let routes = Routes::default();
  let (reflection, routes) = crate::helloworld::init(reflection, routes);

  let reflection = reflection.build().unwrap();
  Server::builder().add_routes(routes).add_service(reflection).serve(addr).await?;

  Ok(())
}
