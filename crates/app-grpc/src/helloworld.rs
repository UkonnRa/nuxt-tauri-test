use std::time::SystemTime;

use backend_core::create_payload;
use chrono::DateTime;
use pb::hello_service_server::{HelloService, HelloServiceServer};
use pb::{HelloRequest, HelloResponse};
use tonic::codec::CompressionEncoding;
use tonic::transport::server::Routes;
use tonic::{Request, Response, Status};
use tonic_reflection::server::Builder;

pub(crate) mod pb {
  tonic::include_proto!("whiterabbit.helloworld.v1");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
    tonic::include_file_descriptor_set!("helloworld_descriptor");
}

#[derive(Debug)]
pub struct HelloServiceImpl {}

#[tonic::async_trait]
impl HelloService for HelloServiceImpl {
  async fn hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
    let body = request.get_ref();
    let payload = create_payload().await;
    let (secs, nsecs) = payload.get_timestamp().unwrap().to_unix();
    let timestamp = DateTime::from_timestamp(secs as i64, nsecs).unwrap();
    Ok(Response::new(HelloResponse {
      message: format!(
        "Hello {}! Response from GRPC, ID = {}, timestamp = {}",
        body.name, payload, timestamp
      ),
      timestamp: Some(SystemTime::from(timestamp).into()),
    }))
  }
}

pub(crate) fn init(reflection_builder: Builder, routes: Routes) -> (Builder, Routes) {
  let service = HelloServiceServer::new(HelloServiceImpl {})
    .send_compressed(CompressionEncoding::Gzip)
    .accept_compressed(CompressionEncoding::Gzip);

  (
    reflection_builder.register_encoded_file_descriptor_set(pb::FILE_DESCRIPTOR_SET),
    routes.add_service(service),
  )
}
