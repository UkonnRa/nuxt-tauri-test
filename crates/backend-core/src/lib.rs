pub mod error;
pub mod models;

pub use error::Error;

use tokio::time::{sleep, Duration};

pub async fn create_payload() -> uuid::Uuid {
  sleep(Duration::from_millis(100)).await;
  uuid::Uuid::now_v7()
}

pub type Result<T> = std::result::Result<T, error::Error>;
