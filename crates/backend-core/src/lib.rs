use tokio::time::{sleep, Duration};

pub async fn create_payload() -> uuid::Uuid {
  sleep(Duration::from_millis(100)).await;
  uuid::Uuid::now_v7()
}
